# Bevy UI Scenes Tracker

## Metadata
- Feature slug: `bevy-ui-scenes`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/bevy-ui-scenes`
- Engine branch: `N/A`
- Root branch base verification: `Rebased onto origin/dev at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 on 2026-07-19`
- Engine branch base verification: `N/A`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Overall status: `Number field and slider widget polish ready to commit`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Number field and slider widget polish completed with gpt-5.4; user requested checkpoint commit`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Reusable text-box completion commit 6ad3f5e pushed to origin/feature/bevy-ui-scenes; number/slider widget polish checkpoint commit pending.`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`
- Prototype reference state: `Prototype is now included through origin/dev at df9d52a7e2c94203904b8a7b72f96af57d1f6a80, which merged f4d2abb Add UI prototype.`
- Working tree note: `Untracked prototype build artifacts may remain locally under prototypes/ from the prior prototype branch; do not include them in this feature unless explicitly requested.`
- Current tweak state: `Reusable common widgets now default to filling the available parent width unless an author overrides the parent/wrapper width; text-box internals resize with the outer box, scrollbars appear per overflowing axis, and scrollbar corner space is reserved only when both axes are visible; validation passed; checkpoint commit pending.`

## Phase 1: Planning
**Status:** In progress  
**Goal:** Capture approved scope, branch state, affected files, and validation plan before implementation.

### Tasks
- [x] Confirm user-requested scope.
  - Status: Complete
  - Repository: `root`
  - Notes: User confirmed summary and added that the current gameplay level must remain while using the new pause menu.
- [x] Read required project skills.
  - Status: Complete
  - Repository: `root`
  - Notes: Read feature planning, gitflow, Foundation architecture, Rust workspace, and Rust coding standards guidance.
- [x] Inspect relevant manifests and scene files.
  - Status: Complete
  - Repository: `root`
  - Notes: Inspected `README.md`, `game/Cargo.toml`, `game/src/scenes/mod.rs`, current menu/gameplay BSN assets, and prototype references from `feature/ui-prototype`.
- [x] Create feature plan and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `docs/plans/bevy-ui-scenes/plan.md` and this tracker.
- [x] User approval to begin implementation.
  - Status: Complete
  - Repository: `root`
  - Notes: User approved the plan and asked that no pull request be created at the end so they can review the branch first.

### Validation
- Game validation: `N/A for planning-only changes`
- Engine validation: `N/A`
- Documentation generation: `Pending for implementation; planning docs created manually.`
- User confirmation: `Received on 2026-07-19`

## Phase 2: Scene Catalog And Navigation
**Status:** Complete  
**Goal:** Register all new scene keys and establish the intended menu/Beacon/gameplay navigation flow.

### Tasks
- [x] Update `game/src/scenes/mod.rs` with new scene constants and registry entries.
  - Status: Complete
  - Repository: `root`
  - Notes: Added Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades scene keys and BSN registrations.
- [x] Update Rust tests for required scene keys and representative registry mappings.
  - Status: Complete
  - Repository: `root`
  - Notes: Preserved splash, credits, gameplay, main menu, settings/options, and pause assertions and added representative new scene mappings.
- [x] Decide final Main Menu button routing.
  - Status: Complete
  - Repository: `root`
  - Notes: Continue/New Game open Dashboard, Quick Run opens the current gameplay level, Settings opens as an overlay, and Credits opens the existing credits scene.

### Validation
- Game validation: `Passed via scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed via cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Implementation approval received; no additional routing confirmation required`

## Phase 3: Prototype-Matched Static UI Scenes
**Status:** Complete  
**Goal:** Replace old UI BSN assets and add new static UI BSN assets that closely match the prototype layout with mock data.

### Tasks
- [x] Replace `game/assets/scenes/main_menu.bsn` with prototype-style Main Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added left menu rail, current-save mock panel, credits/settings/gameplay/dashboard flow, and simple 3D background component.
- [x] Replace `game/assets/scenes/options_menu.bsn` with prototype-style Settings Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static mock settings groups and tabs/sections; no real settings persistence.
- [x] Add Dashboard scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added resources, colony needs, equipped robot panels, and simple 3D background component.
- [x] Add Hangar scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added deployment display placeholder and Launch Expedition button to the current gameplay level.
- [x] Add Garage scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added robot roster cards and selected robot mock stats.
- [x] Add Mission Control scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static main/side/passive mission lists and selected mission detail panel.
- [x] Add Fabrication scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static module browser, robot stat deltas, and feature list.
- [x] Add Silo Upgrades scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static upgrade tree and selected-node detail panel.
- [x] Replace `game/assets/scenes/pause_menu.bsn` with prototype-style Pause Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added Resume, Abandon Run, Settings, Save and Quit to Menu, Save and Quit Game, and current expedition mock stats. The pause scene does not spawn its own 3D background so the current gameplay level remains visible underneath.
- [x] Preserve splash screens, credits scene, and current gameplay level.
  - Status: Complete
  - Repository: `root`
  - Notes: `gameplay_level.bsn`, splash BSN files, and credits BSN file were not changed; gameplay still opens `last-beacon/pause_menu`.

### Validation
- Game validation: `Passed via scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed via cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Runtime smoke checks launched each new scene with --scene and found no BSN parse/load errors before timeout termination.`

## Phase 4: Validation, Commit, And Push
**Status:** Awaiting final tracker commit  
**Goal:** Prove the feature builds, document validation, and prepare the branch for pull request review.

### Tasks
- [x] Run focused Rust checks.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo build --manifest-path game/Cargo.toml --all-features`, and `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- [x] Run root game validation.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `scripts/validate.cmd`.
- [x] Manually verify scene flow when practical.
  - Status: Complete
  - Repository: `root`
  - Notes: Smoke-launched Main Menu, Settings Menu, Dashboard, Hangar, Garage, Mission Control, Fabrication, Silo Upgrades, and Gameplay Level with `cargo run --manifest-path game/Cargo.toml -- --log-inline --scene <scene>`. Runs were terminated by timeout after startup and showed no BSN parse/load errors.
- [x] Commit completed work with required commit message format.
  - Status: Complete
  - Repository: `root`
  - Notes: Implementation commit `daaf8f6 Add Bevy UI scenes` includes the required changed file list.
- [x] Push `feature/bevy-ui-scenes` to origin.
  - Status: Complete
  - Repository: `root`
  - Notes: Pushed implementation commit `daaf8f6` to `origin/feature/bevy-ui-scenes`. No pull request was created per user request.

### Validation
- Game validation: `Passed scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Pending final implementation review/acceptance; no pull request will be created before user review.`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation and `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Do not implement until the user confirms this plan.
- Do not edit `engine/` unless a revised plan/tracker explicitly adds engine work and a valid engine feature branch is created.
- Use `git show feature/ui-prototype:<path>` to inspect prototype source if it is not merged into the implementation branch.
- Preserve `game/assets/scenes/credits.bsn`, `pixel_perfect_splash.bsn`, `bevy_splash.bsn`, and current gameplay behavior.
- Keep all UI data static/mock for this feature.

## Postponed Work
- Hooking UI to real save, settings, colony, mission, robot, fabrication, or upgrade data is postponed because the user requested mock data only.
- Recreating the prototype's placeholder map or background art is postponed/rejected because the user requested a 3D/gameplay-style background instead.
- Dynamic tab, selection, mission toggle, module selection, and upgrade selection behavior may be postponed unless achievable with existing Foundation menu buttons without new runtime systems.

## Notes / Issues / Oversights
- The feature branch was originally created from older local `dev` at c3aa296820dc54dc69e38e88dc065b84b878e208, then rebased onto latest `origin/dev` at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 after the prototype branch was merged.
- The old scene name `options_menu` may remain as the internal key for Settings Menu to minimize engine/menu integration churn, even though the user-facing label should be Settings Menu.
- Main Menu styling now starts a reusable BSN widget library under `game/assets/ui/widgets/main_menu/`. The first implementation adds game-owned `LastBeaconBsnWidget` slots so scenes can compose widget BSN assets without Foundation Engine changes.
- Dedicated widget assets currently cover Main Menu brand, menu buttons, UI Playground navigation, current save panel, and footer. Other scenes still use the earlier static layout and should be migrated as follow-up tweaks.
- Added `last-beacon/ui_playground` as a Feathers-gallery-inspired static widget showcase scene. It now displays only reusable common widgets: menu buttons, Beacon primary buttons, Beacon tabs, panels, and stat rows. Main Menu-specific brand/current-save/footer composition widgets are intentionally excluded.
- Added reusable showcase assets under `game/assets/ui/widgets/common/` so the playground is not built from Main Menu-specific widgets.
- Shared silo UI pass is now targeting the Svelte website's Beacon layout: absolute 16:9 frame feel, centered top tab navigation, resource chip group, 2px slate panel borders, slate-900 panel fills, cyan active tab underline, compact uppercase labels, and floating/edge panels over the existing simple 3D stand-in.
- Added game-owned Beacon button style markers so generic Foundation button interaction colors do not overwrite cyan primary actions or transparent Beacon top-nav tabs.
- Updated BSN asset flow tests to register the new marker types, wait long enough for all BSN assets to load, and serialize the asset-flow tests because the Bevy asset pipeline is shared enough for these app-level tests to race when run concurrently.
- Main Menu widgets were revised to match the Svelte prototype more closely: Tailwind slate palette values, `#fbbf24` menu accent, `rounded-sm`-style 2px radius, button border/padding proportions, rail width/padding/gaps, and NotoSans font application.
- Continue is explicitly treated as the primary menu button and now has a game-owned style enforcement marker so generic Foundation button interaction styling cannot override its yellow background. The Main Menu left rail and viewport placeholder no longer author visible borders.
- The Main Menu smoke run was terminated by timeout after startup and showed no BSN parse/load/apply errors; this confirms startup loading but is not a human visual review.

## Progress Log
- `2026-07-19`: User reviewed the first UI pass and requested reusable BSN widgets in a dedicated assets directory, starting with Main Menu styling.
- `2026-07-19`: Added game-owned BSN widget composition support, moved Main Menu pieces into `game/assets/ui/widgets/main_menu/`, rewrote `main_menu.bsn` to compose those widgets, and validated the focused checks plus root validation.
- `2026-07-19`: Committed and pushed Main Menu widget refinement as `5735044 Refine main menu widgets`; no pull request created.
- `2026-07-19`: Fetched latest origin and rebased `feature/bevy-ui-scenes` onto `origin/dev` at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 so the prototype merge is beneath the feature changes.
- `2026-07-19`: Refined Main Menu widget colors, button shape, font handling, padding, margins, and panel sizing to better match the Svelte prototype; validation passed and commit `9fd1d5e Match main menu prototype style` was pushed.
- `2026-07-19`: Made Continue persist as a yellow primary button with black text and removed visible borders from the Main Menu rail and viewport placeholder; focused validation passed and commit `9f712c3 Refine main menu primary styling` was pushed.
- `2026-07-19`: User clarified that the shared silo UI should be fixed to match the website more closely; implementation resumed for Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades shared chrome/styling.
- `2026-07-19`: Reworked shared Beacon/silo scene chrome for Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades to better match the Svelte website; passed `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, and `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- `2026-07-19`: User approved the shared Beacon/silo UI styling pass for commit.
- `2026-07-19`: Committed shared Beacon/silo UI styling pass as `7012ce1 Match shared silo UI prototype style`, committed tracker update as `02369c7 Update shared silo UI tracker`, and pushed both to `origin/feature/bevy-ui-scenes`.
- `2026-07-19`: User requested a Bevy Feathers-gallery-style UI playground scene to showcase reusable Last Beacon widgets and a Main Menu button to reach it; implementation started.
- `2026-07-19`: Added `last-beacon/ui_playground`, Main Menu `UI PLAYGROUND` button widget, scene registration/tests, and BSN asset-flow coverage. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `78246f9 Add UI playground scene`.
- `2026-07-19`: User clarified that the UI Playground should not showcase Main Menu-specific widgets such as the brand/current-save/footer composition. It should only show reusable widgets such as buttons, tabs, panels, and stat rows; correction started.
- `2026-07-19`: Reworked the UI Playground to exclude Main Menu-specific composition widgets and show common reusable widgets from `game/assets/ui/widgets/common/`. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `2b606c3 Show reusable widgets in UI playground`.
- `2026-07-19`: User requested reusable button variants to be generic primary/secondary/tertiary rather than menu/beacon-specific, with hover/click states, and requested functional tabs with hover/click states and remembered selection; implementation started.
- `2026-07-19`: Replaced menu/beacon-specific common button samples with reusable `primary`, `secondary`, and `tertiary` button widgets, added `LastBeaconUiButton` hover/pressed styling, added stateful `LastBeaconUiTab` selection memory, and updated the UI Playground showcase. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `13e5479 Add reusable widget interactions`.
- `2026-07-19`: User requested yellow/amber as the reusable UI accent colour instead of blue/cyan; implementation started.
- `2026-07-19`: Updated reusable primary/tertiary button and tab interaction styling plus common widget previews to use the yellow/amber accent. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `5fa96a2 Use amber reusable widget accent`.
- `2026-07-19`: Updated the UI Playground `BACK TO MENU` control to use `LastBeaconUiButton { variant: "secondary" }` so it shares the reusable button styling/hover/press behavior while keeping its scene navigation action. This was a BSN-only hot-reload edit, so no recompile was run.
- `2026-07-19`: User requested reusable input widgets using hot-reload without recompiling; added BSN-only reusable samples for text fields, text boxes, radio buttons, toggle buttons, combo boxes, number fields, and sliders, and added an `INPUTS` gallery to the UI Playground. Committed and pushed as `d82d410 Add reusable input widget samples`.
- `2026-07-19`: User requested a reusable property container that lays property names in a left column and values in a right column; added `ui/widgets/common/property_container.bsn`, moved the UI Playground input examples into it, and widened the input gallery for the two-column container. This was a BSN-only hot-reload edit, so no recompile was run. Committed and pushed as `8a4bb57 Add reusable property container sample`.
- `2026-07-19`: User requested the property container be essentially invisible; removed the container background, border, and padding, and changed it to fill the available width. This was a BSN-only hot-reload edit, so no recompile was run.
- `2026-07-19`: User requested removal of the `PROPERTY` and `VALUE` column headers from the property container; removed the header row. This was a BSN-only hot-reload edit, so no recompile was run.
- `2026-07-19`: User requested all UI Playground inputs be interactable and include values. Added runtime reusable input components for editable text inputs and simple value-backed buttons/text, wired them into text field, text box, radio, toggle, combo box, number field, and slider BSN samples, and added BSN asset-flow coverage for the common input widgets. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `7ab15c0 Make playground input widgets interactive`.
- `2026-07-19`: User reported UI Playground input bugs: text field placeholder/value mismatch, text box fixed placeholder and inconsistent colour, yellow input borders, combo box lacking an options popup, and slider lacking cursor dragging. BSN-only presentation fixes were hot-reloaded; combo popup and slider drag required runtime support, so Rust changes were made. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `192aa5e Fix playground input widget behavior`.
- `2026-07-19`: User reported follow-up input bugs: text box missing scrolling, combo options affecting layout instead of overlaying, number field minus rendering as `0`, and slider drag starting from zero instead of the current value/cursor anchor. Hot-reloaded BSN fixes for popup absolute positioning and minus glyph, added text-box scroll state/wheel handling, and anchored slider drags from current value. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `bca27e1 Fix playground input widget edge cases`.
- `2026-07-19`: User reported text-box scrolling/scrollbar still missing, requested direct numeric input for the number field, and requested authored min/max plus optional unit types for number and slider widgets. Added a visible text-box scrollbar, improved multiline scroll hit detection, added `LastBeaconUiNumberInput`, wired number field direct typing with `0..250 cm`, and simplified slider interaction so cursor position maps directly across its authored `0..100 %` range. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `b4ac81f Add numeric input ranges and scroll affordance`.
- `2026-07-19`: User reported regressions: text-box wheel still did not scroll, the number value appeared in the minus button while the readout was blank, and the slider remained cursor-offset. Fixed self-hosted text-input initialization/focus so numeric editability lives on the actual numeric text entity, moved the number readout back to a stable shell with `-`/`+` buttons isolated, directly offsets multiline editable text during wheel scroll, and corrected slider mapping from Bevy's `-0.5..0.5` relative cursor coordinates into `0..1`. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `641d05b Fix input widget regressions`.
- `2026-07-19`: User reported the text box remained unreliable: initial text was offset, top content still clipped, and arrow keys moved Bevy's internal editable text viewport without moving the authored scrollbar. Replaced the multiline text-box path with a custom stored text value plus visible-line window, leaving Bevy `EditableText` only for single-line/number inputs. Wheel scrolling now changes the visible line window and scrollbar thumb from the same state; typing, Enter, and Backspace update the stored multiline value without Bevy's internal scrolling. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `4f05fd4 Make text box scrolling robust`.
- `2026-07-19`: User confirmed the text box works and requested scrollbar indicator dragging. Added `LastBeaconUiTextScrollTrack` and drag state so the authored scrollbar track can be pressed/dragged to set the same visible-line window used by wheel scrolling, with the thumb refreshed from the shared state. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `e4f4309 Add text box scrollbar dragging`.
- `2026-07-19`: User reported the custom multiline text box lacked a blinking cursor and noted square missing-glyph boxes on radio buttons and combo box. Added a focused multiline text-box caret rendered as a blinking ASCII `|` in the custom visible-line window, and replaced unsupported icon-like glyphs (`●`, `○`, `▾`) with ASCII-safe labels. Confirmed the game currently uses `game/assets/fonts/NotoSans-Regular.ttf`; it is broad enough for normal UI text but should not be treated as an icon font without fallback. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `ac9ed5c Add text box caret and safe glyphs`.
- `2026-07-20`: User asked to use Noto Sans Symbols 2 for known symbol icons and make icons state-driven. Bundled `game/assets/fonts/NotoSansSymbols2-Regular.ttf`, added symbol font marker/runtime support, restored radio icons to `●`/`○`, restored combo arrow to `▾`, and added runtime refresh so radio icons follow selected option while combo arrow switches between `▾` and `▴` based on dropdown open state. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `3a01e76 Use symbol font for input icons`.
- `2026-07-20`: User reported the custom multiline text-box caret did not behave like the default text-field caret because it was end-only and could not move around text. Added byte-safe caret position state for multiline text boxes; typing and Enter now insert at the caret, Backspace/Delete remove around the caret, and Left/Right/Up/Down/Home/End move the caret while keeping the caret line visible. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `3436dcd Improve text box caret navigation`.
- `2026-07-20`: User reported the multiline text-box caret still did not match the text field because the inline `|` glyph was white, shifted text, and was too thin. Replaced the inline caret glyph with a separate absolute-positioned `LastBeaconUiTextBoxCaret` UI node using a 2px width and slate cursor color matching Bevy `TextCursorStyle` defaults, so caret blinking no longer changes the rendered text layout. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `1d980ec Style text box caret separately`.
- `2026-07-20`: User reported the separate multiline text-box caret was still offset and not drawn at the correct location. Initial attempts to align a custom caret via `TextLayoutInfo` still produced unacceptable visual offsets. User approved replacing the custom multiline editor with native Bevy `EditableText` plus authored scrollbar sync. Reworked the text box so Bevy owns caret rendering, keyboard navigation, text editing, click placement, and Unicode editing behavior; custom systems now only wheel/drag/sync scrollbar state via Bevy `TextScroll`. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, `scripts/validate.cmd`, and a timeout-terminated smoke launch of `last-beacon/ui_playground` with no BSN load errors. Committed and pushed as `5cf203d Use native text box editing`.
- `2026-07-20`: User reported native text-box scrolling did not work, then snapped to the end/original position, and then jittered up/down while scrolling. Stabilized scrolling by scheduling a post-layout scroll override after Bevy's native editable-text cursor scroll, initializing multiline native `EditableText` with the cursor at the start instead of the end, removing wrapper `ScrollPosition`, switching the authored text-box wrapper overflow from `Scroll` to `Clip`, fixing scrollbar drag progress direction, and making user scroll overrides persist every frame so Bevy does not immediately reset them. User confirmed the behavior is working much better and requested a checkpoint commit. Validation so far: `cargo fmt --manifest-path game/Cargo.toml` and `cargo check --manifest-path game/Cargo.toml` passed. Committed and pushed as `eea3984 Stabilize native text box scrolling`.
- `2026-07-20`: User confirmed text-box scrolling was stable, then requested follow-up polish. Matched the scrollbar thumb width to the track in BSN, corrected scrollbar track click/drag direction, fixed first-click caret placement by queuing Bevy native `TextEdit::MoveToPoint` when the wrapper focuses the native editable text, and added explicit multiline `LineHeight::Px(16.0)` so the initial rendered text and focused native editable text keep the same line spacing. User confirmed the text box is working perfectly and requested a checkpoint commit. Validation: `cargo fmt --manifest-path game/Cargo.toml -- --check` and `cargo check --manifest-path game/Cargo.toml` passed. Committed and pushed as `cee70dc Polish native text box interaction`.
- `2026-07-20`: User requested horizontal scrolling and then asked that text-box scrollbar dimensions/positions be reusable rather than hardcoded for the sample. Added horizontal `TextScroll.x` support via horizontal wheel deltas and Shift+wheel, authored a horizontal scrollbar, added reflected horizontal scrollbar track/thumb markers, and refactored scrollbar layout so track positions, track lengths, thumb sizes, thumb travel, thickness, and insets are calculated from the actual text-box/content sizes at runtime. User then clarified that mouse wheel and scrollbar dragging must be able to move the caret offscreen, while typing/deletion/Enter/arrows/Home/End/PageUp/PageDown should return the viewport to the caret. Added keyboard-driven caret-scroll requests so Bevy's native caret-visible scroll only wins after keyboard text editing/navigation, then the stored override adopts that position. User confirmed this is working perfectly and the text box can be considered done. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo check --manifest-path game/Cargo.toml`, and `cargo test --manifest-path game/Cargo.toml --test bsn_asset_flow --all-features`. Committed and pushed as `6ad3f5e Complete reusable text box scrolling`.
- `2026-07-20`: User reported the number field had the value in the left box and clarified the desired order is `[-][value][+]`, with `-`/`+` as increment buttons and the center value as the only numeric input. Reworked `number_field.bsn` into three touching boxes, moved the editable number to a centered child of the center box for vertical/horizontal centering, preserved authored layout during editable initialization, added numeric character filtering, and prevented value refresh feedback loops. User then requested slider polish: removed the internal `VOLUME` label, moved the percent readout into a centered right-hand slot, and kept the total widget width aligned with the other common inputs. User confirmed the result and requested a checkpoint commit. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo check --manifest-path game/Cargo.toml`, and `cargo test --manifest-path game/Cargo.toml --test bsn_asset_flow --all-features`. Committed and pushed as `9525795 Polish number and slider widgets`.
- `2026-07-20`: User requested reusable widget widths fill available space by default rather than hardcoding sample widths. Updated common widget BSNs so root widget widths default to `Percent(100)`, stretchable internal regions use `flex_grow`, and only true affordances remain fixed-width. Updated `docs/ui-widgets.md` to document the fluid-width convention and the user-overridable properties for each widget. Fixed the text box internals so multiline editable text fills the outer text-box content area, scrollbars are hidden independently when their axis does not overflow, and corner space is only reserved when both horizontal and vertical scrollbars are visible. User confirmed the final behavior and requested a commit. Validation passed: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo check --manifest-path game/Cargo.toml`, and `cargo test --manifest-path game/Cargo.toml --test bsn_asset_flow --all-features`.
- `2026-07-19`: Created `feature/bevy-ui-scenes` from `dev`.
- `2026-07-19`: Confirmed user scope, including preserving current gameplay level and replacing only the pause menu used by gameplay.
- `2026-07-19`: Created plan and tracker for user review.
- `2026-07-19`: User approved implementation and requested that no pull request be created before their review.
- `2026-07-19`: Implemented registered Bevy/BSN UI scenes, replaced the old Main Menu/Settings/Pause assets, preserved splash/credits/current gameplay level, and validated focused checks plus root validation.
- `2026-07-19`: Committed and pushed implementation as `daaf8f6 Add Bevy UI scenes`; no pull request created.
