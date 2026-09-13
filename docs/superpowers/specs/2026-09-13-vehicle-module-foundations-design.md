# Vehicle Module Foundations — Design

## Purpose

Build the foundational system for Besiege-style modular vehicles: standalone
gameplay "modules" (blocks) that attach to each other via physics joints to
form a larger playable structure. This is *not* the editor — modules are
authored by developers as `.bsn` files, and a single test vehicle proves the
system assembles and simulates correctly. Powered features (driven wheels,
thrusters, weapons) are explicitly out of scope; this pass only covers
structural/visual modules and the attachment mechanism itself.

## Why

*Last Beacon*'s core loop is "Build Robot -> Deploy to Surface -> ...". Robots
are built from modules (batteries, cargo bays, armour, wheels, thrusters,
etc. — see `game/README.md`'s vision doc). Before any of those concrete
gameplay modules can exist, the underlying attachment/assembly system needs
to exist: how modules declare attachment points, how they get wired together
into one physically simulated structure, and how a developer authors a new
module without touching Rust systems code for every new block.

## Architecture

### Physics backend: Avian3D, owned by the engine

- `avian3d` is added as a dependency of `engine/crates/foundation-runtime-library`
  (not the game crate). A new `physics.rs` module there defines
  `FoundationPhysicsPlugin`, which installs `avian3d::PhysicsPlugins::default()`.
  It's registered into the existing `FoundationPlugin` bundle in that crate's
  `lib.rs`, alongside `scene_stack`, `menu`, `ui_theme`, etc. — the same place
  every other reusable engine subsystem lives. This is the first physics
  engine in the project; nothing currently in `engine/` or `game/` simulates
  physics (confirmed: no `avian`/`rapier`/`RigidBody` references anywhere in
  the repo before this work).
- `last-beacon` (the game crate) also takes a direct `avian3d` dependency,
  since its own vehicle systems code needs to author `RigidBody`, `Collider`,
  and joint types. Only *plugin installation* is centralized in the engine —
  the game still writes physics-aware gameplay code same as it writes
  BSN-scene code today.
- No gating/pausing of physics simulation is added in this pass. Only vehicle
  entities carry `RigidBody`/`Collider`, so no other scene (menus, Hub, the
  landscape testbed) is affected by physics running.

### One rigid body per module, joints form a graph (not a tree)

A vehicle is **not** a single fused mesh. Each attached module is spawned as
its own `RigidBody::Dynamic` entity with its own `Collider`. Modules are
connected to each other by Avian joint entities. A module can hold joints to
more than one other module simultaneously (e.g. a beam welded to two
different roots to brace a structure) — this is a general constraint graph,
not a Bevy parent/child hierarchy. Bevy's ordinary `Transform`/`ChildOf`
hierarchy is still used, but only for initial placement and organizational
grouping under the vehicle root; it does not carry the physical connection.

### Code location

New module: `game/src/shared/vehicle/`. Vehicles are relevant to both Hub
(building/previewing a robot — future work) and World (physically simulating
a deployed robot on the surface), so per the existing Hub/World one-way
boundary (`game/src/shared/mod.rs`'s documented rule), the vehicle system
lives in `shared`, not nested inside `world`.

### Module identity = asset path

A module is identified by its `.bsn` file path (e.g.
`vehicle/modules/core.bsn`), the same way `LastBeaconBsnWidget { asset_path }`
already references reusable UI widget `.bsn` files
(`game/src/ui_widgets.rs`). There is no separate string-id registry to keep
in sync with the filesystem.

### BSN authoring constraint

This project's `.bsn` files are parsed by a project-owned reflection grammar
(`engine/crates/foundation-runtime-library/src/dynamic_bsn_grammar.lalrpop`),
not Bevy's `bsn!` macro. That grammar can construct plain public-field
structs, tuple structs, and enum tuple-variants via reflection, but it
**cannot call arbitrary associated functions** (e.g. `Collider::cuboid(...)`,
`Mesh::from(...)`). Every existing widget in this codebase that needs
constructed-not-authored data follows the same fallback: a marker component
with plain public fields, authored in `.bsn`, read by a reactive system that
does the real construction in compiled Rust (e.g.
`LastBeaconPlaceholderCubeScene { cube_color, cube_size }` in `game/src/lib.rs`).
This design follows that established convention throughout.

Plain structs *can* be authored directly, though — `bevy_transform`'s
`Transform`/`glam::Vec3`/`glam::Quat` are plain public-field structs (already
proven for `bevy_color::Srgba` and `bevy_ui::geometry::UiRect` elsewhere in
this codebase's `.bsn` files), so module/socket placement is authored as
ordinary `Transform` components, not through a marker+system workaround.

## Data model

Five components, all `#[derive(Component, Reflect)]` with `#[reflect(Component, Default)]`
(required for dynamic-BSN loading, per the existing convention and its
regression test in `foundation-runtime-library`'s `lib.rs`):

- **`LastBeaconVehicleModuleBody { half_extent_x, half_extent_y, half_extent_z, mass, color }`**
  — authored on a box-shaped module's root entity (Core, Beam, Plate). A
  reactive system (`Added<LastBeaconVehicleModuleBody>`) builds the `Cuboid`
  mesh, `StandardMaterial`, `RigidBody::Dynamic`, `avian3d::collision::Collider::cuboid(...)`,
  and `Mass(...)` — mirroring `LastBeaconPlaceholderCubeScene`'s existing
  pattern exactly.
- **`LastBeaconVehicleWheelModuleBody { radius, width, mass, color }`** —
  same idea for the Wheel module's cylinder shape
  (`Collider::cylinder(radius, width)`), since a wheel isn't a box.
- **`LastBeaconVehicleModuleSocket { socket_name, attachment_kind }`** —
  authored on a child entity of a module root, alongside a plain `Transform`
  giving that socket's local anchor offset relative to the module's rigid
  body. `attachment_kind` (`LastBeaconVehicleJointKind::Fixed` default, or
  `::Hinge`) is intrinsic to the *socket*, not the connection: a wheel
  module's axle socket declares itself `Hinge`, every other socket defaults
  to `Fixed`. This is deliberate — a future in-game vehicle editor should
  never ask an end user "is this a hinge?" when they connect two parts; the
  module's own socket already knows. No orientation data beyond the anchor
  offset is authored; a `Fixed` joint welds whatever relative pose the two
  bodies have when connected, and a `Hinge` joint's axis is always the
  hinge-declaring module's own local Y (see below) — so a socket's
  `Transform` only needs a translation in practice.
- **`LastBeaconVehicleModuleInstance { asset_path }`** — authored in a
  *vehicle* `.bsn`, one per module placed in that vehicle. A system loads and
  applies that module's `.bsn` onto this entity, the same load/apply flow
  `LastBeaconBsnWidget` already uses for UI widgets, so the entity becomes
  that module's rigid body once resolved. It carries its own `Name` (e.g.
  `#CoreBlock`) and an approximate placement `Transform` for initial
  layout — approximate because the joint, not the authored transform, is
  what ultimately pins modules together once physics starts stepping.
- **`LastBeaconVehicleConnection { module_a, socket_a, module_b, socket_b }`**
  — a sibling entity under the vehicle root, naming two module instances (by
  `Name`) and a socket on each (by `socket_name`). It carries no joint-kind
  field of its own. A system waits until both referenced module instances
  have finished resolving (no longer carrying their "pending" marker),
  resolves each named socket among that module's children, and spawns the
  corresponding Avian joint (`FixedJoint` or a revolute joint) using each
  socket's local `Transform.translation` as the joint's local anchor on that
  body. The joint kind is derived from the two resolved sockets themselves:
  if either socket's `attachment_kind` is `Hinge`, a revolute joint is
  spawned (rotation axis fixed to that socket's owning module's local Y axis
  — verified against Avian3D's own solver source to be applied independently
  in each body's local frame, so this is correct regardless of which side of
  the connection names the wheel); otherwise a `FixedJoint` welds the two
  bodies. Nothing extra is authored per connection — the vehicle author just
  names two sockets.

Named sockets today, but nothing about this model requires it going forward:
the joint-wiring system only ever needs to resolve two
`(rigid body entity, local anchor, attachment kind)` pairs. A future
free-placement tool could produce a `LastBeaconVehicleConnection`-equivalent
from an arbitrary clicked point instead of a pre-declared socket without
changing this system.

## Starter modules

Four `.bsn` files under `game/assets/vehicle/modules/`:

- **`core.bsn`** — 1x1x1 box, sockets on all faces (`front`, `back`, `left`,
  `right`, `top`, `bottom`). The vehicle's structural root.
- **`beam.bsn`** — elongated box, sockets at both ends (`root`, `tip`).
  Connects two other modules to brace a structure (the two-root case).
- **`plate.bsn`** — flat box, sockets at its four corners
  (`front_left`, `front_right`, `back_left`, `back_right`). Acts as a chassis
  panel.
- **`wheel.bsn`** — cylinder, one socket (`axle`) at its center, with
  `attachment_kind: Hinge` authored directly on that socket, so any
  connection using it spins freely (unpowered — no motor/target velocity,
  just a free revolute constraint) without the vehicle author having to
  specify anything about joints.

## Test vehicle

`game/assets/scenes/vehicle_module_testbed.bsn`: a "wagon" combining both
joint kinds in one artifact —

- `CoreBlock` welded (`Fixed`) to `BeamLeft` and `BeamRight` from two
  different sockets (the multi-root triangulation case).
- Both beams welded (`Fixed`) to `ChassisPlate`'s left/right sockets.
- Four `Wheel` modules hinge-mounted (`Hinge`) at the chassis's four corner
  sockets.

It spawns in the World free-fly scene, positioned above the existing
landscape testbed, so gravity settling and free wheel rotation can be
observed directly (fly the camera up to it, watch it drop and hang together).
There's no terrain collision yet (the landscape testbed is explicitly
collision-less per its own doc comment), so this pass does not demonstrate
rolling on the ground — only that the joint graph holds together and wheels
rotate freely under gravity/disturbance.

## Testing

- Unit tests for the connection-resolution system: given two already-applied
  module instances with named sockets, resolving a `LastBeaconVehicleConnection`
  spawns exactly one joint entity of the right kind between the right
  rigid-body entities, using the sockets' authored local offsets as anchors.
- Unit test for the "wait until both modules are resolved" gating: a
  connection referencing a still-pending module instance does not spawn a
  joint yet, and does so once both resolve (mirrors the existing
  pending/apply-gating pattern already tested in `bsn_assets.rs`).
- Manual QA: run the game, open the World scene, fly to the test vehicle,
  confirm it holds its shape under gravity (fixed joints don't separate) and
  wheels visibly rotate freely (hinge joints aren't locked).
