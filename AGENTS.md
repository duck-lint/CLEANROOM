1. Authority
   accepted docs + explicit operator decisions

2. Scope
   one phase / one seam at a time

3. Autonomy
   mechanical implementation and validation are autonomous

4. Stop conditions
   unresolved semantic decision → stop, don't guess

Within the explicitly authorized phase:

- inspect any relevant repository source, history, tests, schemas, and accepted documentation;
- make all necessary in-scope implementation changes;
- run all relevant non-destructive validation;
- fix mechanical defects discovered during implementation or review;
- refactor implementation details when required to satisfy the accepted contract;
- do not ask for approval for ordinary local edits, tests, formatting, or mechanical corrections.

Stop and report instead of choosing when:
- existing authority does not determine a semantic decision;
- two accepted contracts materially conflict;
- satisfying the task requires changing a previously accepted authority boundary;
- work would materially expand into a later phase;
- corpus evidence is required but unavailable;
- a destructive/external action was not authorized.

When stopping for a decision, return:
1. exact decision;
2. evidence;
3. available options;
4. consequences;
5. recommendation only if evidence licenses one.
