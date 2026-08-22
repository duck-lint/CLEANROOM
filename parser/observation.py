"""Bounded factual observation of a raw authored vault.

This module deliberately stops at observation.  It does not assign semantic
object or unit identities, resolve links, materialize context, canonicalize
records, or expose retrieval/runtime behavior.

The Markdown and YAML mechanics were salvaged from semantic-traversal build
commit ``72ef99219fd260ba71365005273f6d9f68cab939``.  The output is a
CLEANROOM-owned ``vault-observation/v3`` artifact.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from datetime import date, datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import uuid as uuidlib
from typing import Any

from ruamel.yaml import YAML

from .parser import BuildConfig, NoteParseError, _frontmatter, parse_note


SCHEMA_VERSION = "vault-observation/v3"
SOURCE_COMMIT = "72ef99219fd260ba71365005273f6d9f68cab939"
MARKDOWN_EXTENSIONS = {".md"}
APPARATUS_NAMESPACES = {
    ".git": "version_control",
    ".obsidian": "application_state",
    ".semantic-traversal": "generated_runtime_state",
}
HEADING_RE = re.compile(r"^(?P<marks>#{1,6})[ \t]+(?P<text>.*?)[ \t]*$")
FENCE_RE = re.compile(r"^[ \t]*(?P<marker>`{3,}|~{3,})(?P<info>.*)$")
LIST_RE = re.compile(r"^[ \t]*(?:[-+*]|\d+[.)])[ \t]+")
TABLE_SEPARATOR_RE = re.compile(
    r"^[ \t]*\|?[ \t]*:?-{3,}:?[ \t]*(?:\|[ \t]*:?-{3,}:?[ \t]*)+\|?[ \t]*$"
)
INLINE_WIKILINK_RE = re.compile(r"(?P<embed>!)?\[\[(?P<body>[^\]]+)\]\]")


def _sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def _sha256_json(value: Any) -> str:
    return _sha256_bytes(
        json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    )


def _shape(value: Any) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, datetime):
        return "datetime"
    if isinstance(value, date):
        return "date"
    if isinstance(value, str):
        return "string"
    if isinstance(value, (int, float)):
        return "number"
    if isinstance(value, list):
        return "array"
    if isinstance(value, dict):
        return "mapping"
    return type(value).__name__


def _json_safe(value: Any) -> Any:
    if value is None or isinstance(value, (str, int, float, bool)):
        return value
    if isinstance(value, (datetime, date)):
        return value.isoformat()
    if isinstance(value, list):
        return [_json_safe(item) for item in value]
    if isinstance(value, dict):
        return {str(key): _json_safe(item) for key, item in value.items()}
    return str(value)


def _line_spans(text: str) -> list[tuple[int, int, str]]:
    result: list[tuple[int, int, str]] = []
    offset = 0
    for line in text.splitlines(keepends=True):
        end = offset + len(line)
        result.append((offset, end, line))
        offset = end
    if not result or offset < len(text):
        result.append((offset, len(text), text[offset:]))
    return result


def _frontmatter_observation(text: str) -> dict[str, Any]:
    lines = _line_spans(text)
    if not lines or lines[0][2].strip("\r\n") != "---":
        return {
            "status": "absent",
            "raw_text": None,
            "source_span": None,
            "body_span": [0, len(text)],
            "keys": [],
            "values": {},
            "value_shapes": {},
        }
    closing = next(
        (index for index, (_, _, line) in enumerate(lines[1:], 1) if line.strip("\r\n") in {"---", "..."}),
        None,
    )
    if closing is None:
        raw = text[lines[0][1] :]
        return {
            "status": "unterminated",
            "raw_text": raw,
            "source_span": [lines[0][0], len(text)],
            "body_span": [len(text), len(text)],
            "keys": [],
            "values": {},
            "value_shapes": {},
            "parse_issue": "missing_closing_delimiter",
        }
    raw = text[lines[0][1] : lines[closing][0]]
    yaml = YAML(typ="safe", pure=True)
    yaml.version = (1, 2)
    try:
        parsed = yaml.load(raw) or {}
    except Exception as exc:  # Concrete parser exceptions are outside this boundary.
        return {
            "status": "malformed",
            "raw_text": raw,
            "source_span": [lines[0][0], lines[closing][1]],
            "body_span": [lines[closing][1], len(text)],
            "keys": [],
            "values": {},
            "value_shapes": {},
            "parse_issue": f"yaml_parse_failed:{type(exc).__name__}",
        }
    if not isinstance(parsed, dict):
        return {
            "status": "non_mapping",
            "raw_text": raw,
            "source_span": [lines[0][0], lines[closing][1]],
            "body_span": [lines[closing][1], len(text)],
            "keys": [],
            "values": {},
            "value_shapes": {},
            "parse_issue": "frontmatter_not_mapping",
        }
    shapes = {str(key): _shape(value) for key, value in parsed.items()}
    values = {str(key): _json_safe(value) for key, value in parsed.items()}
    return {
        "status": "valid",
        "raw_text": raw,
        "source_span": [lines[0][0], lines[closing][1]],
        "body_span": [lines[closing][1], len(text)],
        "keys": sorted(values),
        "values": values,
        "value_shapes": {key: shapes[key] for key in sorted(shapes)},
    }


def _uuid_observation(frontmatter: dict[str, Any]) -> dict[str, Any]:
    values = frontmatter.get("values", {})
    if "uuid" not in values:
        return {"field_present": False, "raw_value": None, "value_shape": None, "parse_status": "absent"}
    raw = values["uuid"]
    result: dict[str, Any] = {
        "field_present": True,
        "raw_value": raw,
        "value_shape": _shape(raw),
        "parse_status": "not_parseable",
        "parsed_version": None,
        "parsed_value": None,
    }
    if isinstance(raw, str):
        try:
            parsed = uuidlib.UUID(raw)
        except (ValueError, AttributeError):
            pass
        else:
            result.update(parse_status="parseable", parsed_version=parsed.version, parsed_value=str(parsed))
    return result


def _parse_link(match: re.Match[str], surface: str, key_path: str | None) -> dict[str, Any]:
    body = match.group("body")
    separator = re.search(r"\\?\|", body)
    if separator:
        target_with_fragments = body[: separator.start()]
        display = body[separator.end() :]
    else:
        target_with_fragments = body
        display = None
    base = target_with_fragments
    heading = None
    block = None
    if "#" in base:
        base, fragment = base.split("#", 1)
        if fragment.startswith("^"):
            block = fragment[1:]
        else:
            heading = fragment
    elif "^" in base:
        base, block = base.split("^", 1)
    return {
        "source_surface": surface,
        "frontmatter_key_path": key_path,
        "raw_link_markup": match.group(0),
        "raw_target": target_with_fragments,
        "raw_target_without_fragment": base,
        "display_alias": display,
        "heading_fragment": heading,
        "block_fragment": block,
        "embedded": bool(match.group("embed")),
        "source_span": [match.start(), match.end()],
    }


def _render_inline_wikilinks(text: str) -> str:
    def replace(match: re.Match[str]) -> str:
        body = match.group("body")
        separator = re.search(r"\\?\|", body)
        if separator:
            return body[separator.end() :]
        return body.split("#", 1)[0] if "#" in body else body

    return INLINE_WIKILINK_RE.sub(replace, text)


def _source_derived_inline_wikilinks(text: str) -> str:
    def replace(match: re.Match[str]) -> str:
        body = match.group("body")
        separator = re.search(r"\\?\|", body)
        if not separator:
            return body.split("#", 1)[0] if "#" in body else body
        target = body[: separator.start()].split("#", 1)[0]
        display = body[separator.end() :]
        return " ".join(part for part in (target, display) if part)

    return INLINE_WIKILINK_RE.sub(replace, text)


def _heading_address_key(text: str) -> str:
    rendered = _render_inline_wikilinks(text).replace("\\|", "|").casefold()
    rendered = rendered.replace(":", "").replace("|", "")
    rendered = re.sub(r"\(\s+", "(", rendered)
    return " ".join(rendered.split())


def _heading_observation(raw_text: str, level: int, source_span: list[int]) -> dict[str, Any]:
    rendered_text = _render_inline_wikilinks(raw_text)
    source_derived_text = _source_derived_inline_wikilinks(raw_text)
    surfaces = []
    for surface_name, surface_text in (
        ("raw", raw_text),
        ("rendered", rendered_text),
        ("source_derived", source_derived_text),
    ):
        surface = {"surface": surface_name, "text": surface_text, "address_key": _heading_address_key(surface_text)}
        if not any(existing["address_key"] == surface["address_key"] for existing in surfaces):
            surfaces.append(surface)
    return {
        "level": level,
        "raw_text": raw_text,
        "rendered_text": rendered_text,
        "source_derived_text": source_derived_text,
        "address_surfaces": surfaces,
        "address_key": _heading_address_key(rendered_text),
        "source_span": source_span,
    }


def _frontmatter_key_for_offset(raw_text: str, offset: int) -> str | None:
    current: str | None = None
    running = 0
    for line in raw_text.splitlines(keepends=True):
        match = re.match(r"^([A-Za-z0-9_-]+)\s*:", line)
        if match:
            current = match.group(1)
        if running <= offset < running + len(line):
            return current
        running += len(line)
    return current


def _blocks(text: str, body_start: int) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    lines = _line_spans(text)
    headings = [
        _heading_observation(match.group("text"), len(match.group("marks")), [start, end])
        for start, end, line in lines
        if start >= body_start and (match := HEADING_RE.match(line.rstrip("\r\n")))
    ]
    candidates: list[dict[str, Any]] = []
    index = 0
    while index < len(lines):
        start, end, line = lines[index]
        if end <= body_start or not line.strip():
            index += 1
            continue
        raw = line.rstrip("\r\n")
        fence = FENCE_RE.match(raw)
        if fence:
            marker = fence.group("marker")
            last = index
            while last + 1 < len(lines):
                last += 1
                if re.match(r"^[ \t]*" + re.escape(marker[0]) + r"{" + str(len(marker)) + r",}[ \t]*$", lines[last][2].rstrip("\r\n")):
                    break
            kind = "code_fence"
        else:
            last = index
            kind = (
                "heading"
                if HEADING_RE.match(raw)
                else "list"
                if LIST_RE.match(raw)
                else "table"
                if "|" in raw and index + 1 < len(lines) and TABLE_SEPARATOR_RE.match(lines[index + 1][2].rstrip("\r\n"))
                else "blockquote_or_callout"
                if raw.lstrip().startswith(">")
                else "paragraph"
            )
            if kind == "paragraph":
                while last + 1 < len(lines) and lines[last + 1][2].strip() and not HEADING_RE.match(lines[last + 1][2].rstrip("\r\n")) and not FENCE_RE.match(lines[last + 1][2].rstrip("\r\n")):
                    last += 1
        block_end = lines[last][1]
        raw_block = text[start:block_end]
        candidates.append({
            "block_kind_observation": kind,
            "raw_markdown": raw_block,
            "source_span": [start, block_end],
            "line_start": index + 1,
            "line_end": last + 1,
            "explicit_block_ids": re.findall(r"\^([A-Za-z0-9_-]+)\s*$", raw_block, flags=re.MULTILINE),
        })
        index = last + 1
    return headings, candidates


def _relative(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


def _apparatus_root(relative: str) -> tuple[str, str] | None:
    first = relative.split("/", 1)[0]
    category = APPARATUS_NAMESPACES.get(first)
    return (first, category) if category else None


def _directories(vault_root: Path) -> list[dict[str, Any]]:
    result = []
    for path in sorted((item for item in vault_root.rglob("*") if item.is_dir() and not item.is_symlink()), key=lambda item: _relative(item, vault_root)):
        relative = _relative(path, vault_root)
        parent = PurePosixPath(relative).parent.as_posix()
        result.append({
            "relative_path": relative,
            "parent_relative_path": "" if parent == "." else parent,
            "basename": path.name,
            "direct_child_directory_count": sum(child.is_dir() for child in path.iterdir()),
            "direct_child_file_count": sum(child.is_file() for child in path.iterdir()),
            "observation_category": "technical_apparatus" if _apparatus_root(relative) else "vault_resident",
        })
    return result


def _frontmatter_links(frontmatter: dict[str, Any]) -> list[tuple[re.Match[str], str | None]]:
    values = frontmatter.get("values", {})
    result: list[tuple[re.Match[str], str | None]] = []
    for key, value in values.items():
        authored_values = [value] if isinstance(value, str) else value if isinstance(value, list) else []
        for item in authored_values:
            if not isinstance(item, str):
                continue
            result.extend((match, str(key)) for match in INLINE_WIKILINK_RE.finditer(item))
    return result


def _observe_markdown(path: Path, root: Path, file_record: dict[str, Any]) -> dict[str, Any]:
    decoded = path.read_text(encoding="utf-8")
    frontmatter = _frontmatter_observation(decoded)
    body_start = frontmatter["body_span"][0]
    headings, blocks = _blocks(decoded, body_start)
    fence_spans = [block["source_span"] for block in blocks if block["block_kind_observation"] == "code_fence"]
    links: list[dict[str, Any]] = []
    for match in INLINE_WIKILINK_RE.finditer(decoded):
        if any(start <= match.start() < end for start, end in fence_spans):
            continue
        if frontmatter["source_span"] and match.start() < frontmatter["source_span"][1]:
            offset = match.start() - frontmatter["source_span"][0] - len("---\n")
            surface, key_path = "frontmatter", _frontmatter_key_for_offset(frontmatter["raw_text"] or "", offset)
        else:
            surface, key_path = "body", None
        links.append(_parse_link(match, surface, key_path))
    # Parsing is retained as a mechanical parser gate; its result is not
    # promoted into semantic objects, units, relations, or canonical records.
    try:
        parse_note(path, vault_root=root, build_config=BuildConfig("vault", "uuid", (), ()), require_uuid=False)
    except (NoteParseError, OSError, UnicodeError) as exc:
        parse_issues = [f"parser:{type(exc).__name__}:{exc}"]
    else:
        parse_issues = []
    return {
        "source": file_record,
        "raw_markdown": decoded,
        "frontmatter": frontmatter,
        "uuid": _uuid_observation(frontmatter),
        "headings": headings,
        "block_candidates": blocks,
        "authored_links": links,
        "parse_issues": ([frontmatter["parse_issue"]] if frontmatter.get("parse_issue") else []) + parse_issues,
    }


def _complete_link_observations(markdown: list[dict[str, Any]], files: list[dict[str, Any]]) -> None:
    address_surfaces: dict[str, dict[str, set[str]]] = defaultdict(lambda: defaultdict(set))

    def add(surface: str, key: str, source: str) -> None:
        address_surfaces[surface][key].add(source)

    resident_files = [item for item in files if item["observation_category"] == "vault_resident"]
    for file_record in resident_files:
        path = PurePosixPath(file_record["relative_path"])
        source = file_record["relative_path"]
        add("exact_relative_path", source, source)
        if file_record["source_kind"] == "markdown":
            add("exact_relative_path_without_extension", source.removesuffix(path.suffix), source)
            add("basename_stem", path.stem, source)
        else:
            add("resident_basename", path.name, source)
    for record in markdown:
        aliases = record["frontmatter"].get("values", {}).get("aliases", [])
        aliases = [aliases] if isinstance(aliases, str) else aliases if isinstance(aliases, list) else []
        for alias in aliases:
            if isinstance(alias, str):
                add("authored_alias", alias, record["source"]["relative_path"])
    for surface, lookup in list(address_surfaces.items()):
        folded = address_surfaces[f"{surface}_casefold"]
        for key, sources in lookup.items():
            folded[key.casefold()].update(sources)

    by_source = {record["source"]["relative_path"]: record for record in markdown}
    for record in markdown:
        for link in record["authored_links"]:
            target = link["raw_target_without_fragment"].strip()
            evidence: dict[str, set[str]] = {}
            for surface, lookup in address_surfaces.items():
                lookup_target = target.casefold() if surface.endswith("_casefold") else target
                matches = set(lookup.get(lookup_target, set()))
                if lookup_target.endswith(".md") and surface in {"exact_relative_path_without_extension", "exact_relative_path_without_extension_casefold"}:
                    matches.update(lookup.get(lookup_target[:-3], set()))
                if matches:
                    evidence[surface] = matches
            candidates = sorted({source for sources in evidence.values() for source in sources})
            link["target_candidates"] = {
                "cardinality": "zero_candidates" if not candidates else "one_candidate" if len(candidates) == 1 else "multiple_candidates",
                "candidate_source_paths": candidates,
                "candidate_evidence": [
                    {"source_path": source, "surfaces": sorted(surface for surface, sources in evidence.items() if source in sources)}
                    for source in candidates
                ],
            }
            target_record = by_source.get(candidates[0]) if len(candidates) == 1 else None
            if link["heading_fragment"] is None:
                link.update(heading_target_evaluation="not_applicable", heading_target_match_kind="not_applicable", heading_target_matches=[])
            elif target_record is None:
                link.update(heading_target_evaluation="not_evaluable_parent_unresolved", heading_target_match_kind="not_evaluable_parent_unresolved", heading_target_matches=[])
            else:
                fragment_key = _heading_address_key(link["heading_fragment"])
                matches = [heading for heading in target_record["headings"] if any(surface["address_key"] == fragment_key for surface in heading["address_surfaces"])]
                if len(matches) == 1:
                    heading = matches[0]
                    link.update(heading_target_evaluation="observed", heading_target_match_kind="normalized", heading_target_matches=[heading])
                elif matches:
                    link.update(heading_target_evaluation="ambiguous", heading_target_match_kind="ambiguous", heading_target_matches=matches)
                else:
                    link.update(heading_target_evaluation="absent", heading_target_match_kind="absent", heading_target_matches=[])
            block_ids = {block_id for block in target_record["block_candidates"] for block_id in block["explicit_block_ids"]} if target_record else set()
            link["block_target_evaluation"] = "not_applicable" if link["block_fragment"] is None else "observed" if link["block_fragment"] in block_ids else "not_evaluable_parent_unresolved" if target_record is None else "absent"


def observe(vault_root: str | Path, output_root: str | Path) -> tuple[dict[str, Any], dict[str, Any]]:
    root = Path(vault_root).resolve()
    destination = Path(output_root).resolve()
    if not root.is_dir():
        raise ValueError(f"vault root is not a directory: {root}")
    if destination == root or root in destination.parents:
        raise ValueError("output-root must not be inside vault-root")

    directories = _directories(root)
    files: list[dict[str, Any]] = []
    markdown: list[dict[str, Any]] = []
    apparatus_stats: dict[str, dict[str, Any]] = {}
    paths = sorted((item for item in root.rglob("*") if item.is_file() and not item.is_symlink()), key=lambda item: _relative(item, root))
    for path in paths:
        relative = _relative(path, root)
        data = path.read_bytes()
        apparatus = _apparatus_root(relative)
        category = "technical_apparatus" if apparatus else "vault_resident"
        kind = "markdown" if path.suffix.lower() in MARKDOWN_EXTENSIONS else "non_markdown"
        decoding = "not_attempted"
        decoded = None
        if kind == "markdown":
            try:
                decoded = data.decode("utf-8")
                decoding = "utf8"
            except UnicodeDecodeError:
                decoding = "decode_failed"
        file_record = {
            "relative_path": relative,
            "parent_relative_path": "" if path.parent == root else path.parent.relative_to(root).as_posix(),
            "basename": path.name,
            "extension": path.suffix.lower(),
            "source_kind": kind,
            "byte_size": len(data),
            "source_byte_hash": _sha256_bytes(data),
            "text_decoding_status": decoding,
            "observation_status": "observed",
            "observation_category": category,
        }
        files.append(file_record)
        if apparatus:
            root_name, apparatus_category = apparatus
            stat = apparatus_stats.setdefault(root_name, {"relative_root_path": root_name, "apparatus_category": apparatus_category, "entry_count": 0, "file_count": 0, "directory_count": 0, "total_byte_count": 0})
            stat["file_count"] += 1
            stat["total_byte_count"] += len(data)
        elif decoded is not None:
            markdown.append(_observe_markdown(path, root, file_record))
    for item in directories:
        apparatus = _apparatus_root(item["relative_path"])
        if apparatus:
            root_name, apparatus_category = apparatus
            stat = apparatus_stats.setdefault(root_name, {"relative_root_path": root_name, "apparatus_category": apparatus_category, "entry_count": 0, "file_count": 0, "directory_count": 0, "total_byte_count": 0})
            stat["directory_count"] += 1
    for stat in apparatus_stats.values():
        stat["entry_count"] = stat["file_count"] + stat["directory_count"]
    _complete_link_observations(markdown, files)

    resident_dirs = [item for item in directories if item["observation_category"] == "vault_resident"]
    resident_files = [item for item in files if item["observation_category"] == "vault_resident"]
    snapshot = _sha256_json({
        "directories": resident_dirs,
        "files": [{key: item[key] for key in ("relative_path", "source_kind", "extension", "byte_size", "source_byte_hash", "text_decoding_status")} for item in resident_files],
    })
    provenance = {"status": "resolved", "commit": SOURCE_COMMIT}
    observation = {
        "observation_schema_version": SCHEMA_VERSION,
        "observer_provenance": provenance,
        "generated_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "vault_resident_snapshot_identity": snapshot,
        "directory_observations": resident_dirs,
        "file_observations": resident_files,
        "technical_apparatus_observations": sorted(apparatus_stats.values(), key=lambda item: item["relative_root_path"]),
        "markdown_observations": markdown,
        "measurement_limitations": [
            "Only the listed Markdown heading, block, frontmatter, wikilink, embed, and fragment forms are mechanically interpreted.",
            "Unsupported Markdown is retained as raw source and is not interpreted as semantic structure.",
            "Candidate paths are address observations, not canonical identities.",
        ],
    }
    uuid_groups: dict[str, list[str]] = defaultdict(list)
    for record in markdown:
        if record["uuid"].get("parsed_value"):
            uuid_groups[record["uuid"]["parsed_value"]].append(record["source"]["relative_path"])
    for record in markdown:
        occurrences = sorted(uuid_groups.get(record["uuid"].get("parsed_value"), []))
        record["uuid"]["occurrence_source_paths"] = occurrences
        record["uuid"]["duplicate_source_paths"] = occurrences if len(occurrences) > 1 else []

    fm_counts = Counter(record["frontmatter"]["status"] for record in markdown)
    uuid_counts = Counter(record["uuid"]["parse_status"] for record in markdown)
    link_counts = Counter(link["source_surface"] for record in markdown for link in record["authored_links"])
    candidate_counts = Counter(link["target_candidates"]["cardinality"] for record in markdown for link in record["authored_links"])
    summary = {
        "observation_schema_version": SCHEMA_VERSION,
        "observer_provenance": provenance,
        "vault_resident_snapshot_identity": snapshot,
        "technical_apparatus": observation["technical_apparatus_observations"],
        "vault_resident_topology": {"directory_count": len(resident_dirs), "file_count": len(resident_files), "top_level_entries": dict(Counter(PurePosixPath(item["relative_path"]).parts[0] for item in resident_dirs + resident_files))},
        "file_kind_counts": dict(Counter(item["source_kind"] for item in resident_files)),
        "extension_counts": dict(Counter(item["extension"] or "[none]" for item in resident_files)),
        "markdown_source_count": len(markdown),
        "frontmatter_status_counts": dict(fm_counts),
        "frontmatter_key_census": dict(Counter(key for record in markdown for key in record["frontmatter"].get("keys", []))),
        "frontmatter_value_shape_census": dict(Counter(shape for record in markdown for shape in record["frontmatter"].get("value_shapes", {}).values())),
        "uuid_status_counts": dict(uuid_counts),
        "uuid_version_counts": dict(Counter(str(record["uuid"]["parsed_version"]) for record in markdown if record["uuid"].get("parsed_version") is not None)),
        "duplicate_uuid_group_count": sum(len(paths) > 1 for paths in uuid_groups.values()),
        "heading_level_counts": dict(Counter(str(heading["level"]) for record in markdown for heading in record["headings"])),
        "authored_block_kind_counts": dict(Counter(block["block_kind_observation"] for record in markdown for block in record["block_candidates"])),
        "connections": {"total_occurrence_count": sum(len(record["authored_links"]) for record in markdown), "frontmatter_occurrence_count": link_counts["frontmatter"], "body_occurrence_count": link_counts["body"], "embed_count": sum(link["embedded"] for record in markdown for link in record["authored_links"]), "display_alias_count": sum(link["display_alias"] is not None for record in markdown for link in record["authored_links"]), "heading_fragment_count": sum(link["heading_fragment"] is not None for record in markdown for link in record["authored_links"]), "block_fragment_count": sum(link["block_fragment"] is not None for record in markdown for link in record["authored_links"]), "candidate_cardinality_counts": dict(candidate_counts)},
        "measurement_limitations": observation["measurement_limitations"],
    }
    destination.mkdir(parents=True, exist_ok=True)
    (destination / "vault-observation.json").write_text(json.dumps(observation, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    (destination / "vault-observation-summary.json").write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return observation, summary


def main(argv: list[str] | None = None) -> int:
    import argparse

    argument_parser = argparse.ArgumentParser(description="Produce a CLEANROOM factual vault-observation/v3 artifact.")
    argument_parser.add_argument("--vault-root", required=True, type=Path)
    argument_parser.add_argument("--output-root", required=True, type=Path)
    args = argument_parser.parse_args(argv)
    observation, summary = observe(args.vault_root, args.output_root)
    print(json.dumps({"observation_schema_version": observation["observation_schema_version"], "snapshot": observation["vault_resident_snapshot_identity"], "markdown_source_count": summary["markdown_source_count"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
