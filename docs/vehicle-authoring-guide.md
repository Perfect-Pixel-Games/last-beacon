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

## `#Tag` Is Not A Comment

In most languages `#` starts a comment. In this project's `.bsn` grammar
(`engine/crates/foundation-runtime-library/src/dynamic_bsn_grammar.lalrpop`'s
`Name` rule) it is literal syntax for giving an entity a Bevy `Name`
component: `#PlateOne` means "this entity's `Name` is `PlateOne`," nothing
more. It has no other effect and is not stripped out or specially
interpreted beyond that.

Three different-but-similar-looking things exist in this system, and it is
easy to conflate them:

1. **A module instance's tag** -- the `#Tag` on the *module instance* entity
   in the *vehicle* `.bsn` (e.g. `#PlateOne` in `two_plate_coupling.bsn`).
   This is what `LastBeaconVehicleConnection.module_a`/`module_b` matches
   against.
2. **A socket's name** -- the `socket_name: "..."` *field value* on
   `LastBeaconVehicleModuleSocket`, authored inside the *module's own*
   `.bsn` (e.g. `socket_name: "a"` in `coupling_plate.bsn`). This is what
   `LastBeaconVehicleConnection.socket_a`/`socket_b` matches against.
3. **A socket entity's own `#Tag`** -- e.g. `#A` in `coupling_plate.bsn`.
   This is just the `.bsn` author's label for the socket entity itself,
   exactly like any other `#Tag`. **It is never read by the connection
   system.** Renaming `#A` to `#Front` changes nothing about how
   connections resolve; renaming `socket_name: "a"` to
   `socket_name: "front"` does -- and any connection still referencing
   `"a"` would then fail to resolve (see "Diagnosing A Bad Connection"
   below).

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

- `docs/vehicle-module-system-diagram.html` -- a diagrammed companion to
  this guide: the assembly diagram, socket-normal derivation, and code
  walkthrough as a single visual reference. Open directly in a browser.
- `docs/plans/vehicle-fixes/` -- the branch this guide's Phase 2 work landed
  on, including the Avian3D ground-contact-crash fix that was Phase 1.
- `docs/superpowers/specs/2026-09-15-vehicle-authoring-clarity-design.md` --
  the design this guide implements.
