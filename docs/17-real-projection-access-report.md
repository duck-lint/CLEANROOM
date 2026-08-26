# Phase 7 Real Projection Access Report

Status: **provider-backed read-only access artifacts and typed probes passed
against the accepted current-corpus projection; all five real-corpus surfaces
returned projection-bound results.**

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

The provider-backed serialized artifact was retained in ignored
`target/phase7` only. It is not committed.

- Access schema: `projection-access-artifacts/v1`
- Artifact identity:
  `sha256:d8ce6fd667df120d4ed6d55300a22f181131f944833570f28de795900d6c26fc`
- Serialized artifact SHA-256:
  `25caede3004a7620f35cbf7ed7c02388e8cb8bd5baff70f1be455e50880088e2`
- Exact index identity:
  `sha256:e6140fde825d3ef814ab057eca22db790b0124cce3905f56443f1d34bbe4feed`
- Lexical index identity:
  `sha256:094517d2faa44cc4ed969cbc89d0a519ea365640b404775a3718ccf95bfa9903`
- Graph index identity:
  `sha256:13b43bf71599a79b338707e93f5f1101d13ee50735691e2a9bfc9e1e64f98242`
- Temporal index identity:
  `sha256:495b71903a67ed21992478ef6c123ad787079e8f49a98ae3ea451c4bb32145a9`
- Vector index identity:
  `sha256:e2d9cbd2159798ecab3def854be6f5fd6729b11fb1b74799002e2f04c5191bb2`

| Surface | Indexed records | Result identity |
|---|---:|---|
| exact | 237,307 | semantic unit |
| lexical | 1,238,121 postings | semantic unit |
| graph | 161,662 edges | projected edge target |
| temporal | 549 anchors | temporal anchor |
| vector | 14,111 segments | semantic unit (technical segments subordinate) |

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
- Vector nearest-neighbour probe using the first provider-produced corpus
  embedding as a nonzero mechanical operand: five candidates returned, total
  14,108, truncated with a deterministic continuation cursor. Returned
  identities were semantic units and were checked against the frozen
  projection.
- The former zero-vector probe is an invalid cosine operand, not a valid
  zero-result search. It fails closed with `zero vector operand is not
  searchable`; zero-result behavior remains covered by the exact surface.
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
The provider-backed artifact resolved:

```text
endpoint: http://127.0.0.1:11434
resolved model: qwen3-embedding:0.6b
model digest: ac6da0dfba84a81fdbfbaf330198c33cd77c4cdfc53e8bc50eb581914a15621d
max input chars: provider-reported value unavailable; capacity is observed by
  explicit provider rejection and activates the recorded transport policy
```

The vector index contains nonzero corpus-derived segments. Segment identities
remain subordinate to their canonical parent units; they do not become
semantic-unit identities.

## Validation

- `cargo test --test access` — passed: 10 tests, including the provider-backed
  artifact validation, projection/hash fail-closed binding, all five probes,
  and zero-vector rejection.
- Provider-backed artifact validation — passed; the existing provider-backed
  current-corpus artifact deserialized, self-validated, matched the accepted
  projection snapshot/logical hash/corpus identity, and all returned
  identities were checked against the frozen projection.
- Repeated synthetic builds — byte-identical artifacts.
- Synthetic vector-provider build — passed; provider segments remained
  subordinate to canonical unit identities.
- Repeated provider-backed artifact validation — passed against the same
  deterministic artifact identity and serialized SHA-256.
- `cargo check --bin phase7_access` — passed.
- `git diff --check` — passed.

## Technical limitations

1. The private observation source is not currently exposed as a working-tree
   path, so this validation revalidated the surviving provider-backed artifact
   from the accepted Phase-7 build rather than publishing or reconstructing
   private corpus input. The artifact itself passed exact projection binding
   and self-integrity validation.
2. Lexical access is a deterministic Rust full-text inverted index with its
   declared tokenizer contract, not SQLite FTS5. A future SQLite-specific
   requirement would be a separate technical change, not a semantic change.
3. The CLI emits private access artifacts under ignored `target/phase7`; no
   corpus content or private artifact is committed.
