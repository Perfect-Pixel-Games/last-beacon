---
name: rust-workspace-dev
description: Rust workflow for the Last Beacon game crate and Foundation Engine submodule. Use when planning, implementing, formatting, linting, testing, building, benchmarking, or validating Rust code.
---

# Rust Workspace Development

## Purpose
Use this skill for Rust project work in Last Beacon.

This repository is not a single root Cargo workspace. It is composed of:
- `game/`: the Last Beacon Cargo package and game-owned assets.
- `engine/`: the Foundation Engine Git submodule and its own Cargo workspace.

Do not assume crate layout before inspecting the relevant manifest and source files.

## Required pre-work
1. Read `.pi/skills/rust-coding-standards/SKILL.md` before writing, refactoring, generating, or reviewing Rust code.
2. Inspect the relevant manifest before planning architecture or commands:
   - `game/Cargo.toml` for Last Beacon game work.
   - `engine/Cargo.toml` for Foundation Engine work.
3. Inspect relevant source, assets, tests, examples, scripts, and config files before editing.
4. If work touches Foundation Engine code or integration, read `.pi/skills/foundation-architecture/SKILL.md`.
5. If work touches branches, commits, pushes, or the submodule pointer, read `.pi/skills/gitflow-workflow/SKILL.md`.
6. Prefer existing project patterns over introducing new structure.
7. If online research is needed and tools are available, research the specific crates/APIs involved and cite useful findings in the plan.

## Repository boundaries
- Last Beacon game code, game manifests, and game assets belong under `game/`.
- Foundation Engine source, reusable runtime systems, reusable editor-time systems, engine scripts, and engine-owned assets belong under `engine/`.
- Do not copy Foundation Engine crates into the root repository.
- Engine changes must be made and committed inside the `engine/` submodule.
- Engine edits are forbidden while `engine/` is on `main` or `dev`; use a valid `feature/*` or `hotfix/*` branch.
- Root commits may update the `engine` submodule pointer after engine commits are made.
- Track the exact engine commit hash that the game is bound to whenever the root submodule pointer changes.
- Integration into root or engine `dev`/`main` must go through pull requests, not direct merges.

## Game validation commands
Use these root commands for Last Beacon game-facing validation:

```cmd
scripts\validate.cmd
scripts\build.cmd --platform windows-x64 --configuration test --target game
scripts\run.cmd --platform windows-x64 --configuration test --target game
scripts\package.cmd --platform windows-x64 --configuration shipping --target game
```

Focused game Cargo commands should use the game manifest explicitly:

```bash
cargo fmt --manifest-path game/Cargo.toml -- --check
cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path game/Cargo.toml --all-features
cargo build --manifest-path game/Cargo.toml --all-features
cargo doc --manifest-path game/Cargo.toml --all-features --no-deps
```

## Engine validation commands
Use these commands for Foundation Engine work. Run them from the root with the `engine/` prefix or from inside `engine/` without the prefix:

```cmd
engine\scripts\format-project.cmd
engine\scripts\lint-project.cmd
engine\scripts\test-project.cmd
engine\scripts\compile-project.cmd
engine\scripts\doc-project.cmd
engine\scripts\validate-project.cmd
```

Focused engine checks should use engine package names from `engine/Cargo.toml`, for example:

```bash
git -C engine status --short
cargo check --manifest-path engine/Cargo.toml -p foundation
cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library
cargo doc --manifest-path engine/Cargo.toml --workspace --all-features --no-deps
```

## Alternate engine path
The root scripts default to `engine/`. If `FOUNDATION_ENGINE_PATH` is set, scripts may use a different Foundation checkout.

Rules:
- Inspect the alternate checkout before relying on it.
- Do not modify an alternate checkout unless the user explicitly asks to work there.
- If modifying an alternate checkout, apply the same Gitflow rules as the `engine/` submodule.
- Record the alternate path and validation implications in the plan/tracker.

## Workflow

### 1. Validate environment
Before the first Rust task in a session, validate the toolchain when needed:

```cmd
cargo --version
rustc --version
scripts\validate.cmd
```

For engine work, use:

```cmd
engine\scripts\validate-env.cmd
```

If validation fails, explain whether `cargo`, `rustc`, the game manifest, the engine submodule, or required scripts are missing.

### 2. Plan a feature
When the user asks to plan a Rust feature:
1. Read the relevant manifest and source files.
2. Identify whether the feature is game-only, engine-only, or cross-repository.
3. Identify affected modules, public APIs, feature flags, dependency changes, tests, validation commands, and compatibility risks.
4. For engine or cross-repository features, plan engine branch, engine validation, exact engine commit hash tracking, engine pull request readiness, root submodule pointer update, root pull request readiness, and root validation.
5. Use `gpt-5.5` for planning. Never use Anthropic models.

### 3. Implement a feature
When the user asks to implement a feature:
1. Review the approved plan and tracker before code edits.
2. Read and apply `.pi/skills/rust-coding-standards/SKILL.md` before Rust edits.
3. Use `gpt-5.4` for implementation. Never use Anthropic models.
4. Keep changes focused, idiomatic, and consistent with existing Rust style and project coding standards.
5. Add or update tests for behavior changes where practical.
6. For engine changes, confirm `engine/` is on a valid `feature/*` or `hotfix/*` branch before editing, commit inside `engine/`, record the exact engine commit hash bound to the game, then commit the root submodule pointer update.
7. Run validation appropriate to the changed repository or repositories unless the user says not to.
8. Push branches for pull requests; do not merge directly into `main` or `dev`.

### 4. Review a feature
When the user asks for a review:
1. Read and apply `.pi/skills/rust-coding-standards/SKILL.md` before reviewing Rust code.
2. Use `gpt-5.5` for review. Never use Anthropic models.
3. Review against the plan, tracker, user request, changed root files, changed engine files, validation results, and project coding standards.
4. Confirm engine commits, exact bound engine commit hash, pull request readiness, and root submodule pointer updates are coherent when engine work is involved.
5. Record findings in the tracker if the feature workflow is in use.

## Rust-specific guidance
- Prefer small modules with explicit ownership and error handling.
- Avoid unnecessary `unwrap()`/`expect()` in library/runtime code unless a panic is intentional and documented.
- Preserve public API compatibility unless the plan explicitly allows a breaking change.
- Document new public items when appropriate.
- Keep dependency additions minimal and justified in the relevant `Cargo.toml`.
- Consider feature flags, target platforms, async runtime choices, and WASM constraints when relevant.
- Treat format, lint, tests, build, and documentation generation as the default validation evidence for changed Rust code.
