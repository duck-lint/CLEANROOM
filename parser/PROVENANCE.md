# Parser salvage provenance

The retained parser/vault-reading mechanics were copied byte-for-byte from
`duck-lint/semantic-traversal` commit
`72ef99219fd260ba71365005273f6d9f68cab939`.

Source paths and Git blob identities:

| CLEANROOM path | source path | source blob |
| --- | --- | --- |
| `parser/parser.py` | `src/semantic_traversal/build/parser.py` | `e30b2043a6282cbaa21aa0d5c2d91901ad7c3889` |
| `parser/vault.py` | `src/semantic_traversal/build/vault.py` | `deea2fedf0376f3d112fdfb28c0340d8d30ea09b` |

The copied resolver, materializer, and canonicalization modules were removed
from CLEANROOM.  The observer emits raw resident-file, frontmatter,
Markdown-block, heading, link-syntax, and candidate-address observations only.
It does not resolve or materialize a corpus, and it has no retrieval or
runtime imports.
