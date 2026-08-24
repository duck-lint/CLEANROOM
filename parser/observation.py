"""Serialize factual records emitted by :mod:`parser.parser`.

This module owns filesystem inventory, JSON-safe transport, UUID census, and
candidate-address enumeration. It does not parse Markdown. All Markdown
blocks, headings, raw source, parsed text, links, embeds, fences, and code are
consumed from the salvaged parser's ``MarkdownParseRecord``.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from datetime import date, datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import subprocess
import uuid as uuidlib
from typing import Any

from .parser import AuthoredLink, MarkdownParseRecord, NoteParseError, parse_markdown


SCHEMA_VERSION = "vault-observation/v3"
OBSERVER_IMPLEMENTATION_VERSION = "cleanroom-parser-observer/v2"
PARSER_SALVAGE_COMMIT = "72ef99219fd260ba71365005273f6d9f68cab939"
PARSER_SALVAGE_BLOBS = {
    "parser/parser.py": {
        "source_path": "src/semantic_traversal/build/parser.py",
        "blob": "e30b2043a6282cbaa21aa0d5c2d91901ad7c3889",
    },
    "parser/vault.py": {
        "source_path": "src/semantic_traversal/build/vault.py",
        "blob": "deea2fedf0376f3d112fdfb28c0340d8d30ea09b",
    },
}
MARKDOWN_EXTENSIONS = {".md"}
APPARATUS_NAMESPACES = {
    ".git": "version_control",
    ".obsidian": "application_state",
    ".semantic-traversal": "generated_runtime_state",
}


def _sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def _sha256_json(value: Any) -> str:
    return _sha256_bytes(json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


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


def _git_commit() -> str | None:
    checkout = Path(__file__).resolve().parent.parent
    try:
        return subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True, stderr=subprocess.DEVNULL).strip()
    except (FileNotFoundError, subprocess.CalledProcessError):
        return None


def _git_worktree_state() -> str:
    checkout = Path(__file__).resolve().parent.parent
    try:
        status = subprocess.check_output(["git", "-C", str(checkout), "status", "--porcelain", "--", "parser"], text=True, stderr=subprocess.DEVNULL)
    except (FileNotFoundError, subprocess.CalledProcessError):
        return "unavailable"
    return "dirty" if status.strip() else "clean"


def _implementation_source_fingerprint() -> str:
    package = Path(__file__).resolve().parent
    files = {}
    for name in ("parser.py", "observation.py", "vault.py", "__main__.py"):
        path = package / name
        files[name] = _sha256_bytes(path.read_bytes())
    return _sha256_json(files)


def _observer_provenance() -> dict[str, Any]:
    commit = _git_commit()
    return {
        "status": "resolved" if commit else "version_only",
        "implementation_version": OBSERVER_IMPLEMENTATION_VERSION,
        "repository": "CLEANROOM",
        "commit": commit,
        "working_tree_state": _git_worktree_state(),
        "implementation_source_fingerprint": _implementation_source_fingerprint(),
        "parser_salvage": {
            "repository": "duck-lint/semantic-traversal",
            "commit": PARSER_SALVAGE_COMMIT,
            "files": PARSER_SALVAGE_BLOBS,
        },
    }


def _apparatus_root(relative: str) -> tuple[str, str] | None:
    first = relative.split("/", 1)[0]
    category = APPARATUS_NAMESPACES.get(first)
    return (first, category) if category else None


def _relative(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


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


def _frontmatter_json(record: MarkdownParseRecord) -> dict[str, Any]:
    frontmatter = record.frontmatter
    return {
        "status": frontmatter.status,
        "raw_text": frontmatter.raw_text,
        "source_span": list(frontmatter.source_span) if frontmatter.source_span is not None else None,
        "body_span": list(frontmatter.body_span),
        "keys": list(frontmatter.keys),
        "values": {key: _json_safe(value) for key, value in frontmatter.values.items()},
        "value_shapes": dict(frontmatter.value_shapes),
        **({"parse_issue": frontmatter.parse_issue} if frontmatter.parse_issue else {}),
    }


def _uuid_observation(record: MarkdownParseRecord) -> dict[str, Any]:
    value = record.uuid_value
    if record.frontmatter.status != "valid" or "uuid" not in record.frontmatter.values:
        return {"field_present": False, "raw_value": None, "value_shape": None, "parse_status": "absent"}
    result: dict[str, Any] = {
        "field_present": True,
        "raw_value": _json_safe(value),
        "value_shape": _shape(value),
        "parse_status": "not_parseable",
        "parsed_version": None,
        "parsed_value": None,
    }
    if isinstance(value, str):
        try:
            parsed = uuidlib.UUID(value)
        except (ValueError, AttributeError):
            pass
        else:
            result.update(parse_status="parseable", parsed_version=parsed.version, parsed_value=str(parsed))
    return result


def _heading_address_key(text: str) -> str:
    normalized = text.casefold().replace(":", "").replace("|", "")
    return " ".join(normalized.split())


def _heading_json(block: Any) -> dict[str, Any]:
    raw_text = block.heading_raw_text or ""
    rendered_text = block.parsed_text
    source_derived_text = block.heading_address_text or rendered_text
    surfaces = []
    for name, text in (("raw", raw_text), ("rendered", rendered_text), ("source_derived", source_derived_text)):
        surface = {"surface": name, "text": text, "address_key": _heading_address_key(text)}
        if not any(existing["address_key"] == surface["address_key"] for existing in surfaces):
            surfaces.append(surface)
    return {
        "level": block.heading_level,
        "raw_text": raw_text,
        "rendered_text": rendered_text,
        "source_derived_text": source_derived_text,
        "address_surfaces": surfaces,
        "address_key": _heading_address_key(rendered_text),
        "source_span": list(block.source_span),
    }


def _block_json(block: Any) -> dict[str, Any]:
    return {
        "block_kind_observation": block.block_kind,
        "parser_token_type": block.parser_token_type,
        "raw_markdown": block.raw_markdown,
        "parsed_text": block.parsed_text,
        "source_span": list(block.source_span),
        "line_start": block.line_start,
        "line_end": block.line_end,
        "explicit_block_ids": list(block.explicit_block_ids),
    }


def _link_json(link: AuthoredLink) -> dict[str, Any]:
    return {
        "source_surface": link.source_surface,
        "frontmatter_key_path": link.frontmatter_key_path,
        "raw_link_markup": link.raw,
        "raw_target": link.target + (("#" + link.target_region_fragment) if link.target_region_fragment else ""),
        "raw_target_without_fragment": link.target,
        "display_alias": link.label if link.display_alias_present else None,
        "display_alias_present": link.display_alias_present,
        "heading_fragment": link.target_region_fragment if link.target_region_fragment and not link.target_region_fragment.startswith("^") else None,
        "block_fragment": link.target_region_fragment[1:] if link.target_region_fragment and link.target_region_fragment.startswith("^") else None,
        "embedded": link.embedded,
        "source_span": list(link.source_span) if link.source_span is not None else None,
        "source_block_span": list(link.source_block_span) if link.source_block_span is not None else None,
        "source_occurrence_ordinal": link.source_occurrence_ordinal,
    }


def _observe_markdown(path: Path, root: Path, file_record: dict[str, Any]) -> dict[str, Any]:
    record = parse_markdown(path, vault_root=root)
    links = [_link_json(link) for link in record.authored_links]
    return {
        "source": file_record,
        "raw_markdown": record.raw_markdown,
        "frontmatter": _frontmatter_json(record),
        "uuid": _uuid_observation(record),
        "headings": [_heading_json(block) for block in record.headings],
        "block_candidates": [_block_json(block) for block in record.blocks],
        "authored_links": links,
        "parse_issues": list(record.parse_issues),
    }


def _complete_link_observations(markdown: list[dict[str, Any]], files: list[dict[str, Any]]) -> None:
    """Enumerate address candidates from parser-emitted links only."""

    address_surfaces: dict[str, dict[str, set[str]]] = defaultdict(lambda: defaultdict(set))

    def add(surface: str, key: str, source: str) -> None:
        address_surfaces[surface][key].add(source)

    for file_record in (item for item in files if item["observation_category"] == "vault_resident"):
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
                "candidate_evidence": [{"source_path": source, "surfaces": sorted(surface for surface, sources in evidence.items() if source in sources)} for source in candidates],
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
                    link.update(heading_target_evaluation="observed", heading_target_match_kind="normalized", heading_target_matches=matches)
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
            except UnicodeDecodeError:
                decoding = "decode_failed"
            else:
                decoding = "utf8"
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
    snapshot = _sha256_json({"directories": resident_dirs, "files": [{key: item[key] for key in ("relative_path", "source_kind", "extension", "byte_size", "source_byte_hash", "text_decoding_status")} for item in resident_files]})
    provenance = _observer_provenance()
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
            "Markdown structure, source spans, parser-native frontmatter shapes, links, embeds, and code forms are emitted by the salvaged factual parser.",
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
