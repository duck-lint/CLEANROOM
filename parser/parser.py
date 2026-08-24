"""The CLEANROOM factual Markdown/frontmatter parser.

The Markdown mechanics in this file are salvaged from semantic-traversal
build commit ``72ef99219fd260ba71365005273f6d9f68cab939``. The records below
retain authored facts only; they do not admit identifiers, resolve
destinations, materialize context, or create canonical identities.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import date, datetime
from pathlib import Path, PurePosixPath
import re
from typing import Any, Iterable

from markdown_it import MarkdownIt
from mdit_py_plugins.gfm import gfm_plugin
from ruamel.yaml import YAML
from ruamel.yaml.constructor import DuplicateKeyError


class NoteParseError(ValueError):
    """The source cannot be represented by the factual parse contract."""


@dataclass(frozen=True)
class ObservationConfig:
    """Only file-enumeration settings belong to this parser boundary."""

    uuid_field: str = "uuid"
    excluded_folders: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        if not isinstance(self.uuid_field, str) or not self.uuid_field.strip():
            raise NoteParseError("uuid_field must be a non-empty string")
        for excluded_folder in self.excluded_folders:
            _validate_excluded_folder(excluded_folder)


BuildConfig = ObservationConfig


def _validate_excluded_folder(value: str) -> None:
    if not isinstance(value, str) or not value.strip():
        raise NoteParseError(f"excluded_folders entry is not a vault-relative directory path: {value!r}")
    normalized = value.replace("\\", "/")
    raw_parts = tuple(normalized.split("/"))
    path = PurePosixPath(normalized)
    if path.is_absolute() or ".." in raw_parts or (path.parts and ":" in path.parts[0]):
        raise NoteParseError(f"excluded_folders entry is not a vault-relative directory path: {value!r}")


@dataclass(frozen=True)
class AuthoredLink:
    """One parsed authored wikilink or embed, without target resolution."""

    raw: str
    target: str
    label: str
    target_region_fragment: str | None
    embedded: bool
    display_alias_present: bool
    source_surface: str
    frontmatter_key_path: str | None
    source_span: tuple[int, int] | None
    # The parser-owned block is the authoritative body-occurrence source.
    # The finer inline span remains optional evidence, never the source-unit
    # authority. Frontmatter links have no block span.
    source_block_span: tuple[int, int] | None
    # One-based order within the owning body block or frontmatter field.
    source_occurrence_ordinal: int


Wikilink = AuthoredLink
Embed = AuthoredLink


@dataclass(frozen=True)
class FrontmatterRecord:
    status: str
    raw_text: str | None
    source_span: tuple[int, int] | None
    body_span: tuple[int, int]
    keys: tuple[str, ...]
    values: dict[str, Any]
    value_shapes: dict[str, str]
    content_span: tuple[int, int] | None
    parse_issue: str | None = None


@dataclass(frozen=True)
class MarkdownBlock:
    """One root Markdown block emitted by the CommonMark parser."""

    block_kind: str
    parser_token_type: str
    raw_markdown: str
    source_span: tuple[int, int]
    line_start: int
    line_end: int
    parsed_text: str
    authored_links: tuple[AuthoredLink, ...]
    embeds: tuple[AuthoredLink, ...]
    explicit_block_ids: tuple[str, ...]
    heading_level: int | None
    heading_raw_text: str | None
    heading_address_text: str | None


@dataclass(frozen=True)
class MarkdownParseRecord:
    authored_path: str
    raw_markdown: str
    frontmatter: FrontmatterRecord
    uuid_value: Any
    headings: tuple[MarkdownBlock, ...]
    blocks: tuple[MarkdownBlock, ...]
    authored_links: tuple[AuthoredLink, ...]
    parse_issues: tuple[str, ...]


_LINK = re.compile(r"(?P<embed>!)?\[\[(?P<body>[^\]]+)\]\]")
_CALLOUT = re.compile(r"^\[!([A-Za-z0-9_-]+)\](?:[ \t]+|$)")


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


def _line_spans(source: str) -> list[tuple[int, int, str]]:
    lines: list[tuple[int, int, str]] = []
    offset = 0
    for line in source.splitlines(keepends=True):
        end = offset + len(line)
        lines.append((offset, end, line))
        offset = end
    if not lines or offset < len(source):
        lines.append((offset, len(source), source[offset:]))
    return lines


def _frontmatter(source: str) -> FrontmatterRecord:
    lines = _line_spans(source)
    if not lines or lines[0][2].strip("\r\n") != "---":
        return FrontmatterRecord("absent", None, None, (0, len(source)), (), {}, {}, None)
    closing = next(
        (index for index, (_, _, line) in enumerate(lines[1:], 1) if line.strip("\r\n") in {"---", "..."}),
        None,
    )
    if closing is None:
        return FrontmatterRecord("unterminated", source[lines[0][1] :], (lines[0][0], len(source)), (len(source), len(source)), (), {}, {}, (lines[0][1], len(source)), "missing_closing_delimiter")
    raw = source[lines[0][1] : lines[closing][0]]
    yaml = YAML(typ="safe", pure=True)
    yaml.version = (1, 2)
    try:
        parsed = yaml.load(raw) or {}
    except DuplicateKeyError:
        parsed = None
        issue = "yaml_duplicate_key"
    except Exception as exc:  # Concrete parser exceptions are outside this boundary.
        parsed = None
        issue = f"yaml_parse_failed:{type(exc).__name__}"
    if parsed is None:
        return FrontmatterRecord("malformed", raw, (lines[0][0], lines[closing][1]), (lines[closing][1], len(source)), (), {}, {}, (lines[0][1], lines[closing][0]), issue)
    if not isinstance(parsed, dict):
        return FrontmatterRecord("non_mapping", raw, (lines[0][0], lines[closing][1]), (lines[closing][1], len(source)), (), {}, {}, (lines[0][1], lines[closing][0]), "frontmatter_not_mapping")
    values = {str(key): value for key, value in parsed.items()}
    shapes = {key: _shape(value) for key, value in values.items()}
    return FrontmatterRecord("valid", raw, (lines[0][0], lines[closing][1]), (lines[closing][1], len(source)), tuple(sorted(values)), values, {key: shapes[key] for key in sorted(shapes)}, (lines[0][1], lines[closing][0]))


def _wikilink_rule(state: Any, silent: bool) -> bool:
    """Parse Obsidian links as inline Markdown tokens.

    Code spans are consumed by CommonMark before this rule. Fenced and
    indented code never enter this inline rule.
    """

    match = _LINK.match(state.src, state.pos)
    if match is not None and match.group("embed"):
        if silent:
            return True
        token = state.push("embed", "", 0)
        target, fragment, label, alias_present = _link_parts(match.group("body"))
        token.content = label
        token.attrs = {"target": target, "target_region_fragment": fragment, "label": label, "display_alias_present": alias_present, "source_position": (state.pos, match.end()), "raw": match.group(0)}
        state.pos = match.end()
        return True
    if match is None or (state.pos > 0 and state.src[state.pos - 1] == "\\"):
        return False
    if silent:
        return True
    token = state.push("wikilink", "", 0)
    target, fragment, label, alias_present = _link_parts(match.group("body"))
    token.content = label
    token.attrs = {"target": target, "label": label, "target_region_fragment": fragment, "display_alias_present": alias_present, "source_position": (state.pos, match.end()), "raw": match.group(0)}
    state.pos = match.end()
    return True


def _link_parts(body: str) -> tuple[str, str | None, str, bool]:
    separator = re.search(r"\\?\|", body)
    target_with_fragment = body[: separator.start()] if separator else body
    label = body[separator.end() :] if separator else None
    if "#" in target_with_fragment:
        target, fragment = target_with_fragment.split("#", 1)
    else:
        target, fragment = target_with_fragment, None
    return target, fragment, label or target, separator is not None


def _markdown_parser() -> MarkdownIt:
    parser = MarkdownIt("commonmark")
    gfm_plugin(parser)
    parser.inline.ruler.before("text", "cleanroom_wikilink", _wikilink_rule)
    return parser


def _path_data(authored_path: str) -> tuple[str, ...]:
    path = PurePosixPath(authored_path.replace("\\", "/"))
    if path.is_absolute() or ".." in path.parts:
        raise NoteParseError("authored path must be a relative, vault-scoped path")
    return path.parts


def _block_kind(token_type: str) -> str:
    return {
        "heading_open": "heading",
        "fence": "code_fence",
        "code_block": "indented_code",
        "bullet_list_open": "list",
        "ordered_list_open": "list",
        "table_open": "table",
        "blockquote_open": "blockquote_or_callout",
        "paragraph_open": "paragraph",
        "callout_open": "blockquote_or_callout",
        "alert_open": "blockquote_or_callout",
    }.get(token_type, token_type)


def _root_block_spans(tokens: list[Any], body_lines: list[str]) -> list[tuple[int, int, Any]]:
    """Return root CommonMark blocks; nesting, not blank lines, defines spans."""

    spans: list[tuple[int, int, Any]] = []
    depth = 0
    current: tuple[int, Any] | None = None
    current_end = 0
    for token in tokens:
        token_map = token.map
        if depth == 0 and token_map is not None and token.type != "heading_close":
            if current is not None:
                spans.append((current[0], current_end, current[1]))
            current = (token_map[0], token)
            current_end = token_map[1]
        if current is not None and token_map is not None:
            current_end = max(current_end, token_map[1])
        depth += token.nesting
        if depth < 0:
            raise NoteParseError("Markdown parser produced invalid block nesting")
    if current is not None:
        spans.append((current[0], current_end, current[1]))
    return [(start, end, token) for start, end, token in spans if end > start and end <= len(body_lines)]


def _link_from_attrs(
    attrs: dict[str, Any],
    *,
    embedded: bool,
    surface: str,
    key_path: str | None,
    source_span: tuple[int, int] | None,
    source_block_span: tuple[int, int] | None,
    source_occurrence_ordinal: int,
) -> AuthoredLink:
    return AuthoredLink(
        attrs["raw"],
        attrs["target"],
        attrs["label"],
        attrs["target_region_fragment"],
        embedded,
        attrs["display_alias_present"],
        surface,
        key_path,
        source_span,
        source_block_span,
        source_occurrence_ordinal,
    )


def _unique_occurrence(source: str, value: str) -> int | None:
    """Return a position only when the authored representation is unique."""

    if not value:
        return None
    positions: list[int] = []
    cursor = 0
    while True:
        position = source.find(value, cursor)
        if position < 0:
            break
        positions.append(position)
        cursor = position + 1
    return positions[0] if len(positions) == 1 else None


def _inline_source_start(raw: str, token: Any) -> int | None:
    """Map one inline token to raw block source only if its slice is unique."""

    if token.map is None or not token.content:
        return None
    lines = raw.splitlines(keepends=True)
    line_start = sum(len(line) for line in lines[: token.map[0]])
    line_end = sum(len(line) for line in lines[: token.map[1]])
    relative = _unique_occurrence(raw[line_start:line_end], token.content)
    return None if relative is None else line_start + relative


def _parsed_text_and_links(
    parser: MarkdownIt,
    raw: str,
    *,
    source_offset: int | None,
    source_text: str | None = None,
    source_surface: str = "body",
    frontmatter_key_path: str | None = None,
    source_block_span: tuple[int, int] | None = None,
    source_occurrence_ordinal_start: int = 1,
    callout: bool = False,
) -> tuple[str, tuple[AuthoredLink, ...], tuple[AuthoredLink, ...], int]:
    tokens = parser.parse(raw)
    text_parts: list[str] = []
    links: list[AuthoredLink] = []
    embeds: list[AuthoredLink] = []
    occurrence_ordinal = source_occurrence_ordinal_start
    for token in tokens:
        if token.type == "inline" and token.children:
            inline_parts: list[str] = []
            for child in token.children:
                if child.type in {"wikilink", "embed"}:
                    attrs = child.attrs or {}
                    inline_start = _inline_source_start(raw, token)
                    source_position = attrs.get("source_position")
                    local_start = None if inline_start is None or source_position is None else inline_start + source_position[0]
                    raw_link = attrs["raw"]
                    exact = local_start is not None and raw[local_start : local_start + len(raw_link)] == raw_link
                    absolute_start = None if source_offset is None or local_start is None else source_offset + local_start
                    absolute_end = None if absolute_start is None else absolute_start + len(raw_link)
                    absolute_exact = source_text is None or (
                        absolute_start is not None
                        and absolute_end is not None
                        and 0 <= absolute_start <= absolute_end <= len(source_text)
                        and source_text[absolute_start:absolute_end] == raw_link
                    )
                    span = None if not exact or not absolute_exact or absolute_start is None or absolute_end is None else (absolute_start, absolute_end)
                    link = _link_from_attrs(
                        attrs,
                        embedded=child.type == "embed",
                        surface=source_surface,
                        key_path=frontmatter_key_path,
                        source_span=span,
                        source_block_span=source_block_span,
                        source_occurrence_ordinal=occurrence_ordinal,
                    )
                    occurrence_ordinal += 1
                    (embeds if child.type == "embed" else links).append(link)
                    if child.type == "wikilink":
                        inline_parts.append(child.content)
                elif child.type in {"text", "code_inline", "softbreak", "hardbreak"}:
                    inline_parts.append(child.content if child.type != "softbreak" else "\n")
            if inline_parts:
                text_parts.append("".join(inline_parts))
        elif token.type in {"code_block", "fence"}:
            text_parts.append(token.content)
    parsed_text = "\n".join(part for part in text_parts if part)
    if callout:
        parsed_text = _CALLOUT.sub("", parsed_text, count=1)
    return parsed_text, tuple(links), tuple(embeds), occurrence_ordinal


def _heading_address_text(parser: MarkdownIt, raw: str) -> str:
    tokens = parser.parse(raw)
    parts: list[str] = []
    for token in tokens:
        if token.type != "inline" or not token.children:
            continue
        inline: list[str] = []
        for child in token.children:
            if child.type == "wikilink":
                attrs = child.attrs or {}
                inline.append(attrs["target"])
                if child.content != attrs["target"]:
                    inline.append(" " + child.content)
            elif child.type in {"text", "code_inline", "softbreak", "hardbreak"}:
                inline.append(child.content if child.type != "softbreak" else "\n")
        if inline:
            parts.append("".join(inline))
    return "\n".join(parts)


def _heading_raw_text(raw: str) -> str:
    """Return the authored heading text after its CommonMark marks."""

    line = raw.splitlines()[0] if raw.splitlines() else ""
    match = re.match(r"^[ \t]{0,3}#{1,6}[ \t]+(.*?)[ \t]*$", line)
    return match.group(1) if match else line


def _explicit_block_ids(raw: str) -> tuple[str, ...]:
    return tuple(re.findall(r"\^([A-Za-z0-9_-]+)\s*$", raw, flags=re.MULTILINE))


def _frontmatter_links(frontmatter: FrontmatterRecord, *, source_text: str) -> tuple[AuthoredLink, ...]:
    if frontmatter.status != "valid" or frontmatter.raw_text is None or frontmatter.content_span is None:
        return ()
    parser = _markdown_parser()
    links: list[AuthoredLink] = []
    for key, value in frontmatter.values.items():
        values: Iterable[Any] = (value,) if isinstance(value, str) else value if isinstance(value, list) else ()
        field_occurrence_ordinal = 1
        for item in values:
            if not isinstance(item, str):
                continue
            item_offset = _unique_occurrence(frontmatter.raw_text, item)
            source_offset = None if item_offset is None else frontmatter.content_span[0] + item_offset
            _, item_links, item_embeds, field_occurrence_ordinal = _parsed_text_and_links(
                parser,
                item,
                source_offset=source_offset,
                source_text=source_text,
                source_surface="frontmatter",
                frontmatter_key_path=str(key),
                source_occurrence_ordinal_start=field_occurrence_ordinal,
            )
            links.extend(item_links)
            links.extend(item_embeds)
    return tuple(links)


def parse_markdown_text(source: str, *, authored_path: str) -> MarkdownParseRecord:
    """Parse one Markdown source into factual records only."""

    _path_data(authored_path)
    normalized_path = PurePosixPath(authored_path.replace("\\", "/")).as_posix()
    frontmatter = _frontmatter(source)
    body = source[frontmatter.body_span[0] : frontmatter.body_span[1]]
    body_lines = body.splitlines(keepends=True)
    parser = _markdown_parser()
    tokens = parser.parse(body)
    for index, token in enumerate(tokens):
        if token.type != "blockquote_open":
            continue
        inline = next((candidate for candidate in tokens[index + 1 :] if candidate.type == "inline"), None)
        match = _CALLOUT.match(inline.content) if inline is not None else None
        if match:
            token.type = "callout_open"
            token.attrs = {"kind": match.group(1)}

    blocks: list[MarkdownBlock] = []
    for start, end, token in _root_block_spans(tokens, body_lines):
        raw = "".join(body_lines[start:end])
        source_start = frontmatter.body_span[0] + sum(len(line) for line in body_lines[:start])
        source_end = frontmatter.body_span[0] + sum(len(line) for line in body_lines[:end])
        parsed_text, links, embeds, _ = _parsed_text_and_links(
            parser,
            raw,
            source_offset=source_start,
            source_text=source,
            source_block_span=(source_start, source_end),
            callout=token.type == "callout_open",
        )
        level = int(token.tag[1:]) if token.type == "heading_open" else None
        blocks.append(MarkdownBlock(_block_kind(token.type), token.type, raw, (source_start, source_end), start + 1, end, parsed_text, links, embeds, _explicit_block_ids(raw), level, _heading_raw_text(raw) if level is not None else None, _heading_address_text(parser, raw) if level is not None else None))

    frontmatter_links = _frontmatter_links(frontmatter, source_text=source)
    all_links = frontmatter_links + tuple(link for block in blocks for link in (*block.authored_links, *block.embeds))
    issues = (frontmatter.parse_issue,) if frontmatter.parse_issue else ()
    uuid_value = frontmatter.values.get("uuid") if frontmatter.status == "valid" else None
    return MarkdownParseRecord(normalized_path, source, frontmatter, uuid_value, tuple(block for block in blocks if block.heading_level is not None), tuple(blocks), all_links, issues)


def parse_markdown(path: str | Path, *, vault_root: str | Path) -> MarkdownParseRecord:
    source_path = Path(path)
    root = Path(vault_root)
    try:
        authored_path = source_path.relative_to(root).as_posix()
    except ValueError as exc:
        raise NoteParseError("note path must be beneath vault_root") from exc
    return parse_markdown_text(source_path.read_text(encoding="utf-8"), authored_path=authored_path)


parse_note = parse_markdown
