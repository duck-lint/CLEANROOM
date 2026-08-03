//! Synthesis-input contracts.
//!
//! These records package the exact inputs supplied to synthesis. They do not
//! synthesize an answer inside the deterministic crate, reinterpret retrieval,
//! or treat conversational continuity as corpus evidence.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    execution::RetrievalResult, packet::ExecutionLimits, problem_space::ProblemSpaceState,
    semantic_access::SemanticAccessPlan,
};

/// Complete deterministic input package supplied to a synthesis boundary.
///
/// It distinguishes current relational problem state, newest utterance,
/// immediately preceding conversational continuity, the semantic-access plan,
/// retrieval result, and measured execution limits. It does not synthesize an
/// answer or relabel prior conversation as corpus evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SynthesisInput {
    /// Accepted current relational problem-space state.
    pub current_problem_space: ProblemSpaceState,
    /// Newest utterance that remains the current focus.
    pub newest_utterance: CurrentUtterance,
    /// Immediately preceding completed turn, absent only for a fresh thread.
    pub previous_turn: Option<ConversationalContinuity>,
    /// Structurally proposed access plan that explains what was sought.
    pub semantic_access_plan: SemanticAccessPlan,
    /// Canonical returned semantic units and execution provenance.
    pub retrieval_result: RetrievalResult,
    /// Measured coverage and deterministic limits constraining claim scope.
    pub execution_limits: ExecutionLimits,
}

/// Newest user utterance supplied as the synthesis focus.
///
/// It is current conversational input, not corpus evidence or a problem-space
/// summary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CurrentUtterance {
    /// Stable utterance identity.
    pub utterance_id: String,
    /// Exact newest user surface text.
    pub text: String,
}

/// Immediately preceding completed turn supplied for local continuity.
///
/// Its type explicitly marks conversation rather than corpus evidence. It may
/// support referential continuity but cannot acquire retrieval authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationalContinuity {
    /// Stable completed-turn identity.
    pub turn_id: String,
    /// Previous user utterance surface text.
    pub user_utterance: String,
    /// Previous assistant response surface text.
    pub assistant_response: String,
}
