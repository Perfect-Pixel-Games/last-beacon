//! Foundational modular-vehicle attachment system.
//!
//! Modules (`.bsn`-authored blocks like a core, a beam, a plate, a wheel)
//! attach to each other to form a larger structure -- the basis every
//! playable vehicle will eventually be built from. This module owns the
//! attachment mechanism itself; concrete gameplay modules (powered wheels,
//! thrusters, weapons) are future work built on top of it.
//!
//! Two attachment kinds exist, with deliberately different mechanisms.
//! [`LastBeaconVehicleFixedJoint`] sockets ("welds") never become an Avian3D
//! physics joint at all -- Avian3D's XPBD solver proved numerically unstable
//! for rigid multi-way welds under real gameplay conditions (verified
//! experimentally: a "hub" module welded to three or more neighbors could
//! diverge into a runaway explosion on rough terrain, regardless of joint
//! compliance tuning). Instead, [`wire_last_beacon_vehicle_connections`]
//! fuses welded modules directly into one compound rigid body -- reparenting
//! the absorbed module's collider(s) under the surviving module's entity, so
//! Avian3D's native `ColliderOf` hierarchy treats them as a single physical
//! body with zero relative degrees of freedom and no constraint solving
//! involved. [`LastBeaconVehicleHingeJoint`] sockets (wheel axles) are
//! unaffected by this and still produce a real `RevoluteJoint` -- hinges
//! never showed this instability in any tested scenario, so there's no
//! reason to reinvent that mechanism.
//!
//! A module's `.bsn` is meant to be a *complete* asset definition, not just
//! data fed through bespoke Rust construction code: its shape (Bevy's own
//! `Cuboid`/`Cylinder` primitives, just behind the thin
//! [`LastBeaconVehicleModuleCuboidShape`]/[`LastBeaconVehicleModuleCylinderShape`]
//! wrappers `Component` requires -- see their own doc comments for why),
//! rigid body kind (`avian3d::dynamics::rigid_body::RigidBody`), and mass
//! (`avian3d::dynamics::rigid_body::mass_properties::components::Mass`) are
//! all authored directly, the same way `AngularVelocity` already is on a
//! vehicle-level wheel instance -- no Last-Beacon-specific field reinvents
//! what Bevy/Avian3D already provide. [`LastBeaconVehicleModuleColor`] is
//! the one component that carries data of its own, for the same
//! `Handle`-can't-be-static-data reason. [`module_body::materialize_last_beacon_vehicle_cuboid_modules`]/
//! `_cylinder_modules` are correspondingly generic -- they react to the
//! shape wrapper (scoped to vehicle modules via
//! [`LastBeaconVehicleModuleInstance`]), not a module-specific one, so
//! adding a new shape (e.g. `Sphere`) needs one more small wrapper +
//! generic system, never a bespoke data type.

mod connection;
#[cfg(feature = "dev-tools")]
mod debug;
mod instance;
mod module_body;

pub use connection::wire_last_beacon_vehicle_connections;
#[cfg(feature = "dev-tools")]
pub use debug::draw_last_beacon_vehicle_socket_gizmos;
pub use instance::{
    apply_pending_last_beacon_vehicle_module_instances, queue_last_beacon_vehicle_module_instances,
};
pub use module_body::{
    materialize_last_beacon_vehicle_cuboid_modules,
    materialize_last_beacon_vehicle_cylinder_modules,
};

use bevy::prelude::*;
use foundation_runtime_library::prelude::apply_pending_bsn_instances;

/// A module's rendered color -- the one piece of a module's visual/physical
/// makeup that genuinely can't be authored directly in `.bsn` like its shape
/// and mass can be (see the module's own doc comment): `MeshMaterial3d`
/// needs a `Handle<StandardMaterial>`, and a `Handle` can't exist as static
/// `.bsn` data, only as the result of `Assets::add` at runtime. This is that
/// unavoidable minimum -- a plain, directly-reflectable
/// [`bevy_color::Color`] -- bridged to a real material by
/// [`module_body::materialize_last_beacon_vehicle_cuboid_modules`]/
/// `_cylinder_modules`.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleColor(pub Color);

/// Thin `Component` wrapper around Bevy's own [`Cuboid`](bevy::math::primitives::Cuboid)
/// shape primitive.
///
/// `Cuboid` itself lives in `bevy_math`, which has no dependency on
/// `bevy_ecs` and so can't (and doesn't) implement `Component` -- being
/// `Reflect` doesn't imply that. This wrapper adds nothing of its own; every
/// field a `.bsn` author sets lives entirely on the wrapped `Cuboid`, so
/// there's still no Last-Beacon-specific shape data to keep in sync with
/// Bevy's own type.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleCuboidShape(pub Cuboid);

/// Thin `Component` wrapper around Bevy's own [`Cylinder`](bevy::math::primitives::Cylinder)
/// shape primitive -- same rationale as [`LastBeaconVehicleModuleCuboidShape`].
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleCylinderShape(pub Cylinder);

/// Default [`LastBeaconVehicleHingeJoint::translation_compliance`]/
/// [`LastBeaconVehicleHingeJoint::rotation_compliance`]: almost perfectly
/// rigid, but not literally zero.
///
/// Only meaningful for hinges now -- [`LastBeaconVehicleFixedJoint`] welds
/// are fused into one compound rigid body instead of becoming a compliant
/// joint (see this module's doc comment for why). Hinges themselves never
/// showed the multi-way-weld resonance instability that motivated fusion in
/// the first place; this value is kept non-zero purely so a future breaking
/// system has somewhere to introduce give, not because zero was found
/// unstable for hinges specifically.
pub const LAST_BEACON_VEHICLE_DEFAULT_JOINT_COMPLIANCE: f32 = 1.0e-5;

/// Marks a child entity of a module's root as a named attachment point.
///
/// The entity's own `Transform.translation` is that socket's local anchor
/// offset relative to the module's rigid body -- authored directly as a
/// plain `Transform` in `.bsn`, no marker/system workaround needed since
/// `Transform` is a plain public-field struct.
///
/// A socket only names and positions an attachment point; it says nothing
/// about what *kind* of joint it produces. That's a separate, explicit
/// choice: every socket's `.bsn` entry must also carry exactly one
/// joint-type component -- [`LastBeaconVehicleFixedJoint`] or
/// [`LastBeaconVehicleHingeJoint`] today, with more to come -- alongside
/// this one. There's deliberately no implicit default (e.g. via
/// `#[require(...)]`): Bevy's required-components mechanism only means
/// "insert this if absent," which can't express "exactly one of these
/// mutually exclusive types" -- a socket that explicitly authors
/// `LastBeaconVehicleHingeJoint` but also `#[require]`d a default
/// `LastBeaconVehicleFixedJoint` would silently end up with both. Requiring
/// every socket to state its joint type outright avoids that footgun and
/// keeps a module's `.bsn` the single, complete source of truth for its own
/// attachment behavior -- see [`wire_last_beacon_vehicle_connections`] for
/// how an unset socket is handled (warned and skipped, the same as a
/// missing socket name).
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
#[require(Transform)]
pub struct LastBeaconVehicleModuleSocket {
    /// Name used by a [`LastBeaconVehicleConnection`] to reference this socket.
    pub socket_name: String,
}

/// Makes a socket a rigid weld -- zero relative motion, permanently, between
/// the two sockets a [`LastBeaconVehicleConnection`] joins. The ordinary
/// choice for structural connections between non-wheel modules.
///
/// Carries no fields: a weld has no compliance concept, since it never
/// becomes a physics joint at all -- [`wire_last_beacon_vehicle_connections`]
/// fuses the two modules into one compound rigid body instead (see this
/// module's doc comment for why). A future breaking system detaches a weld
/// by reversing the fusion (splitting the absorbed module back out into its
/// own rigid body), not by loosening a compliance value.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleFixedJoint {}

/// Makes a socket a free-spinning hinge -- only meaningful for wheel-like
/// modules. Always rotates about the *wheel* module's own local Y axis,
/// matching `Collider::cylinder`'s natural rotational symmetry axis -- see
/// `connection::LAST_BEACON_VEHICLE_HINGE_AXIS`.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleHingeJoint {
    /// Same units and same "softer of the two sockets" combination rule as
    /// [`LastBeaconVehicleFixedJoint::translation_compliance`].
    pub translation_compliance: f32,
    /// Compliance for the two rotational degrees of freedom the hinge
    /// doesn't free up (`RevoluteJoint::align_compliance`). The hinge's own
    /// spin axis has no compliance concept -- it's fully unconstrained by
    /// design, not softened.
    pub rotation_compliance: f32,
}

impl Default for LastBeaconVehicleHingeJoint {
    fn default() -> Self {
        Self {
            translation_compliance: LAST_BEACON_VEHICLE_DEFAULT_JOINT_COMPLIANCE,
            rotation_compliance: LAST_BEACON_VEHICLE_DEFAULT_JOINT_COMPLIANCE,
        }
    }
}

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

/// A sibling entity under a vehicle root, naming two module instances (by
/// `Name`) and a socket on each (by `socket_name`) to join together.
///
/// [`wire_last_beacon_vehicle_connections`] resolves this once both named
/// module instances have finished loading -- what "resolving" means depends
/// on the two named sockets' own joint-type component
/// ([`LastBeaconVehicleFixedJoint`] or [`LastBeaconVehicleHingeJoint`], not
/// authored here): a weld fuses the two modules into one compound rigid
/// body, while a hinge spawns a real `RevoluteJoint` between them.
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
}

/// Installs Last Beacon's modular-vehicle attachment system.
#[derive(Default)]
pub struct LastBeaconVehiclePlugin;

impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconVehicleModuleColor>()
            .register_type::<LastBeaconVehicleModuleCuboidShape>()
            .register_type::<LastBeaconVehicleModuleCylinderShape>()
            .register_type::<LastBeaconVehicleModuleSocket>()
            .register_type::<LastBeaconVehicleFixedJoint>()
            .register_type::<LastBeaconVehicleHingeJoint>()
            .register_type::<LastBeaconVehicleModuleInstance>()
            .register_type::<LastBeaconVehicleConnection>()
            .add_systems(
                Update,
                (
                    // `queue_*`/`apply_pending_*` must run before the
                    // `materialize_*` systems: `apply_pending_*` is an
                    // exclusive system that synchronously inserts a module's
                    // authored `Cuboid`/`Cylinder`/`RigidBody`/`Mass`/etc. the
                    // instant its `.bsn` resolves (no command buffering), so
                    // running it first makes that insertion visible to the
                    // `materialize_*` systems' `Added<>` queries in this same
                    // `Update` pass. That in turn means `Mesh3d`/`Collider`
                    // insertion (deferred via `Commands`) and
                    // `wire_last_beacon_vehicle_connections`'s joint spawn
                    // (also deferred via `Commands`, and must run last since
                    // it only proceeds once a module instance is no longer
                    // pending) get flushed to the `World` together at the end
                    // of this chain -- so Avian's physics schedule never sees
                    // a joint referencing a module entity that isn't a rigid
                    // body yet. Reversing this order (materialize before
                    // apply) left a one-frame gap where a joint could
                    // reference a not-yet-a-rigid-body entity, which Avian's
                    // island builder panics on ("Neither body ... is in an
                    // island").
                    queue_last_beacon_vehicle_module_instances,
                    apply_pending_last_beacon_vehicle_module_instances,
                    materialize_last_beacon_vehicle_cuboid_modules,
                    materialize_last_beacon_vehicle_cylinder_modules,
                    wire_last_beacon_vehicle_connections,
                )
                    .chain()
                    // `queue_*` reads `Added<LastBeaconVehicleModuleInstance>`,
                    // but nothing previously ordered this chain relative to
                    // Foundation's own `apply_pending_bsn_instances` -- which
                    // is what actually spawns module-instance entities when
                    // an enclosing vehicle `.bsn` (or a nested one) resolves.
                    // Without this constraint, Bevy's scheduler is free to
                    // interleave the two plugins' systems arbitrarily, and
                    // when `apply_pending_bsn_instances` happens to run
                    // *between* `queue_*` and `wire_last_beacon_vehicle_connections`
                    // in the same `Update` pass, `wire_*` sees a
                    // freshly-spawned module instance that has no
                    // `LastBeaconVehicleModuleInstancePending` marker yet
                    // (because `queue_*` hasn't had a chance to add one) and
                    // wrongly treats "not started loading" the same as
                    // "fully resolved" -- permanently marking the connection
                    // resolved without ever finding its sockets or spawning
                    // its joint. Ordering this whole chain after Foundation's
                    // apply guarantees `queue_*` always observes a
                    // newly-spawned module instance in the same frame it
                    // appears, before `wire_*` gets a chance to look at it.
                    .after(apply_pending_bsn_instances),
            );
    }
}
