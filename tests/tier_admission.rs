//! LIP-0008 tier × grammar admission matrix.
//!
//! Exhaustive 3 × 4 = 12 cell matrix asserting the constitutional rule
//! encoded in `GrammarKind::admits`. Per-substrate admission (i.e. whether
//! a particular `CapabilityManifest` accepts the pair) is enforced
//! elsewhere and is not exercised here.

use constitutional_runtime::{GrammarKind, LlmTier};

#[test]
fn tier_grammar_admission_matrix() {
    use GrammarKind::*;
    use LlmTier::*;

    // (grammar, tier, expected_admits)
    let matrix: &[(GrammarKind, LlmTier, bool)] = &[
        // Operational ingress: Mini and Operator only.
        (Operational, Mini,       true),
        (Operational, Operator,   true),
        (Operational, Translator, false),
        (Operational, Frontier,   false),
        // Strong ingress: Operator and Translator only.
        (Strong,      Mini,       false),
        (Strong,      Operator,   true),
        (Strong,      Translator, true),
        (Strong,      Frontier,   false),
        // Dossier ingress: Frontier only.
        (Dossier,     Mini,       false),
        (Dossier,     Operator,   false),
        (Dossier,     Translator, false),
        (Dossier,     Frontier,   true),
    ];

    for (grammar, tier, expected) in matrix {
        let got = grammar.admits(tier);
        assert_eq!(
            got, *expected,
            "GrammarKind::{:?}.admits(LlmTier::{:?}) = {}, expected {}",
            grammar, tier, got, expected
        );
    }
}
