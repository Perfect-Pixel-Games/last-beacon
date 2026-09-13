# Vehicle Module Foundations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundational Besiege-style modular-vehicle attachment system: `.bsn`-authored modules (Core, Beam, Plate, Wheel) that attach to each other through Avian3D physics joints to form a larger physically simulated structure, proven end-to-end by a test "wagon" vehicle.

**Architecture:** Avian3D is installed once, centrally, by the Foundation engine (`foundation-runtime-library`). Each attached module is its own `RigidBody::Dynamic` entity; modules are wired together by separate joint entities (`FixedJoint` for rigid welds, `RevoluteJoint` for free-spinning wheels), not by Bevy's parent/child transform hierarchy, so one module can hold joints to more than one other module at once. Everything a module author needs to write lives in a `.bsn` file; Rust code only supplies the reactive systems that turn plain authored data into real Bevy/Avian constructs (this project's `.bsn` grammar can construct plain structs/tuple-structs/enum-variants via reflection, but can never call a function like `Collider::cuboid(...)` directly).

**Tech Stack:** Bevy 0.19, Avian3D 0.7 (new dependency), this project's own dynamic-BSN scene format, Rust.

**Reference:** Full design rationale lives in `docs/superpowers/specs/2026-09-13-vehicle-module-foundations-design.md`. Read it before starting if anything below seems under-explained.

**A note on the physics API:** This plan pins exact Avian3D 0.7 method names and signatures verified against its published docs (`RigidBody`, `Collider::cuboid`/`Collider::cylinder`, `Mass`, `AngularVelocity`, `FixedJoint`, `RevoluteJoint`, all under `avian3d::prelude::*`). If a `cargo check` in any task below reports a slightly different method name or signature than what's written here, trust the compiler error and `cargo doc -p avian3d --open` over this document — the crate is actively developed and point-release API drift is possible. Every task below includes an explicit "run it and check the output" step for exactly this reason.

---

## File Structure

- `engine/Cargo.toml` — **modify**: add `avian3d` to `[workspace.dependencies]`.
- `engine/crates/foundation-runtime-library/Cargo.toml` — **modify**: add `avian3d.workspace = true`.
- `engine/crates/foundation-runtime-library/src/physics.rs` — **create**: `FoundationPhysicsPlugin`, installs `avian3d::PhysicsPlugins::default()`.
- `engine/crates/foundation-runtime-library/src/lib.rs` — **modify**: register the new module/plugin, add prelude re-export.
- `game/Cargo.toml` — **modify**: add `avian3d` dependency.
- `game/src/shared/vehicle/mod.rs` — **create**: every reflected vehicle component/enum (`LastBeaconVehicleModuleBody`, `LastBeaconVehicleWheelModuleBody`, `LastBeaconVehicleModuleSocket`, `LastBeaconVehicleModuleInstance`, `LastBeaconVehicleConnection`, `LastBeaconVehicleJointKind`) and `LastBeaconVehiclePlugin`.
- `game/src/shared/vehicle/module_body.rs` — **create**: systems that turn `LastBeaconVehicleModuleBody`/`LastBeaconVehicleWheelModuleBody` into real mesh/collider/rigid-body/mass.
- `game/src/shared/vehicle/instance.rs` — **create**: systems that load a module instance's referenced `.bsn` and apply it onto the instance entity.
- `game/src/shared/vehicle/connection.rs` — **create**: the system that resolves named module/socket pairs and spawns the joint.
- `game/src/shared/mod.rs` — **modify**: register `vehicle` module and `LastBeaconVehiclePlugin`.
- `game/src/world/mod.rs` — **modify**: spawn the test wagon into the landscape testbed scene.
- `game/assets/vehicle/modules/core.bsn` — **create**.
- `game/assets/vehicle/modules/beam.bsn` — **create**.
- `game/assets/vehicle/modules/plate.bsn` — **create**.
- `game/assets/vehicle/modules/wheel.bsn` — **create**.
- `game/assets/vehicle/vehicle_module_testbed.bsn` — **create**: the test wagon.

---

### Task 1: Add Avian3D to the engine workspace and install it from Foundation

**Files:**
- Modify: `engine/Cargo.toml`
- Modify: `engine/crates/foundation-runtime-library/Cargo.toml`
- Create: `engine/crates/foundation-runtime-library/src/physics.rs`
- Modify: `engine/crates/foundation-runtime-library/src/lib.rs`

- [ ] **Step 1: Add `avian3d` to the engine workspace's shared dependencies**

In `engine/Cargo.toml`, `[workspace.dependencies]` is alphabetically ordered. Add `avian3d` as the very first entry (before `bevy`):

```toml
[workspace.dependencies]
avian3d = "0.7"
bevy = { version = "0.19.0", features = [
    "file_watcher",
    "reflect_documentation",
    "serialize",
] }
```

- [ ] **Step 2: Add `avian3d` as a dependency of `foundation-runtime-library`**

In `engine/crates/foundation-runtime-library/Cargo.toml`, `[dependencies]` is alphabetically ordered. Add it before `bevy.workspace = true`:

```toml
[dependencies]
avian3d.workspace = true
bevy.workspace = true
```

- [ ] **Step 3: Write `physics.rs`**

Create `engine/crates/foundation-runtime-library/src/physics.rs`:

```rust
//! Installs Avian3D physics simulation for Foundation games.

use avian3d::prelude::*;
use bevy::prelude::*;

/// Installs Avian3D's physics simulation.
///
/// Centralized here so every Foundation game gets the same physics backend
/// without each game crate needing to know or choose which physics engine is
/// in use.
#[derive(Default)]
pub struct FoundationPhysicsPlugin;

impl Plugin for FoundationPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
    }
}
```

- [ ] **Step 4: Register the plugin in `FoundationPlugin` and export it from the prelude**

In `engine/crates/foundation-runtime-library/src/lib.rs`, add the module declaration. Modules are declared alphabetically; insert `pub mod physics;` between `pub mod perf_overlay;` (a `#[cfg(feature = "dev-tools")]` block) and `pub mod scene_stack;`:

```rust
#[cfg(feature = "dev-tools")]
pub mod perf_overlay;
pub mod physics;
pub mod scene_stack;
```

Then add `physics::FoundationPhysicsPlugin` to the sub-plugin tuple inside `FoundationPlugin::build`:

```rust
        app.add_plugins((
            scene_stack::FoundationSceneStackPlugin,
            splash_screen::FoundationSplashScreenPlugin,
            menu::FoundationMenuPlugin,
            credits::FoundationCreditsPlugin,
            ui_theme::FoundationUiThemePlugin,
            window_focus::FoundationWindowFocusPlugin,
            physics::FoundationPhysicsPlugin,
        ))
```

Finally, add the re-export inside `pub mod prelude { ... }`, between the `perf_overlay` block and the `scene_stack` re-export:

```rust
    #[cfg(feature = "dev-tools")]
    pub use crate::perf_overlay::{FoundationPerfOverlayPlugin, FoundationPerfOverlayState};
    pub use crate::physics::FoundationPhysicsPlugin;
    pub use crate::scene_stack::{
```

- [ ] **Step 5: Build the engine workspace to confirm it compiles**

Run: `cd E:/GameDev/last-beacon/engine && cargo check -p foundation-runtime-library`
Expected: compiles with no errors. If `avian3d::prelude::PhysicsPlugins` doesn't resolve, check `cargo doc -p avian3d --open` for the correct path — it should be re-exported from the crate root too as `avian3d::PhysicsPlugins`.

- [ ] **Step 6: Commit**

```bash
git add engine/Cargo.toml engine/crates/foundation-runtime-library/Cargo.toml engine/crates/foundation-runtime-library/src/physics.rs engine/crates/foundation-runtime-library/src/lib.rs
git commit -m "Install Avian3D physics from the Foundation engine"
```

---

### Task 2: Add Avian3D to the game crate and scaffold the vehicle module

**Files:**
- Modify: `game/Cargo.toml`
- Create: `game/src/shared/vehicle/mod.rs`
- Modify: `game/src/shared/mod.rs`

- [ ] **Step 1: Add `avian3d` to the game crate**

In `game/Cargo.toml`, `[dependencies]` is alphabetically ordered. Add it before `bevy = { ... }`:

```toml
[dependencies]
avian3d = "0.7"
bevy = { version = "0.19.0", features = [
    "file_watcher",
    "reflect_documentation",
    "serialize",
] }
```

- [ ] **Step 2: Create an empty `LastBeaconVehiclePlugin` to scaffold the module**

Create `game/src/shared/vehicle/mod.rs`:

```rust
//! Foundational modular-vehicle attachment system.
//!
//! Modules (`.bsn`-authored blocks like a core, a beam, a plate, a wheel)
//! attach to each other via Avian3D physics joints to form a larger,
//! physically simulated structure -- the basis every playable vehicle will
//! eventually be built from. This module owns the attachment mechanism
//! itself; concrete gameplay modules (powered wheels, thrusters, weapons)
//! are future work built on top of it.

use bevy::prelude::*;

/// Installs Last Beacon's modular-vehicle attachment system.
#[derive(Default)]
pub struct LastBeaconVehiclePlugin;

impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, _app: &mut App) {}
}
```

- [ ] **Step 3: Register the module and plugin from `shared/mod.rs`**

In `game/src/shared/mod.rs`, add the module declaration after the existing `use` statements:

```rust
use crate::scenes::{BEACON_SCENE, GAMEPLAY_LEVEL_SCENE};

pub mod vehicle;
```

Then add `vehicle::LastBeaconVehiclePlugin` to `LastBeaconSharedGameplayPlugin::build`, right at the top before the existing `init_resource` calls:

```rust
impl Plugin for LastBeaconSharedGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(vehicle::LastBeaconVehiclePlugin)
            .init_resource::<LastBeaconSharedGameplayState>()
```

- [ ] **Step 4: Build the game crate to confirm it compiles**

Run: `cd E:/GameDev/last-beacon/game && cargo check`
Expected: compiles with no errors.

- [ ] **Step 5: Commit**

```bash
git add game/Cargo.toml game/src/shared/vehicle/mod.rs game/src/shared/mod.rs
git commit -m "Scaffold the vehicle module attachment system"
```

---

### Task 3: Box module body component and materialization system

**Files:**
- Modify: `game/src/shared/vehicle/mod.rs`
- Create: `game/src/shared/vehicle/module_body.rs`

- [ ] **Step 1: Add the `LastBeaconVehicleModuleBody` component**

In `game/src/shared/vehicle/mod.rs`, add below the existing doc comment (before the plugin):

```rust
mod module_body;

pub use module_body::materialize_last_beacon_vehicle_module_bodies;

/// Authored on a box-shaped module's root entity (Core, Beam, Plate). A
/// reactive system turns this into the real mesh, material, rigid body,
/// collider, and mass -- this project's `.bsn` grammar can construct plain
/// structs like this one via reflection, but can never call a constructor
/// function like `Collider::cuboid(...)` directly.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleBody {
    /// Full box width along local X, in meters.
    pub size_x: f32,
    /// Full box height along local Y, in meters.
    pub size_y: f32,
    /// Full box depth along local Z, in meters.
    pub size_z: f32,
    /// Rigid body mass in kilograms.
    pub mass: f32,
    /// Named color, resolved by `module_body::last_beacon_vehicle_module_color`.
    pub color: String,
}

impl Default for LastBeaconVehicleModuleBody {
    fn default() -> Self {
        Self {
            size_x: 1.0,
            size_y: 1.0,
            size_z: 1.0,
            mass: 10.0,
            color: "steel_blue".to_string(),
        }
    }
}
```

Then register it in `LastBeaconVehiclePlugin::build`:

```rust
impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconVehicleModuleBody>()
            .add_systems(Update, materialize_last_beacon_vehicle_module_bodies);
    }
}
```

- [ ] **Step 2: Write the failing test for the materialization system**

Create `game/src/shared/vehicle/module_body.rs`:

```rust
//! Turns authored module-body data into real Bevy meshes/materials and Avian
//! rigid bodies/colliders.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::LastBeaconVehicleModuleBody;

/// Builds the mesh, material, rigid body, collider, and mass for every
/// newly-authored [`LastBeaconVehicleModuleBody`].
pub fn materialize_last_beacon_vehicle_module_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    module_bodies: Query<(Entity, &LastBeaconVehicleModuleBody), Added<LastBeaconVehicleModuleBody>>,
) {
    for (module_entity, module_body) in &module_bodies {
        let mesh = meshes.add(Cuboid::new(
            module_body.size_x,
            module_body.size_y,
            module_body.size_z,
        ));
        let material = materials.add(StandardMaterial {
            base_color: last_beacon_vehicle_module_color(&module_body.color),
            ..default()
        });

        commands.entity(module_entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RigidBody::Dynamic,
            Collider::cuboid(module_body.size_x, module_body.size_y, module_body.size_z),
            Mass(module_body.mass),
        ));
    }
}

/// Resolves a module's authored color name to a concrete color, warning and
/// falling back to `steel_blue` for an unrecognized name.
pub fn last_beacon_vehicle_module_color(color_name: &str) -> Color {
    match color_name.trim().to_ascii_lowercase().as_str() {
        "steel_blue" => Color::srgb(0.29, 0.42, 0.55),
        "charcoal" => Color::srgb(0.16, 0.16, 0.18),
        "olive" => Color::srgb(0.36, 0.38, 0.21),
        unknown_color => {
            warn!("Unknown vehicle module color `{unknown_color}`; using steel_blue");
            Color::srgb(0.29, 0.42, 0.55)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.init_asset::<StandardMaterial>();
        app.add_systems(Update, materialize_last_beacon_vehicle_module_bodies);
        app
    }

    #[test]
    fn materializing_a_module_body_adds_a_dynamic_rigid_body_and_collider() {
        let mut app = test_app();
        let module_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleModuleBody {
                size_x: 2.0,
                size_y: 1.0,
                size_z: 1.0,
                mass: 15.0,
                color: "olive".to_string(),
            })
            .id();

        app.update();

        assert!(matches!(
            app.world().get::<RigidBody>(module_entity),
            Some(RigidBody::Dynamic)
        ));
        assert!(app.world().get::<Collider>(module_entity).is_some());
        assert!(app.world().get::<Mass>(module_entity).is_some());
        assert!(app.world().get::<Mesh3d>(module_entity).is_some());
        assert!(app.world().get::<MeshMaterial3d<StandardMaterial>>(module_entity).is_some());
    }

    #[test]
    fn unknown_color_falls_back_to_steel_blue() {
        assert_eq!(
            last_beacon_vehicle_module_color("not-a-real-color"),
            Color::srgb(0.29, 0.42, 0.55)
        );
    }
}
```

Note: `bevy::asset::AssetPlugin` needs importing at the top of the test module if it isn't already brought in by `bevy::prelude::*` — it is not, since existing code (`bsn_assets.rs`) imports it explicitly. Add `use bevy::asset::AssetPlugin;` inside the `mod tests` block if the next step's compile fails on it.

- [ ] **Step 3: Run the tests to see them fail (the plugin/module doesn't compile yet without the mod.rs wiring from Step 1)**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon module_body:: -- --nocapture`
Expected: compiles and passes, since Step 1 already wired the component and `mod module_body;`/`pub use` into `mod.rs` before this file existed. If it fails to compile because `AssetPlugin` is unresolved, add `use bevy::asset::AssetPlugin;` to the test module's imports as noted above.

- [ ] **Step 4: Run the tests again to confirm they pass**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon module_body::`
Expected: both tests pass.

- [ ] **Step 5: Commit**

```bash
git add game/src/shared/vehicle/mod.rs game/src/shared/vehicle/module_body.rs
git commit -m "Materialize box-shaped vehicle module bodies into rigid bodies"
```

---

### Task 4: Wheel module body component and materialization system

**Files:**
- Modify: `game/src/shared/vehicle/mod.rs`
- Modify: `game/src/shared/vehicle/module_body.rs`

- [ ] **Step 1: Add the `LastBeaconVehicleWheelModuleBody` component**

In `game/src/shared/vehicle/mod.rs`, add next to `LastBeaconVehicleModuleBody`:

```rust
/// Authored on a wheel module's root entity. Separate from
/// [`LastBeaconVehicleModuleBody`] because a wheel is a cylinder, not a box,
/// and needs its own collider/mesh construction.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleWheelModuleBody {
    /// Wheel radius in meters.
    pub radius: f32,
    /// Full wheel width (the cylinder's height, along local Y) in meters.
    pub width: f32,
    /// Rigid body mass in kilograms.
    pub mass: f32,
    /// Named color, resolved by `module_body::last_beacon_vehicle_module_color`.
    pub color: String,
}

impl Default for LastBeaconVehicleWheelModuleBody {
    fn default() -> Self {
        Self {
            radius: 0.5,
            width: 0.3,
            mass: 5.0,
            color: "charcoal".to_string(),
        }
    }
}
```

Update the `pub use module_body::...` line to also export the new system, and register the new type/system in `LastBeaconVehiclePlugin`:

```rust
pub use module_body::{
    materialize_last_beacon_vehicle_module_bodies, materialize_last_beacon_vehicle_wheel_module_bodies,
};
```

```rust
impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .add_systems(
                Update,
                (
                    materialize_last_beacon_vehicle_module_bodies,
                    materialize_last_beacon_vehicle_wheel_module_bodies,
                ),
            );
    }
}
```

- [ ] **Step 2: Write the failing test for the wheel materialization system**

In `game/src/shared/vehicle/module_body.rs`, add the system above `last_beacon_vehicle_module_color`:

```rust
use super::{LastBeaconVehicleModuleBody, LastBeaconVehicleWheelModuleBody};

/// Builds the mesh, material, rigid body, collider, and mass for every
/// newly-authored [`LastBeaconVehicleWheelModuleBody`].
pub fn materialize_last_beacon_vehicle_wheel_module_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wheel_bodies: Query<
        (Entity, &LastBeaconVehicleWheelModuleBody),
        Added<LastBeaconVehicleWheelModuleBody>,
    >,
) {
    for (wheel_entity, wheel_body) in &wheel_bodies {
        let mesh = meshes.add(Cylinder::new(wheel_body.radius, wheel_body.width));
        let material = materials.add(StandardMaterial {
            base_color: last_beacon_vehicle_module_color(&wheel_body.color),
            ..default()
        });

        commands.entity(wheel_entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RigidBody::Dynamic,
            Collider::cylinder(wheel_body.radius, wheel_body.width),
            Mass(wheel_body.mass),
        ));
    }
}
```

Add the test alongside the existing ones in `mod tests`:

```rust
    #[test]
    fn materializing_a_wheel_body_adds_a_dynamic_rigid_body_and_collider() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.init_asset::<StandardMaterial>();
        app.add_systems(Update, materialize_last_beacon_vehicle_wheel_module_bodies);

        let wheel_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleWheelModuleBody {
                radius: 0.6,
                width: 0.4,
                mass: 6.0,
                color: "charcoal".to_string(),
            })
            .id();

        app.update();

        assert!(matches!(
            app.world().get::<RigidBody>(wheel_entity),
            Some(RigidBody::Dynamic)
        ));
        assert!(app.world().get::<Collider>(wheel_entity).is_some());
        assert!(app.world().get::<Mass>(wheel_entity).is_some());
        assert!(app.world().get::<Mesh3d>(wheel_entity).is_some());
    }
```

- [ ] **Step 3: Run the test to see it compile and pass**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon module_body::`
Expected: all three tests in this file pass.

- [ ] **Step 4: Commit**

```bash
git add game/src/shared/vehicle/mod.rs game/src/shared/vehicle/module_body.rs
git commit -m "Materialize wheel vehicle module bodies into cylinder rigid bodies"
```

---

### Task 5: Module socket component

**Files:**
- Modify: `game/src/shared/vehicle/mod.rs`

- [ ] **Step 1: Add `LastBeaconVehicleModuleSocket`**

This component has no associated system -- it's pure authored data. A socket entity is a child of a module root, carrying this marker plus a plain `Transform` for its local anchor offset (which needs no marker/system workaround since `Transform`/`Vec3` are plain public-field structs the BSN grammar can already construct directly).

In `game/src/shared/vehicle/mod.rs`, add:

```rust
/// Marks a child entity of a module's root as a named attachment point.
///
/// The entity's own `Transform.translation` is that socket's local anchor
/// offset relative to the module's rigid body -- authored directly as a
/// plain `Transform` in `.bsn`, no marker/system workaround needed since
/// `Transform` is a plain public-field struct.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleSocket {
    /// Name used by a [`LastBeaconVehicleConnection`] to reference this socket.
    pub socket_name: String,
}
```

Register it in `LastBeaconVehiclePlugin::build`:

```rust
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .register_type::<LastBeaconVehicleModuleSocket>()
```

- [ ] **Step 2: Build to confirm it compiles**

Run: `cd E:/GameDev/last-beacon/game && cargo check`
Expected: compiles with no errors.

- [ ] **Step 3: Commit**

```bash
git add game/src/shared/vehicle/mod.rs
git commit -m "Add the vehicle module socket component"
```

---

### Task 6: Module instance loading

**Files:**
- Modify: `game/src/shared/vehicle/mod.rs`
- Create: `game/src/shared/vehicle/instance.rs`

This mirrors `game/src/ui_widgets.rs`'s existing `LastBeaconBsnWidget`/`LastBeaconBsnWidgetPending` load-and-apply pattern (itself mirroring `foundation-runtime-library`'s `bsn_assets.rs`), applying a module's `.bsn` content directly onto the entity that references it, so that entity becomes the module's rigid body while keeping the `Name`/`Transform` the *vehicle* `.bsn` already authored on it.

- [ ] **Step 1: Add `LastBeaconVehicleModuleInstance`**

In `game/src/shared/vehicle/mod.rs`, add:

```rust
mod instance;

pub use instance::{
    apply_pending_last_beacon_vehicle_module_instances, queue_last_beacon_vehicle_module_instances,
};

/// Authored in a *vehicle* `.bsn`, one per module placed in that vehicle.
///
/// A system loads the referenced module `.bsn` and applies its content onto
/// this same entity, so this entity becomes that module's rigid body once
/// resolved -- keeping whatever `Name`/`Transform` the vehicle `.bsn` already
/// authored here for placement and for [`LastBeaconVehicleConnection`] to
/// find it by name.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleInstance {
    /// Asset-relative path to the module's `.bsn` file, e.g.
    /// `"vehicle/modules/core.bsn"`.
    pub asset_path: String,
}
```

Register the type and the two new systems in `LastBeaconVehiclePlugin::build`:

```rust
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .register_type::<LastBeaconVehicleModuleSocket>()
            .register_type::<LastBeaconVehicleModuleInstance>()
            .add_systems(
                Update,
                (
                    materialize_last_beacon_vehicle_module_bodies,
                    materialize_last_beacon_vehicle_wheel_module_bodies,
                    queue_last_beacon_vehicle_module_instances,
                    apply_pending_last_beacon_vehicle_module_instances,
                )
                    .chain(),
            );
```

- [ ] **Step 2: Write `instance.rs` with its load/apply systems and tests**

Create `game/src/shared/vehicle/instance.rs`:

```rust
//! Loads a [`LastBeaconVehicleModuleInstance`]'s referenced `.bsn` module
//! definition and applies it onto the same entity, mirroring
//! `game/src/ui_widgets.rs`'s `LastBeaconBsnWidget` load/apply pattern.

use std::sync::Arc;

use bevy::{
    asset::{AssetServer, Handle},
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
};

use super::LastBeaconVehicleModuleInstance;

/// Tracks a module instance whose `.bsn` asset is still loading/resolving.
///
/// `pub(super)` so [`super::connection`]'s joint-wiring system can check
/// whether a referenced module instance has finished loading yet.
#[derive(Clone, Debug, Component)]
pub(super) struct LastBeaconVehicleModuleInstancePending {
    pub(super) scene_handle: Handle<ScenePatch>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Component)]
struct LastBeaconVehicleModuleInstanceFailed {
    reason: String,
}

/// Starts loading a newly-authored module instance's `.bsn` module definition.
pub fn queue_last_beacon_vehicle_module_instances(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    module_instances: Query<
        (Entity, &LastBeaconVehicleModuleInstance),
        Added<LastBeaconVehicleModuleInstance>,
    >,
) {
    for (instance_entity, module_instance) in &module_instances {
        if module_instance.asset_path.is_empty() {
            warn!("LastBeaconVehicleModuleInstance on {instance_entity:?} has an empty asset path.");
            continue;
        }

        let scene_handle = asset_server.load(module_instance.asset_path.clone());
        commands
            .entity(instance_entity)
            .insert(LastBeaconVehicleModuleInstancePending { scene_handle });
    }
}

/// Applies each pending module instance's loaded `.bsn` content onto its own
/// entity once the asset resolves, turning it into that module's rigid body.
pub fn apply_pending_last_beacon_vehicle_module_instances(world: &mut World) {
    let pending_instances = {
        let mut pending_query = world.query::<(Entity, &LastBeaconVehicleModuleInstancePending)>();
        pending_query
            .iter(world)
            .map(|(instance_entity, pending)| (instance_entity, pending.scene_handle.clone()))
            .collect::<Vec<_>>()
    };

    for (instance_entity, scene_handle) in pending_instances {
        let scene_patch_id = scene_handle.id();
        let resolve_result = world.resource_scope(
            |world, mut scene_patches: Mut<Assets<ScenePatch>>| -> Result<bool, String> {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return Ok(false);
                };

                if scene_patch.resolved.is_some() {
                    return Ok(true);
                }

                let scene = scene_patches
                    .get_mut(scene_patch_id)
                    .and_then(|mut scene_patch| scene_patch.scene.take());
                let Some(scene) = scene else {
                    return Ok(false);
                };

                let asset_server = world.resource::<AssetServer>();
                let resolved_scene_root =
                    ResolvedSceneRoot::resolve(scene, asset_server, &scene_patches)
                        .map_err(|resolve_error| resolve_error.to_string())?;
                if let Some(mut scene_patch) = scene_patches.get_mut(scene_patch_id) {
                    scene_patch.resolved = Some(Arc::new(resolved_scene_root));
                }
                Ok(true)
            },
        );

        let scene_is_ready = match resolve_result {
            Ok(scene_is_ready) => scene_is_ready,
            Err(resolve_error) => {
                let failure_reason = format!("Failed to resolve vehicle module instance: {resolve_error}");
                mark_module_instance_failed(world, instance_entity, failure_reason);
                continue;
            }
        };

        if !scene_is_ready {
            continue;
        }

        let apply_result = world.resource_scope(
            |world, scene_patches: Mut<Assets<ScenePatch>>| -> Result<(), String> {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return Err("ScenePatch asset disappeared before module apply".to_string());
                };
                let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) else {
                    return Err("Module instance entity disappeared before apply".to_string());
                };

                scene_patch
                    .apply(&mut instance_entity_mut)
                    .map_err(|apply_error| apply_error.to_string())
            },
        );

        match apply_result {
            Ok(()) => {
                if let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) {
                    instance_entity_mut.remove::<LastBeaconVehicleModuleInstancePending>();
                }
            }
            Err(apply_error) => {
                let failure_reason =
                    format!("Failed to apply vehicle module instance to {instance_entity:?}: {apply_error}");
                mark_module_instance_failed(world, instance_entity, failure_reason);
            }
        }
    }
}

fn mark_module_instance_failed(world: &mut World, instance_entity: Entity, failure_reason: String) {
    error!("{failure_reason}");
    if let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) {
        instance_entity_mut.remove::<LastBeaconVehicleModuleInstancePending>();
        instance_entity_mut.insert(LastBeaconVehicleModuleInstanceFailed {
            reason: failure_reason,
        });
    }
}

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use std::io::Write;

    #[derive(Clone, Debug, Default, Component, Reflect)]
    #[reflect(Component, Default)]
    struct FakeModuleMarker;

    fn test_app_with_asset_root(asset_root: &std::path::Path) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            ..default()
        });
        app.register_type::<FakeModuleMarker>();
        app.add_systems(
            Update,
            (
                queue_last_beacon_vehicle_module_instances,
                apply_pending_last_beacon_vehicle_module_instances,
            )
                .chain(),
        );
        app
    }

    #[test]
    fn queuing_an_instance_with_an_empty_asset_path_does_not_panic() {
        let temp_dir = std::env::temp_dir().join("last_beacon_vehicle_instance_test_empty");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let mut app = test_app_with_asset_root(&temp_dir);
        let instance_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleModuleInstance {
                asset_path: String::new(),
            })
            .id();

        app.update();

        assert!(app
            .world()
            .get::<LastBeaconVehicleModuleInstancePending>(instance_entity)
            .is_none());
    }

    #[test]
    fn resolved_module_content_applies_onto_the_instance_entity() {
        let temp_dir = std::env::temp_dir().join("last_beacon_vehicle_instance_test_fixture");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let fixture_path = temp_dir.join("fake_module.bsn");
        let mut fixture_file = std::fs::File::create(&fixture_path).unwrap();
        write!(fixture_file, "last_beacon::shared::vehicle::instance::tests::FakeModuleMarker").unwrap();
        drop(fixture_file);

        let mut app = test_app_with_asset_root(&temp_dir);
        let instance_entity = app
            .world_mut()
            .spawn((
                Name::new("TestInstance"),
                LastBeaconVehicleModuleInstance {
                    asset_path: "fake_module.bsn".to_string(),
                },
            ))
            .id();

        for _ in 0..5 {
            app.update();
        }

        assert!(app
            .world()
            .get::<LastBeaconVehicleModuleInstancePending>(instance_entity)
            .is_none());
        assert!(app.world().get::<FakeModuleMarker>(instance_entity).is_some());
        // The vehicle-authored Name must survive the module apply, since
        // LastBeaconVehicleConnection resolves modules by this name.
        assert_eq!(
            app.world().get::<Name>(instance_entity).unwrap().as_str(),
            "TestInstance"
        );

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
```

This uses a real temp-directory asset root loaded through Bevy's normal file-backed `AssetServer`, matching how `.bsn` assets are loaded everywhere else in this codebase (no in-memory scene shortcut for real file loads is used elsewhere either, since dynamic BSN files are parsed from disk by `DynamicBsnLoader`). The `FakeModuleMarker` type is written into the fixture `.bsn` by the same path the type registry will resolve it under (`last_beacon::shared::vehicle::instance::tests::FakeModuleMarker`, i.e. its real Rust module path since it's declared inside this test module).

- [ ] **Step 3: Run the tests and see them fail first (before double-checking the fixture path resolves)**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon instance::`
Expected: `queuing_an_instance_with_an_empty_asset_path_does_not_panic` passes immediately. `resolved_module_content_applies_onto_the_instance_entity` may fail if the reflected type path guess is wrong -- if it fails with a "type not found" or similar resolution error from the dynamic BSN loader, run `cargo doc -p last-beacon --open`, find `FakeModuleMarker`'s actual documented path, and correct the `write!` line to match exactly.

- [ ] **Step 4: Fix the type path if needed, then confirm both tests pass**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon instance::`
Expected: both tests pass.

- [ ] **Step 5: Commit**

```bash
git add game/src/shared/vehicle/mod.rs game/src/shared/vehicle/instance.rs
git commit -m "Load and apply vehicle module instances from their .bsn definitions"
```

---

### Task 7: Connection component and joint-wiring system

**Files:**
- Modify: `game/src/shared/vehicle/mod.rs`
- Create: `game/src/shared/vehicle/connection.rs`

- [ ] **Step 1: Add `LastBeaconVehicleJointKind` and `LastBeaconVehicleConnection`**

In `game/src/shared/vehicle/mod.rs`, add:

```rust
mod connection;

pub use connection::wire_last_beacon_vehicle_connections;

/// Which kind of Avian3D joint a [`LastBeaconVehicleConnection`] spawns.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect)]
#[reflect(Default)]
pub enum LastBeaconVehicleJointKind {
    /// A rigid weld -- zero relative motion between the two sockets. The
    /// default for ordinary structural connections.
    #[default]
    Fixed,
    /// A free-spinning hinge, for wheels. Always rotates about the *wheel*
    /// module's own local Y axis, matching `Collider::cylinder`'s natural
    /// rotational symmetry axis -- see `connection::LAST_BEACON_VEHICLE_HINGE_AXIS`.
    Hinge,
}

/// A sibling entity under a vehicle root, naming two module instances (by
/// `Name`) and a socket on each (by `socket_name`) to join together.
///
/// [`wire_last_beacon_vehicle_connections`] resolves this once both named
/// module instances have finished loading and spawns the corresponding
/// Avian3D joint entity.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleConnection {
    /// `Name` of the first module instance in this vehicle.
    pub module_a: String,
    /// Socket name on `module_a` to anchor this joint to.
    pub socket_a: String,
    /// `Name` of the second module instance in this vehicle.
    pub module_b: String,
    /// Socket name on `module_b` to anchor this joint to.
    pub socket_b: String,
    /// Which joint type to spawn.
    pub joint_kind: LastBeaconVehicleJointKind,
}
```

Register both types and the new system in `LastBeaconVehiclePlugin::build`:

```rust
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .register_type::<LastBeaconVehicleModuleSocket>()
            .register_type::<LastBeaconVehicleModuleInstance>()
            .register_type::<LastBeaconVehicleConnection>()
            .register_type::<LastBeaconVehicleJointKind>()
            .add_systems(
                Update,
                (
                    materialize_last_beacon_vehicle_module_bodies,
                    materialize_last_beacon_vehicle_wheel_module_bodies,
                    queue_last_beacon_vehicle_module_instances,
                    apply_pending_last_beacon_vehicle_module_instances,
                    wire_last_beacon_vehicle_connections,
                )
                    .chain(),
            );
```

- [ ] **Step 2: Write `connection.rs` with the wiring system**

Create `game/src/shared/vehicle/connection.rs`:

```rust
//! Resolves each [`LastBeaconVehicleConnection`]'s named module/socket pair
//! and spawns the corresponding Avian3D joint entity.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::instance::LastBeaconVehicleModuleInstancePending;
use super::{
    LastBeaconVehicleConnection, LastBeaconVehicleJointKind, LastBeaconVehicleModuleInstance,
    LastBeaconVehicleModuleSocket,
};

/// Marks a connection whose joint has already been spawned (or permanently
/// failed to resolve), so it is not processed again every frame.
#[derive(Clone, Copy, Debug, Component)]
struct LastBeaconVehicleConnectionResolved;

/// The axis every hinge joint spins around, in the *wheel* module's own
/// local space. Matches `Collider::cylinder`'s natural rotational symmetry
/// axis (its shape is defined with its height running along local Y), so a
/// wheel module never needs its own rotation authored just to make its spin
/// axis line up with this convention.
const LAST_BEACON_VEHICLE_HINGE_AXIS: Vec3 = Vec3::Y;

/// Resolves every unresolved [`LastBeaconVehicleConnection`] and spawns its
/// joint once both named module instances have finished loading.
pub fn wire_last_beacon_vehicle_connections(
    mut commands: Commands,
    connections: Query<
        (Entity, &LastBeaconVehicleConnection, &ChildOf),
        Without<LastBeaconVehicleConnectionResolved>,
    >,
    module_instances: Query<
        (Entity, &Name, Option<&LastBeaconVehicleModuleInstancePending>),
        With<LastBeaconVehicleModuleInstance>,
    >,
    children_query: Query<&Children>,
    sockets: Query<(&LastBeaconVehicleModuleSocket, &Transform)>,
) {
    for (connection_entity, connection, parent_link) in &connections {
        let Ok(sibling_entities) = children_query.get(parent_link.parent()) else {
            continue;
        };

        let module_a = find_named_module_instance(sibling_entities, &module_instances, &connection.module_a);
        let module_b = find_named_module_instance(sibling_entities, &module_instances, &connection.module_b);

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

        // Wait for both referenced modules to finish loading their own
        // `.bsn` module definition before their sockets exist to search.
        if module_a.is_pending || module_b.is_pending {
            continue;
        }

        let Some(socket_a) = find_named_socket(module_a.entity, &children_query, &sockets, &connection.socket_a)
        else {
            warn!(
                "LastBeaconVehicleConnection on {connection_entity:?} names socket `{}` that does not exist on module `{}`; skipping.",
                connection.socket_a, connection.module_a
            );
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionResolved);
            continue;
        };
        let Some(socket_b) = find_named_socket(module_b.entity, &children_query, &sockets, &connection.socket_b)
        else {
            warn!(
                "LastBeaconVehicleConnection on {connection_entity:?} names socket `{}` that does not exist on module `{}`; skipping.",
                connection.socket_b, connection.module_b
            );
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionResolved);
            continue;
        };

        match connection.joint_kind {
            LastBeaconVehicleJointKind::Fixed => {
                commands.spawn(
                    FixedJoint::new(module_a.entity, module_b.entity)
                        .with_local_anchor1(socket_a)
                        .with_local_anchor2(socket_b),
                );
            }
            LastBeaconVehicleJointKind::Hinge => {
                commands.spawn(
                    RevoluteJoint::new(module_a.entity, module_b.entity)
                        .with_local_anchor1(socket_a)
                        .with_local_anchor2(socket_b)
                        .with_hinge_axis(LAST_BEACON_VEHICLE_HINGE_AXIS),
                );
            }
        }

        commands
            .entity(connection_entity)
            .insert(LastBeaconVehicleConnectionResolved);
    }
}

struct ResolvedModuleInstance {
    entity: Entity,
    is_pending: bool,
}

fn find_named_module_instance(
    sibling_entities: &Children,
    module_instances: &Query<
        (Entity, &Name, Option<&LastBeaconVehicleModuleInstancePending>),
        With<LastBeaconVehicleModuleInstance>,
    >,
    instance_name: &str,
) -> Option<ResolvedModuleInstance> {
    sibling_entities.iter().find_map(|sibling_entity| {
        let (entity, name, pending) = module_instances.get(sibling_entity).ok()?;
        (name.as_str() == instance_name).then_some(ResolvedModuleInstance {
            entity,
            is_pending: pending.is_some(),
        })
    })
}

fn find_named_socket(
    module_entity: Entity,
    children_query: &Query<&Children>,
    sockets: &Query<(&LastBeaconVehicleModuleSocket, &Transform)>,
    socket_name: &str,
) -> Option<Vec3> {
    let module_children = children_query.get(module_entity).ok()?;
    module_children.iter().find_map(|child_entity| {
        let (socket, transform) = sockets.get(child_entity).ok()?;
        (socket.socket_name == socket_name).then_some(transform.translation)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, wire_last_beacon_vehicle_connections);
        app
    }

    fn spawn_resolved_module(
        world: &mut World,
        name: &str,
        socket_name: &str,
        socket_offset: Vec3,
    ) -> Entity {
        let module_entity = world.spawn(Name::new(name.to_string())).id();
        let socket_entity = world
            .spawn((
                LastBeaconVehicleModuleSocket {
                    socket_name: socket_name.to_string(),
                },
                Transform::from_translation(socket_offset),
            ))
            .id();
        world.entity_mut(module_entity).add_child(socket_entity);
        // The wiring system only requires `With<LastBeaconVehicleModuleInstance>`
        // to recognize an entity as a module instance; content beyond that
        // doesn't matter for this test.
        world
            .entity_mut(module_entity)
            .insert(LastBeaconVehicleModuleInstance {
                asset_path: "unused.bsn".to_string(),
            });
        module_entity
    }

    #[test]
    fn a_fixed_connection_between_two_resolved_modules_spawns_a_fixed_joint() {
        let mut app = test_app();
        let world = app.world_mut();

        let module_a = spawn_resolved_module(world, "ModuleA", "front", Vec3::new(0.5, 0.0, 0.0));
        let module_b = spawn_resolved_module(world, "ModuleB", "root", Vec3::new(-1.0, 0.0, 0.0));
        let vehicle_root = world.spawn_empty().id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
                joint_kind: LastBeaconVehicleJointKind::Fixed,
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

        let mut joints = app.world_mut().query::<&FixedJoint>();
        let joint = joints
            .iter(app.world())
            .next()
            .expect("a FixedJoint should have been spawned");
        assert_eq!(joint.body1, module_a);
        assert_eq!(joint.body2, module_b);
        assert!(app
            .world()
            .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
            .is_some());
    }

    #[test]
    fn a_hinge_connection_spawns_a_revolute_joint_instead() {
        let mut app = test_app();
        let world = app.world_mut();

        let chassis = spawn_resolved_module(world, "Chassis", "corner", Vec3::new(1.0, 0.0, 1.0));
        let wheel = spawn_resolved_module(world, "Wheel", "axle", Vec3::ZERO);
        let vehicle_root = world.spawn_empty().id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "Chassis".to_string(),
                socket_a: "corner".to_string(),
                module_b: "Wheel".to_string(),
                socket_b: "axle".to_string(),
                joint_kind: LastBeaconVehicleJointKind::Hinge,
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[chassis, wheel, connection_entity]);

        app.update();

        let mut joints = app.world_mut().query::<&RevoluteJoint>();
        assert_eq!(joints.iter(app.world()).count(), 1);
        let mut fixed_joints = app.world_mut().query::<&FixedJoint>();
        assert_eq!(fixed_joints.iter(app.world()).count(), 0);
    }

    #[test]
    fn a_connection_referencing_a_still_pending_module_does_not_spawn_a_joint_yet() {
        let mut app = test_app();
        let world = app.world_mut();

        let module_a = spawn_resolved_module(world, "ModuleA", "front", Vec3::ZERO);
        let module_b = spawn_resolved_module(world, "ModuleB", "root", Vec3::ZERO);
        // Simulate ModuleB still loading its own `.bsn` module definition.
        world
            .entity_mut(module_b)
            .insert(LastBeaconVehicleModuleInstancePending {
                scene_handle: Handle::default(),
            });
        let vehicle_root = world.spawn_empty().id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
                joint_kind: LastBeaconVehicleJointKind::Fixed,
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

        let mut joints = app.world_mut().query::<&FixedJoint>();
        assert_eq!(
            joints.iter(app.world()).count(),
            0,
            "no joint should spawn while ModuleB is still pending"
        );
        assert!(app
            .world()
            .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
            .is_none());

        // Now let it finish "loading" and confirm the joint spawns.
        world.entity_mut(module_b).remove::<LastBeaconVehicleModuleInstancePending>();
        app.update();

        let mut joints = app.world_mut().query::<&FixedJoint>();
        assert_eq!(joints.iter(app.world()).count(), 1);
    }
}
```

- [ ] **Step 3: Run the tests to see them fail first**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon connection::`
Expected: fails to compile until Task 7 Step 1's `mod.rs` changes are in place (they already are, from this same task). If it fails to compile on `Handle::default()` needing a type annotation, change that line to `scene_handle: Handle::<bevy::scene::ScenePatch>::default(),` and add `use bevy::scene::ScenePatch;` to the test imports.

- [ ] **Step 4: Fix any compile errors, then confirm all three tests pass**

Run: `cd E:/GameDev/last-beacon/game && cargo test -p last-beacon connection::`
Expected: all three tests pass.

- [ ] **Step 5: Commit**

```bash
git add game/src/shared/vehicle/mod.rs game/src/shared/vehicle/connection.rs
git commit -m "Wire vehicle module connections into Avian3D fixed and hinge joints"
```

---

### Task 8: Full workspace build check

**Files:** none (verification only)

- [ ] **Step 1: Run the full test suite for the game crate**

Run: `cd E:/GameDev/last-beacon/game && cargo test`
Expected: every test passes, including the pre-existing suite (this confirms nothing in Tasks 1-7 broke existing behavior).

- [ ] **Step 2: Run the full test suite for the engine workspace**

Run: `cd E:/GameDev/last-beacon/engine && cargo test -p foundation-runtime-library`
Expected: every test passes.

- [ ] **Step 3: Commit only if either step required fixes**

If both steps passed with no code changes, skip this commit. Otherwise:

```bash
git add -A
git commit -m "Fix build/test issues found in the full workspace check"
```

---

### Task 9: Author the four starter module `.bsn` files

**Files:**
- Create: `game/assets/vehicle/modules/core.bsn`
- Create: `game/assets/vehicle/modules/beam.bsn`
- Create: `game/assets/vehicle/modules/plate.bsn`
- Create: `game/assets/vehicle/modules/wheel.bsn`

None of these files author a `Name` (`#...` header) or a `Transform` on their own root -- a vehicle `.bsn`'s `LastBeaconVehicleModuleInstance` entity already carries both (its authored `Name` for connection lookup, its authored `Transform` for placement), and this content gets *applied onto* that same entity, so the module file must not overwrite either.

- [ ] **Step 1: Write `core.bsn`**

Create `game/assets/vehicle/modules/core.bsn`:

```
last_beacon::shared::vehicle::LastBeaconVehicleModuleBody { size_x: 1.0, size_y: 1.0, size_z: 1.0, mass: 40.0, color: "steel_blue" }
bevy_ecs::hierarchy::Children [
    #Front
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "front" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.5, y: 0.0, z: 0.0 } },

    #Back
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "back" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -0.5, y: 0.0, z: 0.0 } },

    #Left
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "left" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.0, z: -0.5 } },

    #Right
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "right" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.0, z: 0.5 } },

    #Top
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "top" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.5, z: 0.0 } },

    #Bottom
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "bottom" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: -0.5, z: 0.0 } }
]
```

- [ ] **Step 2: Write `beam.bsn`**

Create `game/assets/vehicle/modules/beam.bsn`:

```
last_beacon::shared::vehicle::LastBeaconVehicleModuleBody { size_x: 2.0, size_y: 0.4, size_z: 0.4, mass: 15.0, color: "olive" }
bevy_ecs::hierarchy::Children [
    #Root
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "root" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -1.0, y: 0.0, z: 0.0 } },

    #Tip
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "tip" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 1.0, y: 0.0, z: 0.0 } }
]
```

- [ ] **Step 3: Write `plate.bsn`**

Create `game/assets/vehicle/modules/plate.bsn`:

```
last_beacon::shared::vehicle::LastBeaconVehicleModuleBody { size_x: 2.0, size_y: 0.2, size_z: 1.5, mass: 20.0, color: "charcoal" }
bevy_ecs::hierarchy::Children [
    #FrontLeft
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "front_left" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -1.0, y: 0.0, z: -0.75 } },

    #FrontRight
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "front_right" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -1.0, y: 0.0, z: 0.75 } },

    #BackLeft
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "back_left" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 1.0, y: 0.0, z: -0.75 } },

    #BackRight
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "back_right" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 1.0, y: 0.0, z: 0.75 } }
]
```

- [ ] **Step 4: Write `wheel.bsn`**

Create `game/assets/vehicle/modules/wheel.bsn`:

```
last_beacon::shared::vehicle::LastBeaconVehicleWheelModuleBody { radius: 0.5, width: 0.3, mass: 5.0, color: "charcoal" }
bevy_ecs::hierarchy::Children [
    #Axle
    last_beacon::shared::vehicle::LastBeaconVehicleModuleSocket { socket_name: "axle" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 } }
]
```

- [ ] **Step 5: Commit**

```bash
git add game/assets/vehicle/modules
git commit -m "Author the four starter vehicle modules: core, beam, plate, wheel"
```

---

### Task 10: Author the test wagon and spawn it into the World testbed

**Files:**
- Create: `game/assets/vehicle/vehicle_module_testbed.bsn`
- Modify: `game/src/world/mod.rs`

The wagon exercises both joint kinds in one artifact: a Core welded to a Plate chassis through two Beams from two different Core sockets (the multi-root case), with four Wheels hinge-mounted at the chassis corners. Each wheel starts with an authored `AngularVelocity` around its own local Y (the hinge axis) so it visibly spins from the moment physics starts, proving the hinge joint doesn't lock rotation the way a fixed joint would.

- [ ] **Step 1: Write `vehicle_module_testbed.bsn`**

Create `game/assets/vehicle/vehicle_module_testbed.bsn`:

```
#Vehicle
bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 450.0, z: 1450.0 } }
bevy_ecs::hierarchy::Children [
    #CoreBlock
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/core.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 1.0, z: 0.0 } },

    #BeamLeft
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/beam.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.5, z: -0.75 } },

    #BeamRight
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/beam.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.5, z: 0.75 } },

    #ChassisPlate
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/plate.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 } },

    #WheelFrontLeft
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/wheel.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -1.2, y: -0.3, z: -0.9 } }
    avian3d::dynamics::rigid_body::AngularVelocity(glam::Vec3 { x: 0.0, y: 4.0, z: 0.0 }),

    #WheelFrontRight
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/wheel.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: -1.2, y: -0.3, z: 0.9 } }
    avian3d::dynamics::rigid_body::AngularVelocity(glam::Vec3 { x: 0.0, y: 4.0, z: 0.0 }),

    #WheelBackLeft
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/wheel.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 1.2, y: -0.3, z: -0.9 } }
    avian3d::dynamics::rigid_body::AngularVelocity(glam::Vec3 { x: 0.0, y: 4.0, z: 0.0 }),

    #WheelBackRight
    last_beacon::shared::vehicle::LastBeaconVehicleModuleInstance { asset_path: "vehicle/modules/wheel.bsn" }
    bevy_transform::components::transform::Transform { translation: glam::Vec3 { x: 1.2, y: -0.3, z: 0.9 } }
    avian3d::dynamics::rigid_body::AngularVelocity(glam::Vec3 { x: 0.0, y: 4.0, z: 0.0 }),

    #ConnCoreToBeamLeft
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "CoreBlock", socket_a: "left", module_b: "BeamLeft", socket_b: "root" },

    #ConnCoreToBeamRight
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "CoreBlock", socket_a: "right", module_b: "BeamRight", socket_b: "root" },

    #ConnBeamLeftToChassis
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "BeamLeft", socket_a: "tip", module_b: "ChassisPlate", socket_b: "front_left" },

    #ConnBeamRightToChassis
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "BeamRight", socket_a: "tip", module_b: "ChassisPlate", socket_b: "front_right" },

    #ConnWheelFrontLeft
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "ChassisPlate", socket_a: "front_left", module_b: "WheelFrontLeft", socket_b: "axle", joint_kind: last_beacon::shared::vehicle::LastBeaconVehicleJointKind::Hinge },

    #ConnWheelFrontRight
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "ChassisPlate", socket_a: "front_right", module_b: "WheelFrontRight", socket_b: "axle", joint_kind: last_beacon::shared::vehicle::LastBeaconVehicleJointKind::Hinge },

    #ConnWheelBackLeft
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "ChassisPlate", socket_a: "back_left", module_b: "WheelBackLeft", socket_b: "axle", joint_kind: last_beacon::shared::vehicle::LastBeaconVehicleJointKind::Hinge },

    #ConnWheelBackRight
    last_beacon::shared::vehicle::LastBeaconVehicleConnection { module_a: "ChassisPlate", socket_a: "back_right", module_b: "WheelBackRight", socket_b: "axle", joint_kind: last_beacon::shared::vehicle::LastBeaconVehicleJointKind::Hinge }
]
```

- [ ] **Step 2: Spawn the wagon from the landscape testbed's init system**

In `game/src/world/mod.rs`, inside `initialize_last_beacon_landscape_test_scenes`, spawn the wagon once per new landscape test scene -- right after the `camera_entity` is spawned (which already sits inside this function's per-scene dedup guard), and before the `if let Some(scene_owner) = effective_scene_owner` block that tags the terrain/atmosphere/sun/camera:

```rust
        let camera_entity = commands
            .spawn((
                Camera3d::default(),
                Camera {
                    order: LAST_BEACON_LANDSCAPE_CAMERA_ORDER,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                LastBeaconLandscapeCamera,
                Transform::from_translation(camera_position),
                environment::landscape_camera_rendering_bundle(),
                last_beacon_free_fly_camera_bundle(),
                Name::new("Last Beacon Landscape Free-Fly Camera"),
            ))
            .id();

        // Temporary proof of the modular-vehicle attachment system (see
        // docs/superpowers/specs/2026-09-13-vehicle-module-foundations-design.md):
        // a test wagon spawned near the camera's starting position so it's
        // immediately visible. Like the rest of this testbed, it is not
        // scene-owned and will not despawn when this scene closes -- an
        // accepted temporary limitation matching this file's existing
        // "not shippable, replace before shipping" testbed content.
        commands.spawn_bsn_asset("vehicle/vehicle_module_testbed.bsn");

        if let Some(scene_owner) = effective_scene_owner {
```

- [ ] **Step 3: Build the game crate**

Run: `cd E:/GameDev/last-beacon/game && cargo check`
Expected: compiles with no errors.

- [ ] **Step 4: Commit**

```bash
git add game/assets/vehicle/vehicle_module_testbed.bsn game/src/world/mod.rs
git commit -m "Add a test wagon proving multi-joint modules and free-spinning wheels"
```

---

### Task 11: Manual QA

**Files:** none (manual verification only)

- [ ] **Step 1: Run the game**

Run: `cd E:/GameDev/last-beacon/game && cargo run`
Expected: the game launches to the main menu without errors in the console.

- [ ] **Step 2: Open the World gameplay scene**

From the main menu, choose "QUICK RUN" (opens `last-beacon/gameplay_level`, the scene that owns the landscape testbed).

- [ ] **Step 3: Locate the wagon**

Using the free-fly camera, fly toward world position roughly `(0, 450, 1450)` -- close to the camera's own starting position. Look for a small assembly of blocks with four spinning wheels falling under gravity.

- [ ] **Step 4: Confirm the fixed joints hold the structure together**

Watch the Core/Beam/Plate assembly as it falls. It should stay rigidly connected -- the beams should not visibly separate from either the Core or the Plate, confirming the two-fixed-joints-per-beam (multi-root) case holds.

- [ ] **Step 5: Confirm the hinge joints let the wheels spin freely**

Watch the four wheels. Each was given an initial spin (`AngularVelocity`) around its own axle; they should visibly keep rotating rather than instantly stopping (a `FixedJoint` would fight the authored angular velocity back to zero on the very first physics step, so continued visible rotation is the proof this is really a free hinge). The wheels are not rotated into a "sideways" orientation in this pass, so they will look like flat spinning discs rather than car wheels -- that's expected, cosmetic-only follow-up work, not a defect; only the joint behavior (does it lock rotation or not) is being verified here.

- [ ] **Step 6: Report the result**

If both checks pass, the foundation is proven. If the structure falls apart (fixed joints not holding) or a wheel is rigid instead of spinning (hinge not working), note which joint kind misbehaved and re-check `connection.rs`'s `with_local_anchor1`/`with_local_anchor2`/`with_hinge_axis` calls against `cargo doc -p avian3d --open`'s actual current signatures before changing anchor math.

---

## Summary of new public API surface

- `foundation_runtime_library::prelude::FoundationPhysicsPlugin`
- `last_beacon::shared::vehicle::{LastBeaconVehiclePlugin, LastBeaconVehicleModuleBody, LastBeaconVehicleWheelModuleBody, LastBeaconVehicleModuleSocket, LastBeaconVehicleModuleInstance, LastBeaconVehicleConnection, LastBeaconVehicleJointKind}`
- `game/assets/vehicle/modules/{core,beam,plate,wheel}.bsn`
- `game/assets/vehicle/vehicle_module_testbed.bsn`
