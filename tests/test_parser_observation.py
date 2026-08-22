from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest

from parser.observation import observe


class ParserObservationTests(unittest.TestCase):
    def test_observes_factual_source_without_resolving_fenced_links(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "vault"
            output = Path(temporary) / "output"
            root.mkdir()
            (root / "Target.md").write_text(
                "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abc\naliases: [Alias]\nwhen: 2026-08-22\n---\n# Target\n",
                encoding="utf-8",
            )
            (root / "Source.md").write_text(
                "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\nrelated: \"[[Target]]\"\nwhen: 2026-08-22\n---\n# Heading\n\n[[Target]]\n\n```md\n[[Target]]\n```\n",
                encoding="utf-8",
            )

            observation, summary = observe(root, output)
            records = {record["source"]["relative_path"]: record for record in observation["markdown_observations"]}
            source = records["Source.md"]

            self.assertEqual(observation["observation_schema_version"], "vault-observation/v3")
            self.assertEqual(source["uuid"]["parse_status"], "parseable")
            self.assertEqual(source["frontmatter"]["value_shapes"]["related"], "string")
            self.assertEqual(source["frontmatter"]["value_shapes"]["when"], "date")
            self.assertEqual(len(source["authored_links"]), 2)
            self.assertEqual(summary["connections"]["total_occurrence_count"], 2)
            self.assertEqual(
                {link["target_candidates"]["cardinality"] for link in source["authored_links"]},
                {"one_candidate"},
            )

    def test_snapshot_is_stable_for_same_resident_inputs(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "vault"
            root.mkdir()
            (root / "note.md").write_text("---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abc\n---\ntext\n", encoding="utf-8")
            first, _ = observe(root, Path(temporary) / "one")
            second, _ = observe(root, Path(temporary) / "two")

            self.assertEqual(first["vault_resident_snapshot_identity"], second["vault_resident_snapshot_identity"])
            first_without_time = {key: value for key, value in first.items() if key != "generated_at"}
            second_without_time = {key: value for key, value in second.items() if key != "generated_at"}
            self.assertEqual(first_without_time, second_without_time)

    def test_output_must_not_be_inside_vault(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "vault"
            root.mkdir()
            with self.assertRaises(ValueError):
                observe(root, root / ".observation")

    def test_artifacts_are_json_and_written_only_to_output(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "vault"
            output = Path(temporary) / "output"
            root.mkdir()
            (root / "note.md").write_text("---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abc\n---\ntext\n", encoding="utf-8")
            observe(root, output)

            self.assertEqual(sorted(path.name for path in output.iterdir()), ["vault-observation-summary.json", "vault-observation.json"])
            json.loads((output / "vault-observation.json").read_text(encoding="utf-8"))
            self.assertEqual(sorted(path.name for path in root.iterdir()), ["note.md"])


if __name__ == "__main__":
    unittest.main()
