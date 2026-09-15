# Vehicle Authoring Clarity Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make bad `.bsn` tag/socket references in the vehicle module system loud (rich warnings + a distinguishable failure marker + a red in-scene gizmo) instead of silent, and give authors a standalone guide with a real worked example for building a new vehicle without touching Rust.

**Architecture:** No data-model change. `game/src/shared/vehicle/connection.rs`'s `wire_last_beacon_vehicle_connections` gains richer diagnostics and a new `LastBeaconVehicleConnectionFailed` marker (distinct from `LastBeaconVehicleConnectionResolved`) on every skip path; `debug.rs` gains a gizmo system that visualizes it; two new `.bsn` assets plus a `docs/vehicle-authoring-guide.md` give a concrete, tested, loadable worked example.

**Tech Stack:** Rust, Bevy 0.19, Avian3D (pinned commit, see `docs/plans/vehicle-fixes/`), the project's own `.bsn` reflection-based scene format.

**Branch:** This continues on the existing `feature/vehicle-fixes` branch (root repo only — no engine changes in this plan). Do not create a new branch.

---

## Task 1: Richer lookup-failure diagnostics + `LastBeaconVehicleConnectionFailed`

**Files:**
- Modify: `game/src/shared/vehicle/connection.rs`

- [ ] **Step 1: Write the two new failing tests**

Add these two tests inside the existing `#[cfg(test)] mod tests` block in `game/src/shared/vehicle/connection.rs`, right after the `fixed_joint_no_longer_spawns_any_joint_entity` test (after its closing `}` at what is currently line 913):

```rust
    #[test]
    fn a_connection_naming_a_nonexistent_module_is_marked_failed_with_available_tags_listed() {
        let mut app = test_app();
        let world = app.world_mut();

        let module_a = spawn_resolved_module(
            world,
            "ModuleA",
            Transform::IDENTITY,
            "front",
            Vec3::ZERO,
            false,
        );
        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "Ghost".to_string(),
                socket_b: "whatever".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, connection_entity]);

        app.update();

        let failed = app
            .world()
            .get::<LastBeaconVehicleConnectionFailed>(connection_entity)
            .expect("connection naming a nonexistent module should be marked failed");
        assert!(
            failed.reason.contains("Ghost"),
            "reason should name the missing tag, got: {}",
            failed.reason
        );
        assert!(
            failed.reason.contains("ModuleA"),
            "reason should list the module tag that does exist, got: {}",
            failed.reason
        );
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
                .is_none(),
            "a failed connection must not also be marked Resolved"
        );
        assert_eq!(
            failed.marker_position,
            Some(Vec3::ZERO),
            "marker position should anchor to the one module that was found (ModuleA, at the origin)"
        );
    }

    #[test]
    fn a_connection_naming_a_nonexistent_socket_is_marked_failed_with_available_sockets_listed() {
        let mut app = test_app();
        let world = app.world_mut();

        let module_a = spawn_resolved_module(
            world,
            "ModuleA",
            Transform::IDENTITY,
            "front",
            Vec3::ZERO,
            false,
        );
        let module_b = spawn_resolved_module(
            world,
            "ModuleB",
            Transform::IDENTITY,
            "root",
            Vec3::ZERO,
            false,
        );
        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "nonexistent".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

        let failed = app
            .world()
            .get::<LastBeaconVehicleConnectionFailed>(connection_entity)
            .expect("connection naming a nonexistent socket should be marked failed");
        assert!(
            failed.reason.contains("nonexistent"),
            "reason should name the missing socket, got: {}",
            failed.reason
        );
        assert!(
            failed.reason.contains("front"),
            "reason should list the socket name that does exist on ModuleA, got: {}",
            failed.reason
        );
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
                .is_none()
        );
        assert!(
            failed.marker_position.is_some(),
            "both modules resolved, so a midpoint marker position should be set"
        );
    }
```

Also add one assertion to the existing `a_fixed_connection_fuses_the_absorbed_module_under_the_surviving_one` test (find its `assert!(app.world().get::<LastBeaconVehicleConnectionResolved>(connection_entity).is_some());` line) — add directly after it:

```rust
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionFailed>(connection_entity)
                .is_none(),
            "a successfully-resolved connection must not also be marked Failed"
        );
```

- [ ] **Step 2: Run the tests to verify they fail to compile**

Run: `cargo test --manifest-path game/Cargo.toml --lib shared::vehicle::connection::tests`
Expected: FAIL — compile error, `LastBeaconVehicleConnectionFailed` not found in this scope.

- [ ] **Step 3: Add `ResolvedModuleInstance: Copy` and the new `LastBeaconVehicleConnectionFailed` component**

In `game/src/shared/vehicle/connection.rs`, change the `ResolvedModuleInstance` struct definition (currently at what is line 512-515) from:

```rust
struct ResolvedModuleInstance {
    entity: Entity,
    is_pending: bool,
}
```

to:

```rust
#[derive(Clone, Copy)]
struct ResolvedModuleInstance {
    entity: Entity,
    is_pending: bool,
}
```

Add this new component definition immediately after `LastBeaconVehicleConnectionResolved`'s definition (after its closing `;` at what is currently line 55):

```rust

/// Marks a connection that permanently failed to resolve (a named module or
/// socket doesn't exist, or a socket's joint-type authoring is invalid) --
/// inserted *instead of* [`LastBeaconVehicleConnectionResolved`], so success
/// and failure are distinguishable without parsing logs. `reason` duplicates
/// (in a form a test can assert on directly) whatever was already logged via
/// `warn!` when this was inserted.
///
/// `pub` for the same reason as [`LastBeaconVehicleConnectionResolved`] --
/// `debug::draw_last_beacon_vehicle_connection_failure_gizmos` queries it
/// directly, and that function is itself `pub`, re-exported to the crate
/// root.
#[derive(Clone, Debug, Component)]
pub struct LastBeaconVehicleConnectionFailed {
    pub reason: String,
    /// Best-effort world position to draw a failure marker at -- the
    /// midpoint of both named modules' roots if both were found (a
    /// socket-level failure), the one module that *was* found if only one
    /// was, or `None` if neither resolved (nothing spatial to anchor a
    /// marker to).
    pub marker_position: Option<Vec3>,
}
```

- [ ] **Step 4: Add the candidate-listing helper functions**

Add these functions immediately after `find_named_module_instance` (after its closing `}` at what is currently line 539) and before the `SocketLookupError` enum:

```rust

/// Every module tag (`Name`) currently present in `vehicle_root`'s
/// descendant subtree -- used to list valid candidates in a "module not
/// found" warning. Same traversal as [`find_named_module_instance`].
fn collect_vehicle_module_tags(
    vehicle_root: Entity,
    children_query: &Query<&Children>,
    module_instances: &ModuleInstancesQuery,
) -> Vec<String> {
    children_query
        .iter_descendants(vehicle_root)
        .filter_map(|descendant_entity| {
            module_instances
                .get(descendant_entity)
                .ok()
                .map(|(_, name, _)| name.as_str().to_string())
        })
        .collect()
}

/// Every socket name present directly on `module_entity` -- used to list
/// valid candidates in a "socket not found" warning.
fn collect_module_socket_names(
    module_entity: Entity,
    children_query: &Query<&Children>,
    sockets: &SocketsQuery,
) -> Vec<String> {
    let Ok(module_children) = children_query.get(module_entity) else {
        return Vec::new();
    };
    module_children
        .iter()
        .filter_map(|child_entity| {
            sockets
                .get(child_entity)
                .ok()
                .map(|(_, socket, ..)| socket.socket_name.clone())
        })
        .collect()
}

/// Formats a candidate-name list for a warning message, e.g. `"front, back,
/// left"`, or `"none"` if empty.
fn format_name_list(names: &[String]) -> String {
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
    }
}
```

Add this function immediately after the `SocketLookupError`'s `Display` impl (after its closing `}` at what is currently line 572) and before the `ResolvedSocket` struct:

```rust

/// Describes why [`find_named_socket`] failed, including the module's actual
/// available socket names when the socket simply wasn't found (the other two
/// [`SocketLookupError`] variants already name the exact problem and don't
/// need a candidate list).
fn describe_socket_lookup_error(
    lookup_error: &SocketLookupError,
    socket_name: &str,
    module_name: &str,
    module_entity: Entity,
    children_query: &Query<&Children>,
    sockets: &SocketsQuery,
) -> String {
    match lookup_error {
        SocketLookupError::NotFound => {
            let available = collect_module_socket_names(module_entity, children_query, sockets);
            format!(
                "socket `{socket_name}` not found on module `{module_name}` (available sockets: {})",
                format_name_list(&available)
            )
        }
        other => format!("socket `{socket_name}` on module `{module_name}`: {other}"),
    }
}
```

- [ ] **Step 5: Rewrite the module/socket lookup-failure branches**

In `wire_last_beacon_vehicle_connections`, replace this whole block (currently lines 145-207, from the first `let module_a = find_named_module_instance(` through the closing `};` of the `socket_b` match):

```rust
        let module_a = find_named_module_instance(
            vehicle_root,
            &children_query,
            &module_instances,
            &connection.module_a,
        );
        let module_b = find_named_module_instance(
            vehicle_root,
            &children_query,
            &module_instances,
            &connection.module_b,
        );
        let (Some(module_a), Some(module_b)) = (module_a, module_b) else {
            warn!(
                "LastBeaconVehicleConnection on {connection_entity:?} names a module that does not exist in this vehicle (`{}` / `{}`); skipping.",
                connection.module_a, connection.module_b
            );
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionResolved);
            continue;
        };

        if module_a.is_pending || module_b.is_pending {
            continue;
        }

        let socket_a = match find_named_socket(
            module_a.entity,
            &children_query,
            &sockets,
            &connection.socket_a,
        ) {
            Ok(socket) => socket,
            Err(lookup_error) => {
                warn!(
                    "LastBeaconVehicleConnection on {connection_entity:?} names socket `{}` on module `{}`, but {lookup_error}; skipping.",
                    connection.socket_a, connection.module_a
                );
                commands
                    .entity(connection_entity)
                    .insert(LastBeaconVehicleConnectionResolved);
                continue;
            }
        };
        let socket_b = match find_named_socket(
            module_b.entity,
            &children_query,
            &sockets,
            &connection.socket_b,
        ) {
            Ok(socket) => socket,
            Err(lookup_error) => {
                warn!(
                    "LastBeaconVehicleConnection on {connection_entity:?} names socket `{}` on module `{}`, but {lookup_error}; skipping.",
                    connection.socket_b, connection.module_b
                );
                commands
                    .entity(connection_entity)
                    .insert(LastBeaconVehicleConnectionResolved);
                continue;
            }
        };
```

with:

```rust
        let module_a_lookup = find_named_module_instance(
            vehicle_root,
            &children_query,
            &module_instances,
            &connection.module_a,
        );
        let module_b_lookup = find_named_module_instance(
            vehicle_root,
            &children_query,
            &module_instances,
            &connection.module_b,
        );
        let (Some(module_a), Some(module_b)) = (module_a_lookup, module_b_lookup) else {
            let available_tags =
                collect_vehicle_module_tags(vehicle_root, &children_query, &module_instances);
            let mut reasons = Vec::new();
            if module_a_lookup.is_none() {
                reasons.push(format!(
                    "module `{}` not found (available modules: {})",
                    connection.module_a,
                    format_name_list(&available_tags)
                ));
            }
            if module_b_lookup.is_none() {
                reasons.push(format!(
                    "module `{}` not found (available modules: {})",
                    connection.module_b,
                    format_name_list(&available_tags)
                ));
            }
            let reason = reasons.join("; ");
            warn!("LastBeaconVehicleConnection on {connection_entity:?}: {reason}; skipping.");
            let marker_position = module_a_lookup
                .or(module_b_lookup)
                .and_then(|resolved| global_transforms.get(resolved.entity).ok())
                .map(GlobalTransform::translation);
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionFailed {
                    reason,
                    marker_position,
                });
            continue;
        };

        if module_a.is_pending || module_b.is_pending {
            continue;
        }

        let socket_a_result = find_named_socket(
            module_a.entity,
            &children_query,
            &sockets,
            &connection.socket_a,
        );
        let socket_b_result = find_named_socket(
            module_b.entity,
            &children_query,
            &sockets,
            &connection.socket_b,
        );
        let (socket_a, socket_b) = match (socket_a_result, socket_b_result) {
            (Ok(socket_a), Ok(socket_b)) => (socket_a, socket_b),
            (socket_a_result, socket_b_result) => {
                let mut reasons = Vec::new();
                if let Err(lookup_error) = &socket_a_result {
                    reasons.push(describe_socket_lookup_error(
                        lookup_error,
                        &connection.socket_a,
                        &connection.module_a,
                        module_a.entity,
                        &children_query,
                        &sockets,
                    ));
                }
                if let Err(lookup_error) = &socket_b_result {
                    reasons.push(describe_socket_lookup_error(
                        lookup_error,
                        &connection.socket_b,
                        &connection.module_b,
                        module_b.entity,
                        &children_query,
                        &sockets,
                    ));
                }
                let reason = reasons.join("; ");
                warn!("LastBeaconVehicleConnection on {connection_entity:?}: {reason}; skipping.");
                let marker_position = effective_global_transform(
                    module_a.entity,
                    &pending_fusions,
                    &global_transforms,
                )
                .zip(effective_global_transform(
                    module_b.entity,
                    &pending_fusions,
                    &global_transforms,
                ))
                .map(|(a, b)| (a.translation() + b.translation()) * 0.5);
                commands
                    .entity(connection_entity)
                    .insert(LastBeaconVehicleConnectionFailed {
                        reason,
                        marker_position,
                    });
                continue;
            }
        };
```

Note: `pending_fusions` is declared earlier in the function (before the `for` loop) and is already in scope here — this matches how the rest of the function already uses it.

- [ ] **Step 6: Update the pre-existing test that asserted the old (incorrect) behavior**

Find `a_socket_naming_neither_fixed_nor_hinge_joint_does_not_fuse_or_spawn_a_joint`'s final two assertions:

```rust
        assert!(
            app.world().get::<RigidBody>(module_b).is_some(),
            "no fusion should happen for a socket with no joint-type component"
        );
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
                .is_some(),
            "the connection should still be marked resolved, so it isn't retried forever"
        );
```

Replace the second assertion with:

```rust
        assert!(
            app.world().get::<RigidBody>(module_b).is_some(),
            "no fusion should happen for a socket with no joint-type component"
        );
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionFailed>(connection_entity)
                .is_some(),
            "the connection should be marked failed (not silently resolved), so it isn't retried \
             forever and tooling can tell it apart from a real success"
        );
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
                .is_none(),
            "a permanently-skipped connection must not also be marked Resolved"
        );
```

- [ ] **Step 7: Run the full connection.rs test suite to verify everything passes**

Run: `cargo test --manifest-path game/Cargo.toml --lib shared::vehicle::connection::tests`
Expected: PASS — all tests in that module pass (12 tests: the 10 pre-existing plus the 2 new ones).

- [ ] **Step 8: Commit**

```bash
git add game/src/shared/vehicle/connection.rs
git commit -m "$(cat <<'EOF'
Add richer diagnostics for bad vehicle connection tag/socket references

Files changed:
- game/src/shared/vehicle/connection.rs
EOF
)"
```

---

## Task 2: Visualize failed connections in the debug gizmo overlay

**Files:**
- Modify: `game/src/shared/vehicle/debug.rs`
- Modify: `game/src/shared/vehicle/mod.rs`
- Modify: `game/src/shared/mod.rs`

- [ ] **Step 1: Add the gizmo function**

In `game/src/shared/vehicle/debug.rs`, change the import block from:

```rust
use super::{
    LastBeaconVehicleFixedJoint, LastBeaconVehicleHingeJoint, LastBeaconVehicleModuleSocket,
};
```

to:

```rust
use super::connection::LastBeaconVehicleConnectionFailed;
use super::{
    LastBeaconVehicleFixedJoint, LastBeaconVehicleHingeJoint, LastBeaconVehicleModuleSocket,
};
```

Then add this at the end of the file (after `draw_last_beacon_vehicle_socket_gizmos`'s closing `}`):

```rust

/// Radius of the marker drawn at a failed connection's best-known anchor
/// position -- smaller than [`SOCKET_DOT_RADIUS`] so it doesn't visually
/// compete with real socket markers when both happen to be near each other.
const CONNECTION_FAILURE_MARKER_RADIUS: f32 = 0.15;
/// Bright red -- deliberately outside the cyan/orange/grey palette
/// [`draw_last_beacon_vehicle_socket_gizmos`] already uses, so a failure
/// reads unambiguously as "wrong," not just "a different socket kind."
const CONNECTION_FAILURE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

/// Draws a red marker at every [`LastBeaconVehicleConnectionFailed`]
/// connection's best-known anchor position, so an author can spot a bad
/// tag/socket reference in-scene, not just in logs. Draws nothing for a
/// failed connection whose `marker_position` is `None` (neither named module
/// resolved, so there's nothing spatial to anchor a marker to) -- the
/// `warn!` logged when it failed is the only signal in that case.
pub fn draw_last_beacon_vehicle_connection_failure_gizmos(
    mut gizmos: Gizmos,
    failed_connections: Query<&LastBeaconVehicleConnectionFailed>,
) {
    for failed in &failed_connections {
        let Some(marker_position) = failed.marker_position else {
            continue;
        };
        gizmos.sphere(
            marker_position,
            CONNECTION_FAILURE_MARKER_RADIUS,
            CONNECTION_FAILURE_COLOR,
        );
    }
}
```

- [ ] **Step 2: Re-export the new function from `mod.rs`**

In `game/src/shared/vehicle/mod.rs`, find:

```rust
#[cfg(feature = "dev-tools")]
pub use debug::draw_last_beacon_vehicle_socket_gizmos;
```

Replace with:

```rust
#[cfg(feature = "dev-tools")]
pub use debug::{
    draw_last_beacon_vehicle_connection_failure_gizmos, draw_last_beacon_vehicle_socket_gizmos,
};
```

- [ ] **Step 3: Register the new system**

In `game/src/shared/mod.rs`, find:

```rust
        #[cfg(feature = "dev-tools")]
        app.add_systems(Update, vehicle::draw_last_beacon_vehicle_socket_gizmos);
```

Replace with:

```rust
        #[cfg(feature = "dev-tools")]
        app.add_systems(
            Update,
            (
                vehicle::draw_last_beacon_vehicle_socket_gizmos,
                vehicle::draw_last_beacon_vehicle_connection_failure_gizmos,
            ),
        );
```

- [ ] **Step 4: Verify it compiles**

Run: `cargo check --manifest-path game/Cargo.toml --all-targets --all-features`
Expected: PASS — no errors. (No new automated test is added for the gizmo-drawing function itself: `draw_last_beacon_vehicle_socket_gizmos`, the existing precedent this mirrors, has none either — gizmo drawing is visual and not meaningfully unit-testable here. `LastBeaconVehicleConnectionFailed`'s `marker_position` field is already covered by Task 1's tests.)

- [ ] **Step 5: Commit**

```bash
git add game/src/shared/vehicle/debug.rs game/src/shared/vehicle/mod.rs game/src/shared/mod.rs
git commit -m "$(cat <<'EOF'
Draw a red gizmo marker at failed vehicle connections

Files changed:
- game/src/shared/mod.rs
- game/src/shared/vehicle/debug.rs
- game/src/shared/vehicle/mod.rs
EOF
)"
```

---

## Task 3: Worked-example module and vehicle assets

**Files:**
- Create: `game/assets/vehicle/modules/coupling_plate.bsn`
- Create: `game/assets/vehicle/examples/two_plate_coupling.bsn`
- Create: `game/tests/vehicle_example_two_plate_coupling.rs`

- [ ] **Step 1: Create the coupling-plate module asset**

Create `game/assets/vehicle/modules/coupling_plate.bsn`:

```
// A minimal two-socket module, used as the worked example in
// docs/vehicle-authoring-guide.md. Couples two other modules together via
// its "a"/"b" sockets (both welds). Uses the same "local +X = outward
// normal" convention as core.bsn/beam.bsn (see core.bsn's header comment
// for the full derivation) -- just with two sockets instead of six.
last_beacon::shared::vehicle::LastBeaconVehicleModuleCuboidShape(bevy_math::primitives::dim3::Cuboid { half_size: glam::Vec3 { x: 0.3, y: 0.05, z: 0.3 } })
avian3d::dynamics::rigid_body::RigidBody::Dynamic
avian3d::dynamics::rigid_body::mass_properties::components::Mass(2.0)
last_beacon::shared::vehicle::LastBeaconVehicleModuleColor(bevy_color::color::Color::Srgba(bevy_color::srgba::Srgba { red: 0.55, green: 0.45, blue: 0.3, alpha: 1.0 }))
bevy_ecs::hierarchy::Children [
    #A
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "a" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.3, y: 0.0, z: 0.0 } }
    last_beacon::shared::vehicle::LastBeaconVehicleFixedJoint { },

    // Normal -X: rotation of PI about Y (local +X -> world -X).
    #B
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "b" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -0.3, y: 0.0, z: 0.0 }, rotation: glam::Quat { x: 0.0, y: 1.0, z: 0.0, w: 0.0 } }
    last_beacon::shared::vehicle::LastBeaconVehicleFixedJoint { }
]
```

- [ ] **Step 2: Create the two-plate example vehicle asset**

Create `game/assets/vehicle/examples/two_plate_coupling.bsn`:

```
// Minimal 2-module worked example for docs/vehicle-authoring-guide.md: two
// coupling_plate.bsn instances joined by a single weld connection,
// referencing each other purely by author-chosen tag ("PlateOne"/"PlateTwo")
// and socket name ("a"/"b") -- no code in game/src knows this vehicle
// exists. Exercised by game/tests/vehicle_example_two_plate_coupling.rs.
#TwoPlateCoupling
bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 5.0, z: 0.0 } }
bevy_ecs::hierarchy::Children [
    #PlateOne
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/coupling_plate.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 } },

    #PlateTwo
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/coupling_plate.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 2.0, y: 0.0, z: 0.0 } },

    #PlateOneToPlateTwo
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "PlateOne", socket_a: "a", module_b: "PlateTwo", socket_b: "b" }
]
```

- [ ] **Step 3: Write the failing integration test**

Create `game/tests/vehicle_example_two_plate_coupling.rs`:

```rust
//! Proves `game/assets/vehicle/examples/two_plate_coupling.bsn` -- the
//! worked example referenced by `docs/vehicle-authoring-guide.md` -- is
//! real, loadable, non-drifting data: it settles under real physics without
//! panicking, and the weld actually pulls the two plates' sockets together.
//! Mirrors `vehicle_module_physics.rs`'s test harness pattern.

use std::time::{Duration, Instant};

use avian3d::prelude::{FixedJoint, RevoluteJoint, RigidBody};
use bevy::prelude::*;
use foundation_runtime_library::prelude::*;
use last_beacon::{asset_root, shared::vehicle::LastBeaconVehiclePlugin};

/// The two plates fuse into one compound body (see `connection.rs`'s doc
/// comment for why a weld no longer produces its own `RigidBody`).
const EXPECTED_RIGID_BODY_COUNT: usize = 1;

#[test]
fn two_plate_coupling_example_settles_under_real_physics_without_panicking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin {
        file_path: asset_root().to_string_lossy().to_string(),
        ..default()
    });
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_message::<SceneLoadRequested>();
    app.add_message::<SceneAdded>();
    app.add_message::<SceneFocused>();
    app.add_plugins(FoundationBsnAssetPlugin);
    app.init_asset::<Mesh>();
    app.init_asset::<StandardMaterial>();
    app.add_plugins(LastBeaconVehiclePlugin);
    app.add_plugins(FoundationPhysicsPlugin);

    app.finish();
    app.cleanup();

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn_bsn_asset("vehicle/examples/two_plate_coupling.bsn");
    });

    let steady_state_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(5));

        if rigid_body_count(&mut app) == EXPECTED_RIGID_BODY_COUNT {
            break;
        }

        assert!(
            Instant::now() < steady_state_deadline,
            "two_plate_coupling example never reached its expected steady state \
             ({EXPECTED_RIGID_BODY_COUNT} RigidBody) within the timeout; saw {} RigidBody",
            rigid_body_count(&mut app)
        );
    }

    // Keep stepping real physics ticks past first reaching steady state, same
    // rationale as vehicle_module_physics.rs: the panic class this guards
    // against fires from inside Avian's own physics schedule.
    for _ in 0..120 {
        app.update();
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_eq!(
        rigid_body_count(&mut app),
        EXPECTED_RIGID_BODY_COUNT,
        "two_plate_coupling example should still be at its expected steady state \
         after continued physics stepping"
    );

    let world = app.world_mut();
    assert_eq!(
        world.query::<&FixedJoint>().iter(world).count(),
        0,
        "a weld must never produce a FixedJoint entity"
    );
    assert_eq!(
        world.query::<&RevoluteJoint>().iter(world).count(),
        0,
        "this example has no hinges"
    );

    // PlateOne's "a" socket (local +0.3 on X) should coincide with PlateTwo's
    // "b" socket (local -0.3 on X) once the weld has fused them -- proving
    // the example's authored socket offsets are actually correct, not just
    // "didn't panic."
    let mut named_transforms = world.query::<(&Name, &GlobalTransform)>();
    let mut plate_one_global = None;
    let mut plate_two_global = None;
    for (name, transform) in named_transforms.iter(world) {
        match name.as_str() {
            "PlateOne" => plate_one_global = Some(*transform),
            "PlateTwo" => plate_two_global = Some(*transform),
            _ => {}
        }
    }
    let plate_one_global = plate_one_global.expect("PlateOne should exist");
    let plate_two_global = plate_two_global.expect("PlateTwo should exist");
    let plate_one_socket_a = plate_one_global.transform_point(Vec3::new(0.3, 0.0, 0.0));
    let plate_two_socket_b = plate_two_global.transform_point(Vec3::new(-0.3, 0.0, 0.0));
    assert!(
        plate_one_socket_a.distance(plate_two_socket_b) < 1e-3,
        "PlateOne's socket `a` ({plate_one_socket_a:?}) should coincide with PlateTwo's socket \
         `b` ({plate_two_socket_b:?}) once the weld has settled"
    );
}

fn rigid_body_count(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query::<&RigidBody>().iter(world).count()
}
```

- [ ] **Step 4: Run the test to verify it currently fails or passes as expected**

Run: `cargo test --manifest-path game/Cargo.toml --test vehicle_example_two_plate_coupling`
Expected: PASS. (This is integration-test-level verification of real assets rather than red/green unit TDD — there's no separate "make it fail first" step, since the assets and test are being authored together as the thing under test. If it fails, the failure message will point at whichever of the two new `.bsn` files has a mistake; re-check the socket offsets/rotations against Step 1/2 above before changing the test.)

- [ ] **Step 5: Commit**

```bash
git add game/assets/vehicle/modules/coupling_plate.bsn game/assets/vehicle/examples/two_plate_coupling.bsn game/tests/vehicle_example_two_plate_coupling.rs
git commit -m "$(cat <<'EOF'
Add a two-module worked-example vehicle asset and regression test

Files changed:
- game/assets/vehicle/examples/two_plate_coupling.bsn
- game/assets/vehicle/modules/coupling_plate.bsn
- game/tests/vehicle_example_two_plate_coupling.rs
EOF
)"
```

---

## Task 4: Authoring guide

**Files:**
- Create: `docs/vehicle-authoring-guide.md`
- Modify: `game/src/shared/vehicle/mod.rs`

- [ ] **Step 1: Write the guide**

Create `docs/vehicle-authoring-guide.md`:

```markdown
# Last Beacon Vehicle Authoring Guide

This document explains how to author a new vehicle module and wire it into a
vehicle, entirely in `.bsn` -- no Rust changes required. The full worked
example referenced throughout is real, loadable data:
`game/assets/vehicle/modules/coupling_plate.bsn` and
`game/assets/vehicle/examples/two_plate_coupling.bsn`, exercised by
`game/tests/vehicle_example_two_plate_coupling.rs`.

There is no fixed catalog of modules or vehicles in code. A module is
identified purely by its `.bsn` asset path
(`LastBeaconVehicleModuleInstance { asset_path }`), a module instance's tag
inside a vehicle is just its author-chosen `Name`, and a socket's id is just
its author-chosen `socket_name` string. `game/src/shared/vehicle/connection.rs`
resolves connections purely from these strings -- nothing in Rust
special-cases any particular module or vehicle layout.

## Module Anatomy

A module `.bsn` (see `game/assets/vehicle/modules/core.bsn`, `beam.bsn`,
`wheel.bsn`, `coupling_plate.bsn`) authors, directly on its root entity:

- **Shape**: `LastBeaconVehicleModuleCuboidShape(bevy_math::primitives::dim3::Cuboid { half_size })`
  or `LastBeaconVehicleModuleCylinderShape(bevy_math::primitives::dim3::Cylinder { radius, half_height })`.
  These are thin wrappers around Bevy's own shape primitives -- no
  Last-Beacon-specific shape data exists to keep in sync.
- **`avian3d::dynamics::rigid_body::RigidBody::Dynamic`**.
- **`avian3d::dynamics::rigid_body::mass_properties::components::Mass(<kg>)`**.
- **`LastBeaconVehicleModuleColor(bevy_color::color::Color::Srgba(...))`** --
  the one field that carries data of its own, since a `Handle<StandardMaterial>`
  can't exist as static `.bsn` data.
- A `Children` list of **sockets** -- see below.

## Sockets

Each socket is a child entity carrying:

- `LastBeaconVehicleModuleSocket { socket_name: "..." }` -- the id another
  module's connection references it by. Any string; only needs to be unique
  *within that module*.
- A `Transform` -- `translation` is the socket's local anchor offset from the
  module's own origin; `rotation` encodes the socket's outward-facing normal
  (see below).
- Exactly one of `LastBeaconVehicleFixedJoint {}` (a weld) or
  `LastBeaconVehicleHingeJoint {}` (a free-spinning hinge, only meaningful
  for wheel-like modules). There is no implicit default -- a socket
  authoring neither, or both, is treated as invalid and marked failed (see
  "Diagnosing A Bad Connection" below) rather than silently guessing.

## The Socket-Normal Convention

Every socket's *local* outward normal is defined as local **+X**. A socket's
`Transform.rotation` rotates that local +X into whatever direction the
socket should actually face, in the *module's own local space* -- the
socket's final world-space normal also depends on the module's own rotation,
the same way its world position does.

To connect two sockets, `wire_last_beacon_vehicle_connections` only
automates *position* (it translates the absorbed module so the two socket
positions coincide exactly). It does **not** automate rotation -- the
`.bsn` author is responsible for choosing each module's rotation so the two
sockets' *world* normals end up exactly opposing each other (pointing at
each other, not past each other).

A quaternion representing a rotation of angle `θ` about a unit axis `v` is:

```
Quat { x: v.x * sin(θ/2), y: v.y * sin(θ/2), z: v.z * sin(θ/2), w: cos(θ/2) }
```

Worked example -- `core.bsn`'s `Top` socket needs its local +X normal to end
up pointing world +Y, a 90-degree rotation about Z (`v = (0, 0, 1)`,
`θ = 90°`): `sin(45°) = cos(45°) ≈ 0.7071068`, giving
`Quat { x: 0.0, y: 0.0, z: 0.7071068, w: 0.7071068 }` -- exactly the value
authored in `core.bsn`. The same formula produces every other socket
rotation in `core.bsn`/`beam.bsn` (0° needs no rotation at all; 180° about Y
gives `Quat { x: 0, y: 1, z: 0, w: 0 }` since `sin(90°) = 1, cos(90°) = 0`).

The simplest case -- and the one `coupling_plate.bsn` uses -- is two sockets
that are already local mirror opposites (one at local +X with no rotation,
one at local -X rotated 180° about Y) on two modules that are *both* left at
identity rotation. Neither module needs any extra rotation for their world
normals to oppose, since their local and world spaces already coincide.

## Vehicle Anatomy

A vehicle `.bsn` (see `game/assets/vehicle/vehicle_module_testbed.bsn`,
`game/assets/vehicle/examples/two_plate_coupling.bsn`) is a root entity with
a `Children` list containing:

- One **module instance** per module placed in the vehicle:
  `LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/<module>.bsn" }`,
  tagged with whatever `Name` (the `#SomeTag` syntax) you want to reference
  it by later. This tag is entirely your choice -- it is not read anywhere
  in Rust except by name-matching against `LastBeaconVehicleConnection`
  entries in this same vehicle.
- One **connection** per pair of sockets to join:
  `LastBeaconVehicleConnection { module_a: "<tag>", socket_a: "<socket_name>", module_b: "<tag>", socket_b: "<socket_name>" }`.
  Order between `module_a`/`module_b` only matters for which module survives
  as the compound body's root when both sides are welds (`module_a`'s root
  survives; `module_b`'s gets absorbed into it) -- for a hinge, either side
  may be the wheel.

## Worked Example: `two_plate_coupling.bsn`

`game/assets/vehicle/modules/coupling_plate.bsn` is a minimal two-socket
module: a small flat cuboid (`half_size: (0.3, 0.05, 0.3)`) with sockets
`"a"` (local +X, no rotation) and `"b"` (local -X, rotated 180° about Y) --
both welds.

`game/assets/vehicle/examples/two_plate_coupling.bsn` places two instances
of it, tagged `PlateOne` and `PlateTwo`, and welds them together with a
single connection referencing `PlateOne`'s `"a"` socket and `PlateTwo`'s
`"b"` socket. Neither plate needs a rotation, since `"a"`/`"b"` are already
local mirror opposites (see "The Socket-Normal Convention" above). Once
resolved, `PlateTwo` is absorbed into `PlateOne`'s compound rigid body, and
their sockets coincide exactly -- verified by
`game/tests/vehicle_example_two_plate_coupling.rs`.

To build your own two-module vehicle: copy `two_plate_coupling.bsn`, change
the `asset_path`s to your own module(s), change the tags/socket names in the
`LastBeaconVehicleConnection` to match, and load it the same way
`vehicle_module_testbed.bsn` is loaded (`commands.spawn_bsn_asset("vehicle/your_vehicle.bsn")`).

## Diagnosing A Bad Connection

If a `LastBeaconVehicleConnection` names a module tag or socket name that
doesn't exist, `wire_last_beacon_vehicle_connections` logs a `warn!`
describing exactly what was wrong -- including every valid module tag in
the vehicle, or every valid socket name on the referenced module, so you can
immediately see the typo. That connection is marked with a
`LastBeaconVehicleConnectionFailed` component (instead of
`LastBeaconVehicleConnectionResolved`) and, with the `dev-tools` feature
enabled, a red sphere gizmo is drawn in-scene at the connection's
best-known position (the midpoint of both named modules, if both resolved
but a socket name didn't; whichever one module *did* resolve, if only one
did).

## See Also

- `docs/plans/vehicle-fixes/` -- the branch this guide's Phase 2 work landed
  on, including the Avian3D ground-contact-crash fix that was Phase 1.
- `docs/superpowers/specs/2026-09-15-vehicle-authoring-clarity-design.md` --
  the design this guide implements.
```

- [ ] **Step 2: Cross-link from `mod.rs`'s module doc comment**

In `game/src/shared/vehicle/mod.rs`, the module doc comment ends with (currently the paragraph starting "A module's `.bsn` is meant to be a *complete* asset definition..."). Add one new paragraph immediately after that paragraph's closing (right before the `mod connection;` line):

```rust
//!
//! See `docs/vehicle-authoring-guide.md` for a full walkthrough of building
//! a new module and wiring it into a vehicle, including a real worked
//! example (`game/assets/vehicle/modules/coupling_plate.bsn` +
//! `game/assets/vehicle/examples/two_plate_coupling.bsn`) and how to
//! diagnose a bad tag/socket reference.
```

- [ ] **Step 3: Verify the crate still builds and docs generate cleanly**

Run: `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`
Expected: PASS (may show the same pre-existing rustdoc warnings noted in `docs/plans/vehicle-fixes/tracker.md` -- unrelated broken intra-doc links already present before this work; no *new* warnings should appear from the paragraph added in Step 2, since it contains no `[...]` link syntax).

- [ ] **Step 4: Commit**

```bash
git add docs/vehicle-authoring-guide.md game/src/shared/vehicle/mod.rs
git commit -m "$(cat <<'EOF'
Add a vehicle authoring guide with a worked module/vehicle example

Files changed:
- docs/vehicle-authoring-guide.md
- game/src/shared/vehicle/mod.rs
EOF
)"
```

---

## Task 5: Update the Phase 2 plan/tracker docs and run full validation

**Files:**
- Modify: `docs/plans/vehicle-fixes/plan.md`
- Modify: `docs/plans/vehicle-fixes/tracker.md`

- [ ] **Step 1: Fill in Phase 2's scope in the tracker**

In `docs/plans/vehicle-fixes/tracker.md`, replace the entire "Phase 2: Vehicle definition tidy-ups (not yet scoped)" section (from `## Phase 2: Vehicle definition tidy-ups (not yet scoped)` through its `### Tasks` list) with:

```markdown
## Phase 2: Vehicle authoring clarity
**Status:** Done
**Goal:** Bad `.bsn` tag/socket references are loud (rich warnings, a
distinguishable `LastBeaconVehicleConnectionFailed` marker, a red in-scene
gizmo) instead of silent, and a standalone authoring guide with a real
worked example exists. See
`docs/superpowers/specs/2026-09-15-vehicle-authoring-clarity-design.md` and
`docs/superpowers/plans/2026-09-15-vehicle-authoring-clarity.md`.

### Tasks
- [x] Richer lookup-failure diagnostics + `LastBeaconVehicleConnectionFailed` (`connection.rs`)
  - Status: Done
  - Repository: `root`
- [x] Red gizmo marker for failed connections (`debug.rs`, `mod.rs`, `shared/mod.rs`)
  - Status: Done
  - Repository: `root`
- [x] Worked-example module/vehicle assets + regression test
  - Status: Done
  - Repository: `root`
- [x] `docs/vehicle-authoring-guide.md`
  - Status: Done
  - Repository: `root`

### Validation
- Game validation: `Passed (scripts/validate.cmd)`
- Engine validation: `N/A (no engine changes in Phase 2)`
- Documentation generation: `Done`
- User confirmation: Not required yet
```

Also update the tracker's `## Metadata` `Overall status` line from `In Progress` to `Done` (both phases complete), and its `Last updated` date if it has drifted from `2026-09-15`.

- [ ] **Step 2: Update the plan doc's status**

In `docs/plans/vehicle-fixes/plan.md`, change the `## Metadata` block's `Status: In Progress` line to `Status: Done`, and update its `Last updated` date if needed.

- [ ] **Step 3: Run full validation**

Run: `scripts\validate.cmd` (from the repo root, PowerShell or cmd)
Expected: PASS -- fmt check, clippy `-D warnings`, full test suite (including the 2 new `connection.rs` tests and the new `vehicle_example_two_plate_coupling` integration test), build, and doc generation all succeed. This will take several minutes (first full rebuild after source changes).

If clippy or fmt fail, fix the reported issue directly in the affected file and re-run before proceeding -- do not skip or suppress.

- [ ] **Step 4: Commit**

```bash
git add docs/plans/vehicle-fixes/plan.md docs/plans/vehicle-fixes/tracker.md
git commit -m "$(cat <<'EOF'
Mark vehicle-fixes Phase 2 (authoring clarity) done in the tracker

Files changed:
- docs/plans/vehicle-fixes/plan.md
- docs/plans/vehicle-fixes/tracker.md
EOF
)"
```

- [ ] **Step 5: Push**

```bash
git push
```

Expected: pushes to `origin/feature/vehicle-fixes` (already tracked from Phase 1 -- no `-u` needed).

---

## Self-Review Notes

- **Spec coverage:** Section 1 (richer diagnostics) → Task 1. Section 2 (Failed marker + gizmo) → Task 1 (marker) + Task 2 (gizmo). Section 3 (guide + worked example) → Task 3 (assets + test) + Task 4 (guide). Tracker/plan doc updates → Task 5. All three spec sections have a task; nothing in the spec is unaddressed.
- **Placeholder scan:** No TBD/TODO; every step has complete code or an exact command with expected output.
- **Type consistency:** `LastBeaconVehicleConnectionFailed { reason: String, marker_position: Option<Vec3> }` is defined once in Task 1 Step 3 and used with those exact field names in every later step (Task 1 tests, Task 2's gizmo function). `collect_vehicle_module_tags`/`collect_module_socket_names`/`format_name_list`/`describe_socket_lookup_error` are defined once (Task 1 Step 4) with signatures matching every call site introduced in Task 1 Step 5.
