# Phase 7 Real Projection Access Report

Status: **read-only access artifacts and typed probes implemented; exact,
lexical, graph, and temporal real-corpus access passed; vector provider was
unavailable in this environment and failed closed.**

Phase 8 activation revalidation, semantic-access inference, conformance,
execution, packet assembly, synthesis, UAT, and `semantic-traversal` remain
out of scope.

## Accepted input tuple

- Phase-6 projection snapshot:
  `projection:phase6:8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`
- Projection logical hash:
  `sha256:ce157f412d51e4ec6c9d34c54e6708862836d9bb5f8b58d6ff212335cb6050ec`
- Corpus snapshot identity:
  `8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`
- Regenerated Phase-6 projection SHA-256:
  `0f60da7b48638f5f494e2691f3be64aaad38b4808bdd53b1a77bad29c000d68a`
- Observation SHA-256:
  `cc179d31f4035f84742312bab363a1504173f034e11625022a2142294c6eb000`

The Phase-6 validator was rerun against the accepted Phase-5 projection and
matching observation. It returned `Validated`, with zero violations and the
accepted census of 1,077 objects, 4,492 regions, 14,108 units, 12,627
identifier assignments, 4,920 occurrences, and 549 temporal anchors.

## Implemented boundary

`src/access.rs` provides:

- versioned `ProjectionAccessArtifacts` and an integrity-bound manifest;
- deterministic exact and full-text lexical indexes;
- graph incidence derived only from containment, parent, occurrence, and
  represented target structure;
- temporal exact, same-precision range, and deterministic ordering access;
- a narrow `EmbeddingProvider` seam with subordinate vector segments;
- typed `ProjectionAccessProbe`, `AccessOperand`, continuation cursors, and
  `ProjectionAccessProbeResult` telemetry;
- fail-closed projection and index identity checks.

Hydration joins only a projected unit's exact parser-observation block span.
The joined block's bytes must satisfy the projected content hash. Observation
records cannot create access identities.

The CLI in `src/bin/phase7_access.rs` builds a private artifact and executes
one concrete probe for every declared real-corpus surface/match-mode pair.

## Access artifacts

The private serialized artifact was written to ignored `target/phase7` only.
It is not committed.

- Access schema: `projection-access-artifacts/v1`
- Artifact identity:
  `sha256:fe36db76ae089b9c2cc4e92f45f32d5535415d90d94d1b933641fb8d516c22a7`
- Serialized artifact SHA-256:
  `b5ef172c571d9087ee41ded5e361007e24c8e22f4cc94eb1256952ec34e68403`
- Exact index identity:
  `sha256:e6140fde825d3ef814ab057eca22db790b0124cce3905f56443f1d34bbe4feed`
- Lexical index identity:
  `sha256:094517d2faa44cc4ed969cbc89d0a519ea365640b404775a3718ccf95bfa9903`
- Graph index identity:
  `sha256:13b43bf71599a79b338707e93f5f1101d13ee50735691e2a9bfc9e1e64f98242`
- Temporal index identity:
  `sha256:495b71903a67ed21992478ef6c123ad787079e8f49a98ae3ea451c4bb32145a9`
- Vector index identity:
  `sha256:474456a9ad2b81f2abf98a597c0c272b62f34857734c34f0421f6889f8f6086d`

| Surface | Indexed records | Result identity |
|---|---:|---|
| exact | 237,307 | semantic unit |
| lexical | 1,238,121 postings | semantic unit |
| graph | 161,662 edges | projected edge target |
| temporal | 549 anchors | temporal anchor |
| vector | 0 segments | unavailable in this run |

The lexical manifest records the deterministic Unicode-alphanumeric-run
tokenizer, Unicode lowercase folding, no stop words, and punctuation as
delimiters. It is a Rust full-text inverted index with an explicit tokenizer
contract; it does not import the Python runtime or silently claim SQLite
FTS5 identity.

## Real probes and telemetry

- Exact literal: one unit returned, total one, not truncated.
- Lexical terms `the`: five returned, total 11,156, truncated with a
  deterministic continuation cursor.
- Graph outgoing from the first projected object: two candidates returned,
  the projected root region and its projected unit.
- Temporal exact probe over the first projected anchor: three same-value
  temporal-anchor candidates returned.
- Vector nearest-neighbour probe: zero candidates, no total, and retryable
  `provider_connect_failed` telemetry because `localhost:11434` refused the
  connection.
- Zero-result exact probes are represented as valid results with total zero.

## Vector provider state

The requested technical contract is recorded without changing the projection:

```text
provider: Ollama
requested model: qwen3-embedding:0.6b
dimension: 1024
dtype: float32
normalization: L2
similarity: cosine
truncation: disabled
```

The native adapter uses Ollama `POST /api/embed`, resolves model identity from
the local model listing, and records the immutable model digest when available.
No model digest or vector output was claimed because the local service was
offline. A later provider-enabled build must regenerate the vector derivative;
it must not mutate the projection snapshot or reuse this unavailable vector
index as if it were ready.

## Validation

- `cargo test` — passed: all repository unit, integration, schema, and doc
  tests; 165 Rust tests passed and doc-tests had zero tests.
- Current-corpus access test with the private accepted inputs — passed; all
  five declared probes executed, returned identities were checked against the
  frozen projection, and input bytes were unchanged.
- Repeated synthetic builds — byte-identical artifacts.
- Synthetic vector-provider build — passed; provider segments remained
  subordinate to canonical unit identities.
- `cargo check --bin phase7_access` — passed.
- `git diff --check` — passed.

## Technical limitations

1. Ollama was unavailable, so this run does not establish provider-backed
   vector embeddings, vector segment counts, model digest, or nearest-neighbor
   candidates.
2. Lexical access is a deterministic Rust full-text inverted index with its
   declared tokenizer contract, not SQLite FTS5. A future SQLite-specific
   requirement would be a separate technical change, not a semantic change.
3. The CLI emits private access artifacts under ignored `target/phase7`; no
   corpus content or private artifact is committed.
