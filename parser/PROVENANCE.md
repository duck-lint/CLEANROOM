# Parser salvage provenance

The retained parser/vault-reading mechanics were salvaged from
`duck-lint/semantic-traversal` commit
`72ef99219fd260ba71365005273f6d9f68cab939`. CLEANROOM adapts those mechanics
to factual parse records; the observer implementation is versioned separately
as `cleanroom-parser-observer/v2`.

Source paths and Git blob identities:

| CLEANROOM path | source path | source blob |
| --- | --- | --- |
| `parser/parser.py` | `src/semantic_traversal/build/parser.py` | `e30b2043a6282cbaa21aa0d5c2d91901ad7c3889` |
| `parser/vault.py` | `src/semantic_traversal/build/vault.py` | `deea2fedf0376f3d112fdfb28c0340d8d30ea09b` |

The copied resolver, materializer, and canonicalization modules were removed
from CLEANROOM. `parser/parser.py` is the sole mechanical source for
frontmatter parsing, Markdown block boundaries, headings, raw source, parsed
text, wikilinks, embeds, and code handling. `parser/observation.py` performs
only JSON transport, filesystem inventory, UUID census, and candidate-address
enumeration over those parser records. It does not resolve or materialize a
corpus, and it has no retrieval or runtime imports.
