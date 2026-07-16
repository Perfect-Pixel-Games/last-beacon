---
name: foundation-architecture
description: Foundation Engine submodule boundaries for runtime systems, editor extension points, game integration, settings, scenes, assets, and validation.
---

# Foundation Architecture

## Purpose
Use this skill when planning, implementing, or reviewing Foundation Engine work, Last Beacon integration with Foundation crates, engine launch behavior, editor-time tooling, game settings, scene-stack behavior, reusable game components, BSN scenes, or Foundation-owned assets.

## Repository boundary
Foundation Engine is a Git submodule at `engine/`.

Rules:
- Foundation Engine source changes must be made inside `engine/`.
- Treat `engine/` as an independent repository with its own branch, commits, remotes, validation, docs, and release process.
- Root Last Beacon code must not contain copied Foundation Engine crates or patched engine source outside the submodule.
- Root changes may update the `engine` submodule pointer after engine commits are created.
- If an alternate `FOUNDATION_ENGINE_PATH` is used, inspect it before relying on it and do not modify it unless the user explicitly asks.

## Current root project layout
- `game/` owns Last Beacon game source, `foundation.game.toml`, game-specific manifests, and game-owned assets.
- `game/assets/` owns Last Beacon assets.
- `scripts/` owns game-facing build, run, package, and validation wrappers.
- `engine/` owns Foundation Engine source, engine scripts, engine docs, engine assets, and reusable engine crates.

## Current Foundation Engine layout
Inspect `engine/README.md` and `engine/Cargo.toml` before relying on exact crate names. The current engine direction is:
- `engine/crates/foundation` is the Foundation engine executable/wrapper around Bevy. It discovers game extension manifests and forwards launch arguments.
- `engine/crates/foundation-runtime-library` owns reusable runtime/game systems, reflected components, shared settings data, scene-stack APIs, splash/menu runtime behavior, credits, and persistence helpers that standalone games can use.
- `engine/crates/foundation-editor-library` owns Bevy-only editor-time extension points and reusable editor-time support. It is intentionally not a Jackdaw integration layer.
- `engine/games/template-game` owns the engine's template game extension and example content.

## Ownership rules
- Last Beacon-specific gameplay, content, configuration, manifests, startup glue, and assets belong under `game/`.
- Reusable Foundation runtime systems belong under `engine/crates/foundation-runtime-library`.
- Reusable Foundation editor-time extension points belong under `engine/crates/foundation-editor-library`.
- Engine launcher behavior belongs under `engine/crates/foundation`.
- Engine template/example behavior belongs under `engine/games/template-game`.
- Do not add Last Beacon-specific behavior to generic Foundation crates unless the feature plan clearly states that the behavior is reusable engine functionality.

## Dependency rules
- Last Beacon may depend on Foundation crates through paths under `../engine/...` from `game/Cargo.toml`.
- Foundation runtime code should not depend on Last Beacon game crates.
- The Foundation engine executable should discover or launch game extensions rather than hard-coding Last Beacon behavior unless a plan explicitly approves a bundled/static integration path.
- Do not add Jackdaw dependencies unless the user explicitly approves a major architecture change. The current Foundation direction is Bevy-only.
- Keep dependency additions minimal and justify them in the plan.

## Settings, scenes, and assets
- Shared settings data, defaults, validation, and file persistence belong in Foundation runtime code when standalone games need them.
- Last Beacon-specific fallback settings, player-facing content, and content selection belong in `game/`.
- Scene-stack and reusable scene lifecycle APIs belong in Foundation runtime code.
- Last Beacon concrete scenes and game-specific scene registration belong in `game/`.
- Foundation-owned assets may live under `engine/assets` and are packaged separately under `assets/engine` when present.
- Last Beacon-owned assets live under `game/assets`.

## Gitflow requirements for Foundation work
For any engine change:
1. Read `.pi/skills/gitflow-workflow/SKILL.md`.
2. Verify `engine/` is on a `feature/*` or `hotfix/*` branch created from the engine repository's correct base.
3. Stop before editing if `engine/` is on `main` or `dev`; switch or create a valid `feature/*` or `hotfix/*` branch first.
4. Commit and push engine changes inside `engine/` before committing the root submodule pointer update.
5. Record engine branch, exact engine commit hash bound to the game, engine push state, engine pull request state, root branch, root commit, root pull request state, and root submodule pointer state in the tracker.
6. Do not merge directly into `main` or `dev`; all engine and root integration must go through pull requests.
7. Do not mark engine work complete while `git -C engine status --short` contains uncommitted source changes.

## Verification
- Check `game/Cargo.toml` for Last Beacon dependencies and feature flags.
- Check `engine/Cargo.toml` for Foundation workspace members and package names.
- Run focused checks for changed Foundation crates before full validation when practical.
- Run root `scripts\validate.cmd` after cross-repository changes that affect Last Beacon integration.
- Run engine validation wrappers for Foundation Engine changes.
- Confirm generated docs include changed public APIs in the relevant repository.
