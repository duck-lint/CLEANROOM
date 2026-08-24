from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest

from parser.observation import observe
from parser.parser import parse_markdown_text


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

    def test_salvaged_parser_owns_all_markdown_structure_and_link_syntax(self) -> None:
        source = (
            "---\n"
            "uuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\n"
            "when: 2026-08-22\n"
            "related: \"[[Target#Heading|display]]\"\n"
            "aliases:\n"
            "  - Alias\n"
            "  - \"[[Target]]\"\n"
            "embed: \"![[Target#^block|embed]]\"\n"
            "---\n"
            "# [[Target|Heading]]\n\n"
            "A [[Target#Heading|display]] and ![[Target#^block|embed]].\n\n"
            "`[[Target]]` \\[[Target]]\n\n"
            "    [[Target]]\n\n"
            "~~~md\n[[Target]]\n~~~\n\n"
            "- list [[Target]]\n  - nested [[Target]]\n\n"
            "| left | right |\n| --- | --- |\n| [[Target]] | value |\n\n"
            "body ^block\n"
        )
        record = parse_markdown_text(source, authored_path="Source.md")

        self.assertEqual(record.frontmatter.value_shapes["when"], "date")
        self.assertEqual(record.frontmatter.values["aliases"][0], "Alias")
        self.assertEqual(
            [block.block_kind for block in record.blocks],
            ["heading", "paragraph", "paragraph", "indented_code", "code_fence", "list", "table", "paragraph"],
        )
        self.assertEqual(record.blocks[0].heading_raw_text, "[[Target|Heading]]")
        self.assertEqual(record.blocks[1].parsed_text, "A display and .")
        self.assertEqual(record.blocks[4].authored_links, ())
        self.assertEqual(record.blocks[3].authored_links, ())
        self.assertEqual(record.blocks[5].parsed_text.count("Target"), 2)
        self.assertEqual(record.blocks[6].parsed_text.count("Target"), 1)
        self.assertEqual(len(record.authored_links), 9)
        self.assertEqual(
            [(link.target, link.target_region_fragment, link.embedded, link.source_surface) for link in record.authored_links],
            [
                ("Target", "Heading", False, "frontmatter"),
                ("Target", None, False, "frontmatter"),
                ("Target", "^block", True, "frontmatter"),
                ("Target", None, False, "body"),
                ("Target", "Heading", False, "body"),
                ("Target", "^block", True, "body"),
                ("Target", None, False, "body"),
                ("Target", None, False, "body"),
                ("Target", None, False, "body"),
            ],
        )

    def test_candidate_step_consumes_parser_links_and_fragments(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "vault"
            output = Path(temporary) / "output"
            root.mkdir()
            (root / "Target.md").write_text(
                "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abc\n---\n## Heading\n\nblock ^block\n",
                encoding="utf-8",
            )
            (root / "Source.md").write_text(
                "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\nrelated: \"[[Target#Heading|display]]\"\n---\n[[Target#Heading|display]]\n![[Target#^block|embed]]\n",
                encoding="utf-8",
            )

            observation, _ = observe(root, output)
            source = next(item for item in observation["markdown_observations"] if item["source"]["relative_path"] == "Source.md")
            links = source["authored_links"]

            self.assertEqual(len(links), 3)
            self.assertTrue(all(link["target_candidates"]["cardinality"] == "one_candidate" for link in links))
            heading_link = next(link for link in links if link["heading_fragment"] == "Heading")
            block_link = next(link for link in links if link["block_fragment"] == "block")
            self.assertEqual(heading_link["heading_target_evaluation"], "observed")
            self.assertEqual(block_link["block_target_evaluation"], "observed")

    def test_link_spans_are_exact_or_explicitly_unavailable(self) -> None:
        source = (
            "---\n"
            "uuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\n"
            "one: \"[[Target]]\"\n"
            "two: \"[[Target]]\"\n"
            "related:\n"
            "  - \"[[Target]]\"\n"
            "  - \"[[Target]]\"\n"
            "aliased: \"[[Target|Target]]\"\n"
            "---\n"
            "A [[Target]] and [[Target]] and [[Target|Target]].\n"
            "`[[Target]]` [[Target]]\n\n"
            "| [[Target]] | [[Target]] |\n"
            "| --- | --- |\n"
        )
        record = parse_markdown_text(source, authored_path="Source.md")
        links = record.authored_links

        repeated_frontmatter = [link for link in links if link.source_surface == "frontmatter" and link.raw == "[[Target]]" and link.frontmatter_key_path in {"one", "two"}]
        self.assertEqual(len(repeated_frontmatter), 2)
        self.assertEqual([link.source_span for link in repeated_frontmatter], [None, None])
        self.assertEqual([link.source_block_span for link in repeated_frontmatter], [None, None])
        self.assertEqual([link.source_occurrence_ordinal for link in repeated_frontmatter], [1, 1])
        repeated_field = [link for link in links if link.source_surface == "frontmatter" and link.frontmatter_key_path == "related"]
        self.assertEqual([link.source_occurrence_ordinal for link in repeated_field], [1, 2])
        aliased_frontmatter = next(link for link in links if link.source_surface == "frontmatter" and link.raw == "[[Target|Target]]")
        self.assertIsNotNone(aliased_frontmatter.source_span)
        self.assertTrue(aliased_frontmatter.display_alias_present)
        self.assertEqual(aliased_frontmatter.label, "Target")
        self.assertEqual(aliased_frontmatter.source_occurrence_ordinal, 1)

        body_links = [link for link in links if link.source_surface == "body"]
        self.assertEqual([link.display_alias_present for link in body_links[:3]], [False, False, True])
        self.assertEqual([link.source_occurrence_ordinal for link in body_links[:3]], [1, 2, 3])
        self.assertTrue(all(link.source_block_span == record.blocks[0].source_span for link in body_links[:3]))
        for link in body_links[:3]:
            self.assertIsNotNone(link.source_span)
            start, end = link.source_span
            self.assertEqual(source[start:end], link.raw)
        inline_code_and_real_link = [link for link in body_links if link.raw == "[[Target]]"][2]
        self.assertIsNotNone(inline_code_and_real_link.source_span)
        start, end = inline_code_and_real_link.source_span
        self.assertEqual(source[start:end], inline_code_and_real_link.raw)
        table_links = [link for link in body_links if link.source_span and source[link.source_span[0] : link.source_span[1]] == link.raw]
        self.assertLess(len(table_links), len(body_links))

    def test_body_occurrence_ordinals_share_the_parser_block_and_survive_unavailable_spans(self) -> None:
        record = parse_markdown_text(
            "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\n---\n"
            "| [[Target]] | [[Target]] |\n| --- | --- |\n",
            authored_path="Table.md",
        )

        links = record.authored_links
        self.assertEqual(len(links), 2)
        self.assertEqual([link.source_occurrence_ordinal for link in links], [1, 2])
        self.assertEqual([link.source_block_span for link in links], [record.blocks[0].source_span] * 2)
        self.assertEqual([link.source_span for link in links], [None, None])

    def test_callout_uses_authored_block_category_and_retains_parser_token(self) -> None:
        record = parse_markdown_text(
            "---\nuuid: 018f3e7e-7b6a-7c3b-8d0e-123456789abd\n---\n> [!note]\n> callout body\n",
            authored_path="Callout.md",
        )

        self.assertEqual(len(record.blocks), 1)
        self.assertEqual(record.blocks[0].block_kind, "blockquote_or_callout")
        self.assertIn(record.blocks[0].parser_token_type, {"callout_open", "alert_open"})

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
