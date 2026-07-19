# Bevy UI Scenes Plan

## Metadata
- Feature slug: `bevy-ui-scenes`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/bevy-ui-scenes`
- Engine branch: `N/A`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## User Request
Convert the completed Svelte, TypeScript, and Tailwind UI prototype into in-game Bevy UI scenes. Keep the existing splash screens and credits scene. Keep the gameplay level as the current gameplay level, but replace its old pause overlay with the new prototype-inspired pause menu. The new game UI scenes should be: Main Menu, Settings Menu, Dashboard, Hangar, Garage, Mission Control, Fabrication, Silo Upgrades, and Pause Menu. Use mock data for every UI element and focus on proving scene flow. Do not recreate the website placeholder background or map art; the UI should sit over an existing or simple 3D/gameplay-style background.

## Feature Summary
This feature replaces Last Beacon's placeholder menu BSN assets with prototype-matched Bevy UI scenes. The work is game-owned because the concrete scene keys, game-specific menu flow, placeholder content, and Last Beacon visual language live under `game/`. No reusable Foundation Engine runtime changes are planned unless implementation discovers a missing scene-stack or BSN capability that cannot be handled in game assets.

## Feature Area Classification
- Area: `game`
- Primary area: `game`
- Rationale: The feature creates Last Beacon-specific BSN scene assets and scene registrations. Foundation's scene stack, menu buttons, pause opener, splash flow, and credits systems already provide the reusable runtime behavior.

## Codebase Research
- `game/src/scenes/mod.rs` currently registers `last-beacon/splash_pixel_perfect`, `last-beacon/splash_bevy`, `last-beacon/main_menu`, `last-beacon/options_menu`, `last-beacon/credits`, `last-beacon/gameplay_level`, and `last-beacon/pause_menu`.
- The startup flow currently opens Pixel Perfect splash, then Bevy splash, then `last-beacon/main_menu`. This should be preserved.
- `game/assets/scenes/main_menu.bsn`, `options_menu.bsn`, and `pause_menu.bsn` are simple placeholder UI scenes using `FoundationMenuButton` actions.
- `game/assets/scenes/gameplay_level.bsn` currently owns the gameplay placeholder through `FoundationSimpleGameplayLevel` and opens `last-beacon/pause_menu` through `FoundationPauseOpener`. This scene should remain the gameplay level and should continue opening the new pause scene key.
- `game/assets/scenes/credits.bsn`, `pixel_perfect_splash.bsn`, and `bevy_splash.bsn` should remain intact unless navigation references need minor updates.
- Foundation runtime supports `FoundationMenuButton` actions: `none`, `open_scene`, `open_overlay_scene`, `clear_and_open_scene`, `close_current`, `resume`, and `exit`.
- The UI prototype exists on the previously created `feature/ui-prototype` branch at commit `f4d2abb Add UI prototype`. The current planning branch is from `dev`, so implementation should either use `git show feature/ui-prototype:<path>` as reference or wait until that branch is merged into `dev`.
- Prototype routes and components map to game scenes as follows:
  - `/menu` -> Main Menu.
  - `/menu/settings` -> Settings Menu.
  - `/beacon` -> Dashboard.
  - `/beacon/hangar` -> Hangar.
  - `/beacon/garage` -> Garage.
  - `/beacon/missions` -> Mission Control.
  - `/beacon/fabrication` -> Fabrication.
  - `/beacon/upgrades` -> Silo Upgrades.
  - `PauseMenu.svelte` -> Pause Menu.

## External Research
No external online research was performed because this feature is driven by repository-owned prototype files and existing Bevy/Foundation scene patterns.

## Affected Files And Systems
- `game/src/scenes/mod.rs`: Add stable scene keys and registrations for Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades; update tests for required keys and registry mappings.
- `game/assets/scenes/main_menu.bsn`: Replace placeholder centered menu with the prototype's left-side main menu layout over a background/gameplay-style layer.
- `game/assets/scenes/options_menu.bsn`: Replace generic Foundation options placeholder with a prototype-style Settings Menu using mock rows and tabs/sections.
- `game/assets/scenes/pause_menu.bsn`: Replace placeholder pause menu with prototype-style left-side pause menu and current expedition mock stats.
- `game/assets/scenes/gameplay_level.bsn`: Keep current gameplay level and `FoundationPauseOpener`; update only if pause key or stack metadata changes.
- New `game/assets/scenes/*.bsn` files: Add Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades scenes.
- `game/assets/scenes/credits.bsn`: Preserve the existing credits scene; only ensure Main Menu navigation still opens it.
- `game/assets/scenes/pixel_perfect_splash.bsn` and `game/assets/scenes/bevy_splash.bsn`: Preserve splash behavior.

## Proposed Implementation Approach
1. Preserve current splash, credits, and gameplay-level behavior as baseline.
2. Add scene constants in `game/src/scenes/mod.rs` for:
   - `last-beacon/dashboard`
   - `last-beacon/hangar`
   - `last-beacon/garage`
   - `last-beacon/mission_control`
   - `last-beacon/fabrication`
   - `last-beacon/silo_upgrades`
3. Register new BSN files under `game/assets/scenes/`.
4. Keep `last-beacon/main_menu`, `last-beacon/options_menu`, and `last-beacon/pause_menu` as stable keys, but replace their BSN content.
5. Build a shared visual style directly in authored BSN assets: dark translucent panels, slate borders, compact uppercase labels, industrial accent colors, left/right/top navigation bars, and mock status/resource chips.
6. Represent the 3D background as either the existing gameplay-level scene visible below overlays where scene-stack presentation supports it, or as a simple non-map background/placeholder layer in the menu BSN. Do not recreate the website's placeholder map/background art.
7. Implement navigation flow with existing `FoundationMenuButton` actions:
   - Splash flow -> Main Menu unchanged.
   - Main Menu Continue / Quick Run / New Game -> Dashboard or Gameplay depending on label intent; default plan is Continue/New Game -> Dashboard for Beacon UI flow, with a Quick Run button opening the current gameplay level.
   - Main Menu Settings -> Settings overlay or full scene.
   - Main Menu Credits -> existing Credits scene.
   - Dashboard top nav -> Hangar, Garage, Mission Control, Fabrication, Silo Upgrades.
   - Hangar Launch Expedition -> current gameplay level.
   - Pause Resume -> `resume`.
   - Pause Settings -> Settings overlay.
   - Pause Abandon Run / Save and Quit to Menu -> clear stack and open Main Menu or Dashboard according to final UX choice.
8. Use static BSN-authored mock content for resources, robot cards, mission rows, fabrication modules, upgrade nodes, settings controls, and expedition stats.
9. Update Rust tests in `game/src/scenes/mod.rs` to assert stable scene keys and representative registry mappings.
10. Run focused game validation and full root validation.

## Submodule Plan
- Engine changes required: `No planned engine changes.`
- Engine branch: `N/A`
- Engine commit expectation: `N/A`
- Bound engine commit hash: `1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Root pointer update required: `No`

## Alternatives Considered
- Implement reusable UI builders in Rust instead of BSN assets: deferred because current project scenes are BSN-authored and this feature needs static mock UI rather than reusable runtime systems.
- Recreate the prototype's map and background panels exactly: rejected by user request; these are stand-ins and should be represented by current gameplay/simple 3D background instead.
- Replace the existing gameplay level: rejected by user request; gameplay level should remain current and only use the new pause menu.
- Modify Foundation Engine scene-stack/menu systems: not planned because existing actions appear sufficient for static flow.

## Risks, Constraints, And Assumptions
- BSN asset syntax and reflected Bevy UI components may make large static UI scenes verbose; implementation should favor clear scene structure over perfect CSS parity.
- Some prototype interactions, such as tabs or selecting mission/module/upgrade rows, may not be dynamic without new Rust systems. The plan assumes static mock states are acceptable as long as scene-to-scene flow works.
- If prototype source is not merged into `dev` before implementation, the implementation model must use the local `feature/ui-prototype` branch or commit `f4d2abb` as the reference.
- If a new interactive behavior is required beyond `FoundationMenuButton`, the plan may need revision to decide whether that behavior belongs in game code or reusable Foundation runtime code.
- The current planning branch has leftover untracked prototype build artifacts from switching away from `feature/ui-prototype`; they are not part of this feature and should not be committed.

## Open Questions
- Should Main Menu Continue/New Game open the Beacon Dashboard first, or should one of those actions open the current gameplay level directly? Proposed default: Continue/New Game open Dashboard, Quick Run opens gameplay.
- Should Settings be a full-screen scene or an overlay over the current menu/gameplay scene? Proposed default: open it as an overlay from Main Menu and Pause Menu.

## Documentation Expectations
- Public Rust API changes should be limited to scene-key constants. Any new public constants should have Rustdoc comments.
- No feature-level player documentation is required unless implementation adds non-obvious controls or scene-stack behavior.
- Generated Rust documentation must be produced before completion, or a user-approved waiver must be recorded in the tracker.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read this plan, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before editing.
- Confirm `feature/bevy-ui-scenes` is still based on root `dev` before implementation edits.
- Do not edit `engine/` unless the tracker is updated and a valid engine feature branch is created first.
- Use `git show feature/ui-prototype:<prototype-path>` for prototype reference if the prototype branch has not been merged into `dev`.
- Avoid committing `prototypes/` generated artifacts or dependency folders as part of this feature.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify old UI scenes are replaced while splash screens, credits, and gameplay level are preserved.
- Verify scene keys, navigation actions, and stack presentations match intended flow.
- Verify pause menu works from the current gameplay level.
- Verify mock data is visually present and not wired to real gameplay/settings state.

## Success Criteria
- Startup splash sequence still reaches Main Menu.
- Main Menu visually matches the prototype's left-side menu and opens Settings, Credits, Dashboard, and current gameplay flow as planned.
- Settings Menu, Dashboard, Hangar, Garage, Mission Control, Fabrication, Silo Upgrades, and Pause Menu are registered as Bevy/BSN scenes.
- Beacon navigation flow can move between Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades.
- Hangar can launch the existing gameplay level.
- The existing gameplay level still runs and opens the new Pause Menu.
- Credits scene remains available and unchanged in purpose.
- All UI data is static mock content.
- No Foundation Engine submodule changes are required unless explicitly approved later.

## Testing Methodology
- Focused checks:
  - `cargo fmt --manifest-path game/Cargo.toml -- --check`
  - `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`
  - `cargo test --manifest-path game/Cargo.toml --all-features`
  - `cargo build --manifest-path game/Cargo.toml --all-features`
  - `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`
- Game validation:
  - `scripts/validate.cmd`
- Launch/flow validation when practical:
  - `scripts/run.cmd --platform windows-x64 --configuration test --target game -- --scene last-beacon/main_menu`
  - `scripts/run.cmd --platform windows-x64 --configuration test --target game -- --scene last-beacon/gameplay_level`
  - Manual confirmation that Main Menu, Settings, Dashboard, Hangar, Garage, Mission Control, Fabrication, Silo Upgrades, Pause Menu, and Credits can be reached through intended controls.
