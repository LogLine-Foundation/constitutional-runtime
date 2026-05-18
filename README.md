# constitutional-runtime

> Execution is not sovereign. Material acts must be **semantically admissible**, **policy-permitted**, **capability-realizable**, and **evidentially accountable**.

A Rust crate that defines the constitution of legitimate execution: IR primitives, policy classes, capability manifests, evidence boundaries, and admission rulings. It is the **mechanism and shape** of accountable runtimes — not tied to any specific application or substrate.

## What it provides

| module | role |
|---|---|
| `ir` | Sixteen canonical IR primitives (`Observe`, `Collect`, `Compress`, `Classify`, `Prioritize`, `Compare`, `Route`, `Schedule`, `Execute`, `Emit`, `Persist`, `Confirm`, `Cancel`, `Reconcile`, `Fetch`, `Decide`). Surface-neutral. |
| `policy` | Policy classes A/B/C/D. A = read-only intel + bounded emit/route/schedule. B = bounded material. C = authority-sensitive at runtime boundary. D = reserved (representable in planning graphs, not executable at the normal boundary). |
| `capability` | `CapabilityManifest` — which primitives a substrate can realize, with optional kind filter and evidence guarantee. |
| `admission` | `evaluate_admission(act, boundary, gate, passport, visa)` — returns an `AdmissionRuling`. Each boundary crossing is explicit. |
| `evidence` | `EvidenceContract` + `EvidenceStore`. Three stores ship: `FileEvidenceStore` (default), `SqliteEvidenceStore` (feature `sqlite-evidence`), `SupabaseRestEvidenceStore` (feature `supabase-evidence`). |
| `lowering` | `Lowerer` trait + `StandardRuntimeLowerer` — IR primitives → `OperationalCommand`. |
| `operational_grammar` | Surface-level operational program parser. |
| `planning_compiler` | End-to-end: surface → IR graph → admission → routing → lowering → `CompiledOperationalPlan`. Deterministic, all-or-nothing, no side effects. |
| `decision` | `DecideResolver` — `Decide` nodes are resolved to concrete IR before lowering; the runtime lowerer never sees an unresolved `Decide`. |
| `validation` | `validate_structure`, `validate_policy`, `validate_capability`, `validate_admissibility`. |
| `act_identity` | `CanonicalActionId` — `namespace.verb` dotted identifiers. |
| `idempotency` | `IdempotencyClass` + `IdempotencyContract`. Idempotency is a **declared property**, never inferred. |
| `failure` | `RuntimeFailure` — stage-tagged structured failures. No free strings cross the runtime boundary. |
| `ingress` | `IngressMode` — fast path / premium exception. |
| `strong_grammar` | JSON-shaped canonical IR ingress (non-textual surface). |
| `plan_executor` | Executes a compiled plan against registered runtime commands. |

## Doctrine

```
Intelligence proposes.
Policy and capability govern.
The runtime lowers only what is admissible.
Evidence legitimizes outcomes.
LABs (substrates) execute. They do not govern.
Natural language never executes — only IR primitives cross the boundary.
Decide is not lowered by the runtime; it must be resolved first.
Receipts beat stories — every executable claim must be evidence-closable.
```

## Build

```bash
cargo build --release
cargo test
cargo test --features sqlite-evidence
cargo test --features supabase-evidence
```

`unsafe_code` is forbidden at workspace level.

## Substrate-neutral by design

The IR knows nothing about specific labs, hosts, or operators. `InferSurface::Named(id)` carries the substrate identifier as an opaque string — downstream config maps it to a real target. Routing classes (`Local` / `Cloud` / `Hybrid`) cover the generic cases without naming anything.

`StandardRuntimeLowerer` is the default lowerer. Applications may implement the `Lowerer` trait for substrate-specific targets without forking this crate.

## LIP-0008 — LLM tier and dossier discipline

The runtime can represent and validate LLM ingress tier × grammar discipline:
Mini/Operator/Translator/Frontier tiers and Operational/Strong/Dossier grammars.

Current support is intentionally narrow: the runtime validates declared ingress
legitimacy and manifest acceptance. It does not call LLMs, dispatch work, or
close evidence.

See the [LIP-0008 spec](https://github.com/LogLine-Foundation/governance/blob/main/lips/LIP-0008-llm-tier-discipline-and-dossier-discipline.md) in the governance repo for the constitutional rule the runtime obeys.

## Where it sits in the LogLine ecosystem

```
LogLine-Foundation/canon          → defines the receipt format (frozen)
LogLine-Foundation/conformance    → proves receipt-format obedience
LogLine-Foundation/engine         → LogLine 10-crate body (slot crates + CLI)
LogLine-Foundation/governance     → LIPs and process
LogLine-Foundation/constitutional-runtime  (this repo)
                                  → IR + policy + capability + evidence + admission
                                    mechanism, application-agnostic

Applications (downstream)         → use this crate as a dependency,
                                    bring their own substrate ids, policies,
                                    evidence stores, and surface verbs.
```

## License

MIT OR Apache-2.0.
