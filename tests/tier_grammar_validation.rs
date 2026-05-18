//! LIP-0008 ingress enforcement in validation.
//!
//! Exercises the three-pass admissibility against the new LIP-0008 rules:
//!
//! 1. Constitutional matrix on `(ingress_tier, ingress_grammar)`.
//! 2. Half-context rejection (one side `Some`, the other `None`).
//! 3. Per-manifest filter that does NOT fail early — only fails when no
//!    manifest survives `can_realize` ∧ `kind_allowed` ∧
//!    `evidence_realizable` ∧ `manifest_accepts_ingress`.

use std::collections::BTreeSet;

use constitutional_runtime::{
    validate_admissibility, AdmissibilityContext, CapabilityManifest, GrammarKind, IRPrimitive,
    IrNode, LlmTier, NodeId, PolicyClass, PrimitiveName, TargetRef, ValidationError,
};

fn observe_node() -> IrNode {
    IrNode {
        id: NodeId("n0".to_string()),
        body: IRPrimitive::Observe {
            target: TargetRef("subject".to_string()),
            scope: "default".to_string(),
        },
    }
}

fn base_ctx() -> AdmissibilityContext {
    AdmissibilityContext {
        policy_class: PolicyClass::C,
        runtime_permitted: true,
        at_execution_boundary: true,
        require_evidence_closure: false,
        ingress_tier: None,
        ingress_grammar: None,
    }
}

fn manifest(
    substrate_id: &str,
    tiers: Option<BTreeSet<LlmTier>>,
    grammars: Option<BTreeSet<GrammarKind>>,
) -> CapabilityManifest {
    CapabilityManifest {
        substrate_id: substrate_id.into(),
        substrate_version: "1".into(),
        supported_primitives: BTreeSet::from_iter([PrimitiveName::Observe]),
        supported_kinds: BTreeSet::new(),
        declared_guarantees: BTreeSet::new(),
        bindings: Vec::new(),
        allowed_ingress_tiers: tiers,
        allowed_grammars: grammars,
    }
}

// 1. Legacy: both context fields None → no LIP-0008 check applied.
#[test]
fn legacy_both_none_passes() {
    let node = observe_node();
    let ctx = base_ctx();
    let mfs = vec![manifest("any", None, None)];
    validate_admissibility(&node, &mfs, &ctx).expect("legacy both-None must pass");
}

// 2a. Tier-only present is half-context → IncompleteIngressContext.
#[test]
fn incomplete_tier_only_fails() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Mini);
    let mfs = vec![manifest("any", None, None)];
    let err = validate_admissibility(&node, &mfs, &ctx).unwrap_err();
    match err {
        ValidationError::IncompleteIngressContext {
            ingress_tier_present,
            ingress_grammar_present,
        } => {
            assert!(ingress_tier_present);
            assert!(!ingress_grammar_present);
        }
        other => panic!("expected IncompleteIngressContext, got {other:?}"),
    }
}

// 2b. Grammar-only present is half-context → IncompleteIngressContext.
#[test]
fn incomplete_grammar_only_fails() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_grammar = Some(GrammarKind::Operational);
    let mfs = vec![manifest("any", None, None)];
    let err = validate_admissibility(&node, &mfs, &ctx).unwrap_err();
    match err {
        ValidationError::IncompleteIngressContext {
            ingress_tier_present,
            ingress_grammar_present,
        } => {
            assert!(!ingress_tier_present);
            assert!(ingress_grammar_present);
        }
        other => panic!("expected IncompleteIngressContext, got {other:?}"),
    }
}

// 3. Constitutional matrix: (Mini, Strong) is illegitimate.
#[test]
fn mini_plus_strong_fails_constitutionally() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Mini);
    ctx.ingress_grammar = Some(GrammarKind::Strong);
    let mfs = vec![manifest("any", None, None)];
    let err = validate_admissibility(&node, &mfs, &ctx).unwrap_err();
    match err {
        ValidationError::TierGrammarIllegitimate { tier, grammar } => {
            assert_eq!(tier, LlmTier::Mini);
            assert_eq!(grammar, GrammarKind::Strong);
        }
        other => panic!("expected TierGrammarIllegitimate, got {other:?}"),
    }
}

// 4. Constitutional matrix: (Operator, Strong) is legitimate.
#[test]
fn operator_plus_strong_passes_constitutionally() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Operator);
    ctx.ingress_grammar = Some(GrammarKind::Strong);
    let mfs = vec![manifest("any", None, None)];
    validate_admissibility(&node, &mfs, &ctx)
        .expect("(Operator, Strong) is constitutionally legitimate");
}

// 5. Per-manifest tier rejection without alternative → NoCapabilityForIngress.
#[test]
fn manifest_restricts_tier_fails_when_no_alternative() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Mini);
    ctx.ingress_grammar = Some(GrammarKind::Operational);
    // Constitutional matrix passes (Operational admits Mini), but the only
    // manifest restricts ingress to {Operator}, so capability fails.
    let mfs = vec![manifest(
        "operator_only",
        Some(BTreeSet::from_iter([LlmTier::Operator])),
        None,
    )];
    let err = validate_admissibility(&node, &mfs, &ctx).unwrap_err();
    match err {
        ValidationError::NoCapabilityForIngress {
            primitive,
            tier,
            grammar,
        } => {
            assert_eq!(primitive, PrimitiveName::Observe);
            assert_eq!(tier, Some(LlmTier::Mini));
            assert_eq!(grammar, Some(GrammarKind::Operational));
        }
        other => panic!("expected NoCapabilityForIngress, got {other:?}"),
    }
}

// 6. Per-manifest grammar rejection without alternative → NoCapabilityForIngress.
#[test]
fn manifest_restricts_grammar_fails_when_no_alternative() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Operator);
    ctx.ingress_grammar = Some(GrammarKind::Strong);
    let mfs = vec![manifest(
        "operational_only",
        None,
        Some(BTreeSet::from_iter([GrammarKind::Operational])),
    )];
    let err = validate_admissibility(&node, &mfs, &ctx).unwrap_err();
    match err {
        ValidationError::NoCapabilityForIngress { grammar, .. } => {
            assert_eq!(grammar, Some(GrammarKind::Strong));
        }
        other => panic!("expected NoCapabilityForIngress, got {other:?}"),
    }
}

// 7. Multi-manifest: one rejects ingress, another accepts → passes (no early fail).
#[test]
fn multi_manifest_one_accepts_passes() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Mini);
    ctx.ingress_grammar = Some(GrammarKind::Operational);
    let mfs = vec![
        manifest(
            "rejects_mini",
            Some(BTreeSet::from_iter([LlmTier::Operator])),
            None,
        ),
        manifest(
            "accepts_mini",
            Some(BTreeSet::from_iter([LlmTier::Mini, LlmTier::Operator])),
            None,
        ),
    ];
    validate_admissibility(&node, &mfs, &ctx)
        .expect("multi-manifest must not fail early when an alternative accepts the ingress");
}

// 8. Legacy manifest (None+None) is permissive when context declares ingress.
#[test]
fn legacy_manifest_passes_when_context_declares_ingress() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Translator);
    ctx.ingress_grammar = Some(GrammarKind::Strong);
    let mfs = vec![manifest("legacy_permissive", None, None)];
    validate_admissibility(&node, &mfs, &ctx)
        .expect("manifest with allowed_*=None is treated as legacy permissive");
}

// 9. Manifest that explicitly accepts the (tier, grammar) → passes.
#[test]
fn manifest_explicitly_accepts_passes() {
    let node = observe_node();
    let mut ctx = base_ctx();
    ctx.ingress_tier = Some(LlmTier::Operator);
    ctx.ingress_grammar = Some(GrammarKind::Strong);
    let mfs = vec![manifest(
        "explicit_operator_strong",
        Some(BTreeSet::from_iter([LlmTier::Operator])),
        Some(BTreeSet::from_iter([GrammarKind::Strong])),
    )];
    validate_admissibility(&node, &mfs, &ctx)
        .expect("manifest that explicitly accepts (tier, grammar) must pass");
}
