//! LLM tier and grammar discipline (LIP-0008).
//!
//! Constitutional rules for how LLMs participate in the pipeline.
//! Four tiers ([`LlmTier::Mini`], [`LlmTier::Operator`], [`LlmTier::Translator`],
//! [`LlmTier::Frontier`]) and three grammars ([`GrammarKind::Operational`],
//! [`GrammarKind::Strong`], [`GrammarKind::Dossier`]). Each tier carries the
//! smallest grammar it can honestly emit; raising tier is an efficiency
//! failure, not a capability badge.
//!
//! These types make LIP-0008 representable in the type system. They do not
//! implement admission enforcement — that belongs to a later patch that
//! teaches `admission` and the planning compiler to consult tier × grammar.
//!
//! See `LogLine-Foundation/governance/lips/LIP-0008-llm-tier-discipline-and-dossier-discipline.md`.

use serde::{Deserialize, Serialize};

/// The four LLM tiers.
///
/// Ordering reflects typical model size and call-cost envelope, not
/// authority — no tier can close evidence or authorize material execution.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmTier {
    /// 1.5–3.5B (or hot local 7B). Classify, extract, mark uncertainty,
    /// propose tiny candidate fragments. Sustained 24/7 call pattern.
    Mini,
    /// 9–14B local. Conduct session, decompose goals, route workorders.
    Operator,
    /// 9–14B local, escalates when needed. Natural language → LogLine
    /// candidate.
    Translator,
    /// External API. Receives a prepared [`crate::Dossier`] and returns a
    /// bounded verdict. Called rarely.
    Frontier,
}

/// The three grammars admissible at ingress boundaries.
///
/// [`GrammarKind::Operational`] is the runtime's line-oriented surface
/// grammar (see [`crate::operational_grammar`]). [`GrammarKind::Strong`]
/// is the JSON canonical IR ingress (see [`crate::strong_grammar`]).
/// [`GrammarKind::Dossier`] is the only shape that crosses the Frontier
/// boundary (see [`crate::Dossier`]).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrammarKind {
    /// Line-oriented surface grammar. Mini and Operator tiers.
    Operational,
    /// JSON canonical IR ingress. Operator and Translator tiers.
    Strong,
    /// Prepared dossier with evidence chain and bounded question.
    /// Frontier tier only.
    Dossier,
}

impl GrammarKind {
    /// Constitutional rule: which tiers may legitimately emit via this
    /// grammar.
    ///
    /// This is the matrix from LIP-0008 §5. The per-substrate admission
    /// (whether a particular [`crate::CapabilityManifest`] accepts the
    /// pair) is enforced separately in admission/planning code.
    ///
    /// ```
    /// use constitutional_runtime::{GrammarKind, LlmTier};
    /// assert!(GrammarKind::Operational.admits(&LlmTier::Mini));
    /// assert!(GrammarKind::Dossier.admits(&LlmTier::Frontier));
    /// assert!(!GrammarKind::Operational.admits(&LlmTier::Frontier));
    /// ```
    pub fn admits(&self, tier: &LlmTier) -> bool {
        matches!(
            (self, tier),
            (GrammarKind::Operational, LlmTier::Mini)
                | (GrammarKind::Operational, LlmTier::Operator)
                | (GrammarKind::Strong, LlmTier::Operator)
                | (GrammarKind::Strong, LlmTier::Translator)
                | (GrammarKind::Dossier, LlmTier::Frontier)
        )
    }
}
