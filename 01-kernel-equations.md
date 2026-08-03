# Kernel Equations

## Purpose

These equations describe the runtime without assuming any existing classes, modules, or implementation architecture.

The system contains two semantically interpretive operations:

1. inference, which deconstructs the utterance and constructs the traversal;
2. synthesis, which interprets the returned semantic units and produces the answer.

Everything between them is structural conformance, deterministic execution, and provenance-preserving packet construction.

## Terms

Let:

- \(u_t\) be the newest user utterance at turn \(t\);
- \(P_{t-1}\) be the aggregate problem-space state before the newest utterance;
- \(B_t\) be the boundary markers contributed by the newest utterance;
- \(P_t\) be the updated aggregate problem space;
- \(M_\sigma\) be the immutable projected semantic-space snapshot used for the turn;
- \(W_t\) be the bounded, positive, expandable working projection activated by the current problem space;
- \(T_t\) be the inferred semantic-access plan;
- \(C\) be structural conformance;
- \(R_t\) be the retrieval result;
- \(L_t\) be measured execution limits and coverage facts;
- \(A_t\) be the synthesized answer.

## 1. Utterance deconstruction

\[
B_t = D(P_{t-1}, u_t)
\]

The inference model deconstructs the newest utterance while considering the existing problem-space context.

The result is not an answer. It is a contribution of contextual boundary markers, such as:

- current focus;
- referenced or newly introduced objects;
- distinctions;
- temporal or comparative orientation;
- unresolved references;
- changes to previous assumptions;
- requested evidentiary form;
- continuity with or departure from previous turns.

## 2. Problem-space evolution

\[
P_t = U(P_{t-1}, B_t)
\]

The thread is treated as a continuously morphing problem space.

A fresh thread begins with an empty or explicitly initialized \(P_0\).

## 3. Problem-space-shaped activation

\[
W_t^{(0)} = A_{\mathrm{cfg}}(M_\sigma, P_t, u_t)
\]

The runtime deterministically activates a bounded working view of the frozen semantic projection using the current problem-space state, the newest utterance, and configured surface budgets.

Activation is positive-only: it controls what is presently visible, not what is relevant or what exists.

The working view may be expanded during the second inference call:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma, W_t^{(k)}, q_k, \beta)
\]

## 4. Semantic-access inference

\[
T_t = I_2(P_t, u_t, W_t)
\]

A fresh second inference call explores the activated semantic space, resolves canonical addresses, and emits the final semantic-access plan.

The plan connects the current problem-space lens to identifiers, semantic objects, semantic units, relations, occurrences, anchors, and retrieval surfaces represented in \(M_\sigma\).

The plan is a proposed collection route through an existing semantic space. It is not a new ontology.

## 5. Structural conformance

\[
C(T_t, M_\sigma) \in \{\text{valid}, \text{invalid}\}
\]

Equivalent shorthand:

\[
T_t \subseteq M_\sigma
\]

This means every identifier, address, surface, relation, direction, and transition used by the traversal is represented as a valid possibility or canonical instance in \(M_\sigma\).

Conformance performs no semantic interpretation. It checks membership and structural existence.

If invalid, the failure may be returned to the inference model for one bounded repair:

\[
T'_t = I_{\text{repair}}(P_t, u_t, M_\sigma, \text{violations}(T_t, M))
\]

The repair model receives structural violations, not a deterministic reinterpretation of the user's meaning.

## 6. Execution

For a conforming traversal:

\[
R_t = E(T_t, M_\sigma)
\]

Execution materializes the semantic units addressed by the traversal.

The retrieval result preserves:

- canonical object identity;
- canonical semantic-unit identity;
- object/unit containment and belonging;
- inherited identifiers;
- occurrence identity;
- link target and fragment identity;
- temporal anchors;
- retrieval-surface provenance;
- query and traversal provenance;
- execution status;
- deterministic bounds.

Execution does not decide whether a returned unit “really means” the same thing as a generated paraphrase.

## 7. Coverage and execution limits

\[
L_t = Q(T_t, R_t)
\]

Coverage is derived from what actually executed.

Coverage constrains claim scope. It does not reinterpret the meaning of retrieved units.

## 8. Synthesis

\[
A_t = S(P_t, u_t, T_t, R_t, L_t)
\]

The synthesis model receives:

- \(P_t\): thread continuity and aggregate problem-space context;
- \(u_t\): the newest utterance as the focus;
- \(T_t\): what was sought and through which semantic connections;
- \(R_t\): the returned semantic units and their provenance;
- \(L_t\): measured execution limits and claim constraints.

Synthesis determines what the retrieved semantic units imply for the current question.

## Complete compression

\[
\begin{aligned}
B_t &= D(P_{t-1}, u_t) \\
P_t &= U(P_{t-1}, B_t) \\
W_t^{(0)} &= A_{\mathrm{cfg}}(M_\sigma, P_t, u_t) \\
W_t^{(k+1)} &= \operatorname{expand}(M_\sigma, W_t^{(k)}, q_k, \beta) \\
T_t &= I_2(P_t, u_t, W_t) \\
C(T_t, M_\sigma) &\rightarrow \text{valid or bounded repair} \\
R_t &= E(T_t, M_\sigma) \\
L_t &= Q(T_t, R_t) \\
A_t &= S(P_t, u_t, T_t, R_t, L_t)
\end{aligned}
\]

## Architectural reading

- \(P_t\) is fluid, inferred, and thread-local.
- \(M_\sigma\) is materialized, corpus-derived, frozen for the turn, and closed over the accessible semantic substrate.
- \(W_t\) is the bounded, positive working view produced by the problem-space lens.
- \(T_t\) is the inferred semantic-access plan connecting the current problem space to represented semantic structure.
- \(C\) validates structural existence.
- \(E\) retrieves.
- \(Q\) records epistemic limits.
- \(S\) interprets.

No additional semantic adjudicator exists between \(E\) and \(S\).
