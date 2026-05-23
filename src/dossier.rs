//! Dossier: the only admissible shape crossing the Frontier boundary (LIP-0008).
//!
//! A [`Dossier`] is assembled bottom-up by lower tiers (Mini, Operator,
//! Translator) before any Frontier call. The Frontier never sees raw user
//! input, unbounded context, or chat history — only the prepared case.
//! The Frontier's response, [`FrontierVerdict`], is bounded (yes/no with
//! reason) and is itself a content-citable record; it is **never** used
//! as evidence closure on its own.
//!
//! This module introduces the types; admission, dispatch, and
//! Frontier-side wiring belong to later patches.
//!
//! See `LogLine-Foundation/governance/lips/LIP-0008-llm-tier-discipline-and-dossier-discipline.md`.

use serde::{Deserialize, Serialize};

use crate::capability::CostEnvelope;
use crate::evidence::EvidenceRecord;

/// A bounded decision the Frontier is asked to rule on. The institution
/// frames the question; the Frontier does not invent it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DecisionRequest {
    pub action_id: String,
    pub scope: String,
}

/// A candidate already prepared by lower tiers. The Frontier picks among
/// or ratifies one of them — it does not author candidates.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub candidate_id: String,
    pub summary: String,
}

/// A structured absence observed by the pipeline before the dossier was
/// assembled. Surfaced to the Frontier so its verdict is informed by
/// what was NOT resolvable lower in the stack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GhostRecord {
    pub ghost_id: String,
    pub kind: String,
    pub reason: String,
}

/// The only admissible shape crossing the Frontier boundary.
///
/// Forbidden inputs to the Frontier (never modeled here, and rejected by
/// admission later): raw chat history, unbounded workspace context,
/// entire repo dumps, ambiguous "please solve this", provider miracle
/// prompts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dossier {
    pub dossier_id: String,
    pub target_decision: DecisionRequest,
    pub evidence_chain: Vec<EvidenceRecord>,
    pub alternatives: Vec<Candidate>,
    pub summary: String,
    pub cumulative_cost: CostEnvelope,
    pub frontier_question: String,
    pub known_ghosts: Vec<GhostRecord>,
}

/// The Frontier's bounded verdict on a [`Dossier`].
///
/// Yes/No, each with a reason. `signed_at` on `Yes` records when the
/// verdict was bound (ISO-8601 string at this stage; later patches may
/// tighten this to a canonical receipt id). `alternative_suggestion`
/// on `No` is optional and informational only — it is not authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "verdict")]
pub enum FrontierVerdict {
    Yes {
        reason: String,
        signed_at: String,
    },
    No {
        reason: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        alternative_suggestion: Option<String>,
    },
}
