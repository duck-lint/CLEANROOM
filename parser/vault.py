"""Deterministic raw Markdown discovery and factual parse collection."""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from ruamel.yaml.error import YAMLError

from .parser import MarkdownParseRecord, ObservationConfig, NoteParseError, parse_markdown


@dataclass(frozen=True)
class CorpusFailure:
    """An independently detected candidate failure, before repair serialization."""

    kind: str
    message: str
    source_paths: tuple[str, ...]


@dataclass(frozen=True)
class VaultParseResult:
    """Parsed Markdown records plus independently observed parse failures."""

    records: tuple[MarkdownParseRecord, ...]
    failures: tuple[CorpusFailure, ...]
    config: ObservationConfig

    @property
    def is_valid(self) -> bool:
        """Whether every discovered Markdown source parsed mechanically."""

        return not self.failures


def _relative_path(path: Path, vault_root: Path) -> str:
    return path.relative_to(vault_root).as_posix()


def _excluded(relative_path: PurePosixPath, config: ObservationConfig) -> bool:
    """Apply exact path-prefix exclusion, not leaf-name or glob matching."""

    for excluded_folder in config.excluded_folders:
        excluded = PurePosixPath(excluded_folder.replace("\\", "/"))
        if relative_path.parts[: len(excluded.parts)] == excluded.parts:
            return True
    return False


def discover_markdown_notes(vault_root: str | Path, config: ObservationConfig) -> tuple[Path, ...]:
    """Discover included Markdown notes in deterministic vault-relative order."""

    root = Path(vault_root)
    if not root.is_dir():
        raise NoteParseError(f"vault root is not a directory: {root}")

    discovered: list[tuple[str, Path]] = []
    for path in root.rglob("*"):
        if not path.is_file() or path.suffix != ".md":
            continue
        relative = PurePosixPath(_relative_path(path, root))
        if not _excluded(relative, config):
            discovered.append((relative.as_posix(), path))
    discovered.sort(key=lambda item: item[0])
    return tuple(path for _, path in discovered)


def parse_vault(vault_root: str | Path, config: ObservationConfig = ObservationConfig()) -> VaultParseResult:
    """Parse all included Markdown sources without admission or resolution."""

    root = Path(vault_root)
    records: list[MarkdownParseRecord] = []
    failures: list[CorpusFailure] = []
    for path in discover_markdown_notes(root, config):
        source_path = (_relative_path(path, root),)
        try:
            records.append(parse_markdown(path, vault_root=root))
        except (NoteParseError, OSError, UnicodeError, YAMLError) as exc:
            failures.append(CorpusFailure("parse", str(exc), source_path))

    return VaultParseResult(tuple(records), tuple(failures), config)
