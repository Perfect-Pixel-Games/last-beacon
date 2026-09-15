# Vehicle Authoring Clarity — Design

## Purpose

Make the existing vehicle module/connection system easier to author against
without touching Rust: better diagnostics when a `.bsn` connection references
a bad tag or socket name, a visible in-scene marker for a connection that
failed to resolve, and a standalone authoring guide (with a real worked
example) covering the whole loop of adding a module and wiring it into a
vehicle.

## Why

Investigation during this session (see `docs/plans/vehicle-fixes/tracker.md`)
confirmed the underlying system is already fully asset-driven: a module
instance's tag is just its `.bsn`-authored `Name`, a socket's id is a
free-text `socket_name`, and `LastBeaconVehicleConnection` stitches two
tag+socket pairs together — nothing in `game/src` hardcodes a particular
vehicle layout (verified by grep: zero hits for any concrete module name
like `"CoreBlock"` outside test assertions). So this is not an architecture
change. The user confirmed the data model already matches what they wanted;
the actual friction is authoring ergonomics: a bad tag/socket typo just logs
a generic warning with no candidate list, a failed connection is invisible
except in logs, and the socket-normal convention (the genuinely hard part of
writing a new module) only exists as scattered `.bsn`/Rust doc comments, not
a standalone reference.

## Scope

This is Phase 2 of `docs/plans/vehicle-fixes/`, continuing on the existing
`feature/vehicle-fixes` branch (root, already containing the Phase 1
ground-contact crash fix) — **no new branch**. No engine-side change is
needed; everything here is game-crate source (`game/src/shared/vehicle/`)
and docs.

## Architecture

### 1. Richer lookup-failure diagnostics (`connection.rs`)

Today, `wire_last_beacon_vehicle_connections` fires one combined `warn!` when
either `module_a` or `module_b` can't be found, and `find_named_socket`'s
`SocketLookupError::NotFound` just says "no such socket exists on it" — in
neither case does the message say what *does* exist.

Changes:
- Split the module-lookup-failure branch into two independent checks (one
  per side of the connection). Each missing side gets its own `warn!` naming
  which tag was bad, and lists every module tag actually present in the
  vehicle — collected by walking `children_query.iter_descendants(vehicle_root)`
  for every entity carrying `LastBeaconVehicleModuleInstance`, the same query
  shape `find_named_module_instance` already uses.
- Extend `SocketLookupError::NotFound`'s message (via a value carried on the
  variant, not just its `Display` impl, since the candidate list depends on
  which module was queried) to list every `socket_name` actually present on
  that module, or state "module has no sockets" if none exist.
- `NoJointType` and `ConflictingJointTypes` are unchanged — those already
  name the exact problem; a candidate list adds nothing there.

### 2. Distinguish "resolved" from "gave up" + visualize failures

`LastBeaconVehicleConnectionResolved` is currently inserted on both the
success path and every skip/give-up path, so nothing downstream (tooling,
tests, the debug gizmo system) can tell a working connection from a
permanently-abandoned one without parsing logs.

Changes:
- Add `LastBeaconVehicleConnectionFailed { reason: String }`, mirroring the
  existing `LastBeaconVehicleModuleInstanceFailed` pattern in `instance.rs`.
  Every skip branch in `wire_last_beacon_vehicle_connections` inserts this
  *instead of* `LastBeaconVehicleConnectionResolved` (a connection is either
  successfully resolved, or marked failed with a reason — never silently
  just "resolved" with no signal that it didn't actually do anything).
- Extend `debug.rs`'s existing socket-gizmo system (or add a sibling system
  in the same module, gated the same way behind `dev-tools`) to query
  `LastBeaconVehicleConnectionFailed` connections and draw a red marker:
  - both `module_a`/`module_b` resolved (socket-level failure): a small red
    sphere at the midpoint of the two module roots' `GlobalTransform`s.
  - only one module resolved: a red sphere at that module's root position.
  - neither resolved: no gizmo (nothing spatial to anchor to; the `warn!`
    is the only signal, same as today).

### 3. `docs/vehicle-authoring-guide.md`

New standalone doc, styled like `docs/ui-widgets.md` (direct, asset-path
referenced, no fluff) rather than folding more into scattered comments.
Sections:
- **Module anatomy**: shape (`LastBeaconVehicleModuleCuboidShape`/
  `CylinderShape`), `RigidBody`, `Mass`, `LastBeaconVehicleModuleColor`, then
  sockets (`LastBeaconVehicleModuleSocket { socket_name }` + `Transform` +
  exactly one of `LastBeaconVehicleFixedJoint`/`LastBeaconVehicleHingeJoint`).
- **The socket-normal convention**, worked through concretely with actual
  numbers (not just "see core.bsn's comment"): local +X as a socket's
  outward normal, and the rule for connecting two modules — rotate so the
  attaching socket's *world* normal exactly opposes the target's, translate
  so positions coincide. Include the quaternion math for at least one
  non-trivial case (e.g. a 90-degree face), not just the identity/180-degree
  cases already in `core.bsn`.
- **Vehicle anatomy**: module instances (tag = `Name`) plus
  `LastBeaconVehicleConnection` entries referencing tag+socket pairs.
- **Worked example**: one new minimal module `.bsn` (a simple two-socket
  coupling plate — small enough to fully explain inline) plus a tiny
  2-module example vehicle `.bsn` that uses it, added under
  `game/assets/vehicle/` as real, loadable assets alongside the guide (not
  just inline code blocks that can drift from what actually parses).
- Cross-link from this guide to `docs/plans/vehicle-fixes/` for the
  ground-contact-crash context, and from `shared/vehicle/mod.rs`'s module
  doc comment to this guide, so both directions are discoverable.

## Testing

- New unit tests in `connection.rs`'s existing `#[cfg(test)] mod tests` for
  the richer diagnostics: a connection naming a nonexistent module tag lists
  the real tags in its warning; a connection naming a nonexistent socket
  lists the real socket names; both cases insert
  `LastBeaconVehicleConnectionFailed` (not `Resolved`).
  (Warning content itself isn't asserted via `warn!` capture — these tests
  assert the `Failed` marker and its `reason` string field instead, which is
  actually observable from a test.)
- A test confirming a successfully-resolved connection still gets
  `LastBeaconVehicleConnectionResolved` and *not* `Failed` (regression guard
  against accidentally inverting the two paths).
- The new example module/vehicle `.bsn` assets get exercised the same way
  `vehicle_module_physics.rs` exercises the existing testbed vehicle: a
  focused integration test loads the example vehicle and asserts it settles
  without panicking. This also doubles as a guarantee the guide's worked
  example is real, loadable, non-drifting data.
- `docs/vehicle-authoring-guide.md` itself isn't tested by tooling; its
  worked-example assets being covered by an integration test is what keeps
  it honest.

## Out Of Scope

- No change to the underlying data model (tags remain plain `Name`, sockets
  remain plain `socket_name` strings) — confirmed with the user this already
  satisfies the "no pre-determined bits in code" requirement.
- No schema validation, autocomplete, or editor tooling for `.bsn` authoring
  — plain-text `.bsn` with better runtime diagnostics is the agreed scope.
- No startup-time whole-vehicle-graph validator/report — per-connection
  warnings (now richer) are sufficient; a consolidated validator is
  speculative scope not currently needed.
