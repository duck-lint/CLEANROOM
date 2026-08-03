# Kernel Equations

## Purpose

These equations describe the runtime without assuming any existing classes, modules, or implementation architecture.

The system contains two semantic roles:

1. **inference**, performed through two fresh calls:
   - boundary inference;
   - semantic-access inference;
2. **synthesis**, which interprets what semantic access returns.

Everything between semantic-access inference and synthesis is structural conformance, deterministic execution, measured coverage, and provenance-preserving packet construction.

## Terms

Let:

- \(u_t\) be the newest user utterance at turn \(t\);
- \(v_{t-1}\) be the immediately preceding completed conversational turn, supplied only for local continuity;
- \(P_{t-1}\) be the thread-local problem-space state before the newest utterance;
- \(B_t\) be the boundary contribution produced from the newest utterance;
- \(P_t\) be the updated problem space;
- \(\Lambda_t\) be the current attention lens over \(P_t\);
- \(M_\sigma\) be the immutable projected semantic-space snapshot used for the turn;
- \(W_t\) be the bounded, positive, expandable working projection activated through \(P_t\) and \(\Lambda_t\);
- \(T_t\) be the inferred semantic-access plan;
- \(C\) be structural conformance;
- \(R_t\) be the retrieval result;
- \(L_t\) be measured execution limits and coverage facts;
- \(A_t\) be the synthesized answer.

## 1. Boundary inference

\[
B_t = D(P_{t-1}, u_t, v_{t-1})
\]

The first inference call interprets how the newest utterance perturbs the existing problem space.

The boundary contribution may declare operations such as:

- preserve;
- reinforce;
- extend;
- merge;
- split;
- connect;
- constrain;
- open a tension;
- resolve a tension;
- redirect attention;
- supersede;
- retire.

The result is not an answer and is not a retrieval plan.

It is a turn-local description of how the currently bounded problem gestalt should change.

For conceptual emphasis:

\[
B_t \equiv \Delta_t
\]

where \(\Delta_t\) is a semantic perturbation of the prior problem space.

## 2. Problem-space structure

The current problem space may be represented abstractly as:

\[
P_t =
(
\mathcal{G}_t,
\mathcal{E}_t,
\mathcal{C}_t,
\mathcal{O}_t,
\mathcal{H}_t,
\Lambda_t
)
\]

Where:

- \(\mathcal{G}_t\) contains currently individuated problem regions or gestalts;
- \(\mathcal{E}_t\) contains represented relations among those regions;
- \(\mathcal{C}_t\) contains active constraints;
- \(\mathcal{O}_t\) contains open tensions, unresolved references, contradictions, and missing distinctions;
- \(\mathcal{H}_t\) contains contribution and persistence history;
- \(\Lambda_t\) contains the current attentional activation of the problem space.

This tuple is a conceptual contract, not a demand for one literal storage format.

## 3. Problem-space evolution

\[
P_t = U(P_{t-1}, B_t)
\]

The runtime deterministically applies the declared boundary operations.

The update function may validate and apply an inference-issued transformation, but it may not independently infer that two regions are semantically equivalent.

A fresh thread begins with an empty or explicitly initialized \(P_0\).

## 4. Problem-space coherence

A problem region is coherent insofar as it preserves an identifiable relational structure while incorporating new boundary contributions.

Coherence does not require stasis.

A coherent update may:

- refine the region;
- revise one of its relations;
- redirect its temporal orientation;
- incorporate a correction;
- merge duplicated formulations;
- expose an unresolved contradiction.

Coherence is initially qualitative and structural.

It is not:

- a scalar confidence score;
- an automatic decay function;
- a truth measure;
- an evidence-admission threshold;
- a post-retrieval filter.

## 5. Attention lens

The attention lens \(\Lambda_t\) exposes activation strata over the same relational problem space:

```text
primary activation
secondary activation
tertiary activation
background activation
```

These are not four separate stores.

They are different current intensities of access to one bounded problem gestalt.

A region may move among activation bands without being duplicated.

## 6. Problem-space-shaped projection activation

\[
W_t^{(0)}
=
A_{\mathrm{cfg}}(M_\sigma, P_t, u_t, \Lambda_t)
\]

The runtime deterministically activates a bounded working view of the frozen semantic projection using:

- the relational problem-space structure;
- the current attention lens;
- the newest utterance;
- configured retrieval-surface budgets.

Activation is positive-only.

It controls what is presently visible, not what is relevant, nonexistent, or evidentially absent.

The working view may be expanded during the second inference call:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma, W_t^{(k)}, q_k, \beta)
\]

where \(q_k\) is a typed expansion request and \(\beta\) is the configured hard budget.

## 7. Semantic-access inference

\[
T_t = I_2(P_t, u_t, W_t)
\]

A fresh second inference call explores the activated semantic space, resolves canonical addresses, and emits the final semantic-access plan.

The plan connects:

- problem regions;
- represented relations;
- active constraints;
- open tensions;
- the current attention lens;

to identifiers, semantic objects, semantic units, regions, occurrences, anchors, and retrieval surfaces represented in \(M_\sigma\).

The plan is a proposed collection route through an existing semantic space.

It is not a new ontology and does not resolve the problem-space tension itself.

## 8. Structural conformance

\[
C(T_t, M_\sigma)
\in
\{\text{valid},\text{invalid}\}
\]

Equivalent shorthand:

\[
T_t \subseteq M_\sigma
\]

Every identifier, address, surface, relation, direction, and transition used by the plan must be represented as a valid possibility or canonical instance in \(M_\sigma\).

Conformance performs no semantic interpretation.

If invalid, the failure may be returned for one bounded repair:

\[
T'_t
=
I_{\mathrm{repair}}
(
P_t,
u_t,
W_t,
T_t,
\operatorname{violations}(T_t,M_\sigma)
)
\]

The repair call receives exact structural violations, not a deterministic reinterpretation of the user's meaning.

## 9. Execution

For a conforming plan:

\[
R_t = E(T_t, M_\sigma)
\]

Execution materializes the semantic units addressed by the plan.

The retrieval result preserves:

- canonical object identity;
- canonical semantic-unit identity;
- object/unit containment and belonging;
- inherited identifiers;
- occurrence identity;
- link target and fragment identity;
- temporal anchors;
- retrieval-surface provenance;
- path provenance;
- execution status;
- deterministic bounds.

Execution does not decide whether a returned unit is coherent with a runtime-generated paraphrase.

## 10. Coverage and execution limits

\[
L_t = Q(T_t, R_t)
\]

Coverage is derived from what actually executed.

Coverage constrains claim scope.

It does not reinterpret the meaning of retrieved units and does not measure problem-space coherence.

## 11. Synthesis

\[
A_t
=
S(P_t,u_t,v_{t-1},T_t,R_t,L_t)
\]

The synthesis model receives:

- \(P_t\): the relational problem-space gestalt as background context;
- \(u_t\): the newest utterance as the focus;
- \(v_{t-1}\): the immediately preceding turn as local conversational continuity;
- \(T_t\): what was sought and through which semantic connections;
- \(R_t\): the returned semantic units and provenance;
- \(L_t\): measured execution limits and claim constraints.

The preceding turn is contextual continuity, not retrieval evidence.

Synthesis determines what the retrieved semantic units imply for the current problem.

## Complete compression

\[
\begin{aligned}
B_t &= D(P_{t-1},u_t,v_{t-1}) \\
P_t &= U(P_{t-1},B_t) \\
W_t^{(0)} &= A_{\mathrm{cfg}}(M_\sigma,P_t,u_t,\Lambda_t) \\
W_t^{(k+1)} &= \operatorname{expand}(M_\sigma,W_t^{(k)},q_k,\beta) \\
T_t &= I_2(P_t,u_t,W_t) \\
C(T_t,M_\sigma) &\rightarrow \text{valid or bounded repair} \\
R_t &= E(T_t,M_\sigma) \\
L_t &= Q(T_t,R_t) \\
A_t &= S(P_t,u_t,v_{t-1},T_t,R_t,L_t)
\end{aligned}
\]

## Architectural reading

- \(P_t\) is a thread-local relational problem gestalt.
- \(B_t\) describes how the newest utterance perturbs that gestalt.
- \(\Lambda_t\) describes current activation, not separate topic storage.
- \(M_\sigma\) is materialized, corpus-derived, and frozen for the turn.
- \(W_t\) is a bounded, positive working view into \(M_\sigma\).
- \(T_t\) connects the problem-space lens to represented semantic structure.
- \(C\) validates structural existence.
- \(E\) retrieves.
- \(Q\) records epistemic limits.
- \(S\) interprets.

No additional semantic adjudicator—including a coherence evaluator—exists between execution and synthesis.
