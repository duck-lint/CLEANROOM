use semantic_traversal_core::problem_space::{BoundaryContribution, ProblemSpaceState};

#[derive(Clone)]
pub struct ScriptedBoundaryCase {
    pub expected_prior_version: Option<u64>,
    pub expected_newest_utterance: String,
    pub expected_previous_turn: Option<String>,
    pub contribution: BoundaryContribution,
}

impl ScriptedBoundaryCase {
    pub fn infer(
        &self,
        prior: Option<&ProblemSpaceState>,
        newest_utterance: &str,
        previous_turn: Option<&str>,
    ) -> BoundaryContribution {
        assert_eq!(
            prior.map(|state| state.version),
            self.expected_prior_version
        );
        assert_eq!(newest_utterance, self.expected_newest_utterance);
        assert_eq!(previous_turn, self.expected_previous_turn.as_deref());
        self.contribution.clone()
    }
}
