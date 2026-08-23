# BSN Parser Panic Safety Plan

## Metadata
- Feature slug: `bsn-parser-panic-safety`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/bsn-parser-panic-safety`
- Engine branch: `feature/bsn-parser-panic-safety`
- Engine submodule pointer: `6faaf445edbe8fa1ba1548cec72a1d0c5663a669` (current, bound by `feature/async-scene-loading`; this feature branches engine from `dev` at `1bc59f9a0039dfe412b735c869a90f38a0d58582` instead, since the async-scene-loading work is unrelated and not yet merged)
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## User Request
Following a full codebase review against `README.md`'s vision, the user asked to act on the review's two open recommendations: (1) fix the confirmed BSN-parser panic risks, and (2) keep the codebase "rock solid" going forward. This plan covers item (1), the concrete actionable item from that review.

## Feature Summary
`engine/crates/foundation-runtime-library/src/dynamic_bsn.rs` resolves hand-authored `.bsn` scene content into spawnable Bevy reflection values at runtime. Four call sites use `.unwrap()` where the value being unwrapped can genuinely be `None`/absent as a direct result of malformed or stale `.bsn` content (a typo'd enum variant name, or a field whose Rust type was never `register_type::<T>()`'d) rather than an internal invariant violation. Each currently crashes the entire running game instead of surfacing a `DynamicBsnLoaderError` the way every structurally-identical sibling code path already does. This feature replaces those four `.unwrap()` calls with the same `.ok_or_else(...)` error-propagation pattern already established elsewhere in the same file, with no behavior change for well-formed content.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The fix itself is entirely inside the Foundation Engine's `dynamic_bsn.rs`. The root repository only needs a submodule pointer update once the engine change lands, per the Foundation Engine submodule rule.

## Codebase Research
- Confirmed via a focused code-review pass (read `dynamic_bsn.rs` and `dynamic_bsn_lexer.rs` in full) that of ~18 production-path `unwrap()`/`panic!()` call sites across both files, 14 are provably safe (guaranteed by an invariant established earlier in the same function or a `nom` parser combinator that cannot return `Err`), and exactly 4 are reachable from malformed `.bsn` content:
  - `dynamic_bsn.rs:303` — `BsnPatch::Struct` enum-variant resolution: `enumeration.variant(&symbol.1).unwrap()`. No prior check confirms `symbol.1` is an actual variant of the resolved enum type.
  - `dynamic_bsn.rs:396` — identical issue in `BsnPatch::NamedTuple`'s tuple-variant resolution.
  - `dynamic_bsn.rs:694` — `BsnExpr::StringLit`: `type_registry.get(expected_template_type).unwrap()`. `expected_template_type` is a field's declared Rust type, not something already confirmed present in the `TypeRegistry` — a field whose type implements `Reflect` but was never registered with `app.register_type::<T>()` hits this.
  - `dynamic_bsn.rs:854` — `create_reflect_default`, same unregistered-field-type issue, reached from `FloatLit`/`BoolLit`/`IntLit` literal assignment.
- Confirmed the fix pattern already exists in the same file for the structurally identical `BsnExpr::Struct`/`BsnExpr::NamedTuple` expression paths (as opposed to the top-level `BsnPatch::Struct`/`BsnPatch::NamedTuple` patch paths, which is what's broken):
  - Lines 598–602 and 670–674 guard the same `enumeration.variant(name)` lookup with `.ok_or_else(|| DynamicBsnLoaderError::UnknownType(symbol.as_path()))?`.
  - Lines 752–758 guard the same `type_registry.get(expected_template_type)` lookup with `.ok_or_else(|| DynamicBsnLoaderError::UnknownType(format!("TypeId {:?}", expected_template_type)))?`.
- `DynamicBsnLoaderError` (defined ~line 125) already has an `UnknownType(String)` variant used by both existing guarded patterns above — no new error variant is needed.
- Both functions containing the four unsafe call sites (`convert_bsn_patch_to_patch` and `convert_bsn_expr_to_reflect`/`create_reflect_default`) already return `Result<_, DynamicBsnLoaderError>` and use `?` throughout, so each fix is a mechanical swap from `.unwrap()` to `.ok_or_else(...)?` with no signature changes.
- A scene/widget that fails to resolve already degrades gracefully today (`apply_pending_last_beacon_bsn_widgets` catches a `Result::Err` from scene-patch resolution and calls `mark_widget_failed`, logging and clearing the loading marker instead of crashing) — so once `dynamic_bsn.rs` stops panicking and starts returning `Err`, that failure path already has somewhere safe to go without further plumbing changes.

## External Research
No external online research was performed; this is a self-contained internal bug fix with the correct pattern already present in the same file.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/dynamic_bsn.rs`: the four `.unwrap()` → `.ok_or_else(...)?` fixes, plus new regression tests.
- `engine/crates/foundation-runtime-library/src/lib.rs`: no change expected; confirms nothing in the public prelude needs updating since `DynamicBsnLoaderError` is already the established error type.
- `engine` submodule pointer in the root repository: bumped once the engine fix is committed and pushed.

## Proposed Implementation Approach
1. Create `feature/bsn-parser-panic-safety` from engine `dev` inside `engine/`.
2. TDD: add four regression tests to `dynamic_bsn.rs`'s existing test module, each constructing the minimal malformed input for one call site (an unknown enum variant name on a struct-variant patch; the same on a tuple-variant patch; a field typed with a `Reflect`-but-unregistered type assigned a string literal; the same assigned a numeric/bool literal) and asserting the load returns `Err(DynamicBsnLoaderError::UnknownType(_))` rather than panicking. Confirm each test panics (red) against the current code before the fix.
3. Apply the four mechanical fixes, each mirroring its already-guarded sibling exactly:
   - `dynamic_bsn.rs:303`: `enumeration.variant(&symbol.1).ok_or_else(|| DynamicBsnLoaderError::UnknownType(symbol.as_path()))?.as_struct_variant()?`
   - `dynamic_bsn.rs:396`: same pattern for the tuple-variant lookup.
   - `dynamic_bsn.rs:694`: `type_registry.get(expected_template_type).ok_or_else(|| DynamicBsnLoaderError::UnknownType(format!("TypeId {:?}", expected_template_type)))?`
   - `dynamic_bsn.rs:854` (`create_reflect_default`): same pattern as 694.
4. Confirm all four new tests pass (green) and the full existing `dynamic_bsn.rs` test suite still passes unchanged.
5. Run full engine validation (`engine/scripts/validate-project.cmd`).
6. Create `feature/bsn-parser-panic-safety` from root `dev` inside the root repository, update the `engine` submodule pointer to the new engine commit, and commit.
7. Run game validation (`scripts/validate.cmd`) to confirm Last Beacon still builds and passes against the updated engine pointer.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/bsn-parser-panic-safety`
- Engine commit expectation: four `.unwrap()` → `.ok_or_else(...)?` fixes plus four new regression tests in `dynamic_bsn.rs`, one commit.
- Bound engine commit hash: `<recorded once known>`
- Root pointer update required: `yes`

## Alternatives Considered
- Introduce a new, more specific `DynamicBsnLoaderError` variant for each site (e.g. `UnknownVariant`, `UnregisteredFieldType`) instead of reusing `UnknownType`: rejected for this pass — the existing sibling code paths already reuse `UnknownType` for both cases, and matching that convention exactly keeps the fix minimal and consistent rather than introducing a parallel taxonomy.
- Widen the pass to also add `#[must_use]`/broader error-message improvements across `dynamic_bsn.rs`: rejected — out of scope for a targeted robustness fix; would turn a small, low-risk change into an open-ended refactor of parser code neither requested nor evidenced as broken.

## Risks, Constraints, And Assumptions
- Risk: none of the four sites are expected to be exercised by any currently-authored `.bsn` asset in `game/assets/scenes/`, since all committed scenes presumably use only valid, registered types today — the fix is preventative, not a fix for an active in-game crash. Confirmed no currently-authored scene triggers this by the fact that `scripts/validate.cmd` and the game's own integration tests already load every `.bsn` scene asset without panicking.
- Assumption: `.as_path()` (used by the existing guarded siblings at lines 601 and 673) is the correct, already-established way to render a symbol for an `UnknownType` error message; the new call sites will match this exactly rather than inventing a new message format.
- Constraint: this fix must not change behavior for any currently well-formed `.bsn` content — verified by the existing `dynamic_bsn.rs` test suite and the game's `bsn_asset_flow.rs` integration tests (which load every authored scene and widget asset) continuing to pass unchanged.

## Open Questions
- None. Scope, fix pattern, and error type are all fully determined by existing code in the same file.

## Documentation Expectations
- No public API surface changes — `DynamicBsnLoaderError::UnknownType` already exists and is already documented via its `#[error(...)]` message.
- No `docs/` or `engine/docs/` changes expected; this is an internal robustness fix, not a behavior or API change developers need to be told about.
- Generated documentation (`engine/scripts/doc-project.cmd`) must still be run and pass as part of validation.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Follow the exact `.ok_or_else(|| DynamicBsnLoaderError::UnknownType(...))?` pattern already present at lines 598–602, 670–674, and 752–758 — do not invent a different error-construction style for the four new sites.
- Write the four regression tests first and confirm they fail against the current `.unwrap()` code before applying the fix, per the project's test-driven-development expectation.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm none of the four fixes changed control flow for the success path (i.e. `.ok_or_else(...)?.as_struct_variant()?` / `.as_tuple_variant()?` still return the same value on `Some`).
- Confirm the four new tests actually exercise the intended failure mode (wrong variant name / unregistered field type) rather than failing for an unrelated reason.

## Success Criteria
- All four identified call sites return `Err(DynamicBsnLoaderError::UnknownType(_))` instead of panicking when given the malformed input each represents.
- Four new regression tests exist and pass; confirmed red before the fix, green after.
- Full existing engine and game test suites pass unchanged.
- `engine/scripts/validate-project.cmd` and `scripts/validate.cmd` both pass cleanly.

## Testing Methodology
- Engine validation: `engine/scripts/validate-project.cmd` (format, clippy `-D warnings`, full test suite, doc generation).
- Game validation: `scripts/validate.cmd` after the submodule pointer update, to confirm Last Beacon still builds and passes against the new engine commit.
