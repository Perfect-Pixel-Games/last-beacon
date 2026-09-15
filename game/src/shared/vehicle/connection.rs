//! Resolves each [`LastBeaconVehicleConnection`]'s named module/socket pair.
//!
//! A weld ([`LastBeaconVehicleFixedJoint`] on either socket) fuses the two
//! modules into one compound rigid body -- reparenting the absorbed
//! module's fusion root under the surviving module's fusion root, so
//! Avian3D's native `ColliderOf` hierarchy (any `Collider` entity with no
//! `RigidBody` of its own automatically gets attached to the nearest
//! ancestor that has one) treats them as a single physical body with zero
//! relative degrees of freedom and no constraint solving involved. This is
//! also the reason reparenting -- not just pointing `ColliderOf` at the new
//! body directly without moving anything -- matters: Avian3D only writes a
//! fused collider's simulated motion back into its own `Transform` via
//! Bevy's ordinary transform *propagation*, which requires it to actually
//! be a hierarchy descendant of the body driving it. A hinge
//! ([`LastBeaconVehicleHingeJoint`] on either socket) still spawns a real
//! `RevoluteJoint`, referencing whichever compound body the chassis-side
//! module currently belongs to -- see `mod.rs`'s doc comment for why these
//! two mechanisms are split this way.
//!
//! Reparenting an absorbed module means it's no longer a direct child of the
//! vehicle root, so a *later* connection referencing it by name can't just
//! search the vehicle root's immediate children -- [`find_named_module_instance`]
//! searches the whole descendant subtree instead, so a module stays
//! findable no matter how many fusions deep it's nested.
//!
//! Because a weld connection can reference a module that was itself already
//! absorbed into a bigger fusion group by an earlier connection -- possibly
//! in this exact same frame, before any of this system's `Commands` have
//! flushed to the `World` -- every root/pose lookup below consults this
//! frame's not-yet-applied decisions before falling back to live ECS state,
//! rather than trusting live state alone.

use std::collections::HashMap;

use avian3d::prelude::*;
use bevy::prelude::*;

use super::instance::LastBeaconVehicleModuleInstancePending;
use super::{
    LastBeaconVehicleConnection, LastBeaconVehicleFixedJoint, LastBeaconVehicleHingeJoint,
    LastBeaconVehicleModuleInstance, LastBeaconVehicleModuleSocket,
};

/// Marks a connection whose weld/hinge has already been resolved (or
/// permanently failed to resolve), so it is not processed again every frame.
///
/// `pub` (not `pub(crate)`) because [`wire_last_beacon_vehicle_connections`]
/// is itself `pub fn` and gets re-exported all the way to the crate root
/// (`connection::wire_last_beacon_vehicle_connections` -> `pub mod vehicle`
/// -> `pub mod shared`), so rustc treats it as reachable at full `pub`
/// visibility. This type appears in that function's `Query` parameter type,
/// so it must meet that same visibility bar — `pub(crate)` alone is not
/// enough and trips clippy's `private_interfaces` lint under `-D warnings`.
#[derive(Clone, Copy, Debug, Component)]
pub struct LastBeaconVehicleConnectionResolved;

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

/// The axis every hinge joint spins around, in the *wheel* module's own
/// local space. Matches `Collider::cylinder`'s natural rotational symmetry
/// axis (its shape is defined with its height running along local Y).
///
/// Per Avian3D's `RevoluteJoint::hinge_axis` docs and its
/// `RevoluteJointSolverData::prepare` (in
/// `avian3d::dynamics::solver::xpbd::joints::revolute`), this single field is
/// reinterpreted independently in *each* body's own local frame --
/// `a1 = rotation1 * local_basis1 * hinge_axis` and the equivalent `a2` for
/// body2. For these two to describe the same physical axis, both sides must
/// agree on it in *world* space. The wheel side always does, trivially --
/// its own local Y, transformed by its own actual rotation, *is* the wheel's
/// real spin axis by definition, so its frame's basis is left at the
/// identity default. The chassis side does not: its fusion root's own local
/// Y generally has nothing to do with the wheel's spin axis, so
/// [`wire_last_beacon_vehicle_connections`] gives the root's frame a local
/// basis computed from both the root's and the wheel's actual world
/// rotations, regardless of which side of the connection the wheel is named
/// on.
const LAST_BEACON_VEHICLE_HINGE_AXIS: Vec3 = Vec3::Y;

/// Every module instance in the vehicle, along with whether it's still
/// loading its own `.bsn` content -- the one piece of state
/// [`wire_last_beacon_vehicle_connections`] can't determine from fusion
/// state alone. Factored into a named type (rather than inlined at each call
/// site) both to satisfy clippy's `type_complexity` lint and because the
/// same query shape is needed by [`find_named_module_instance`].
type ModuleInstancesQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static Name,
        Option<&'static LastBeaconVehicleModuleInstancePending>,
    ),
    With<LastBeaconVehicleModuleInstance>,
>;

/// Every socket entity, along with whichever joint-type component (if any)
/// it carries -- see [`LastBeaconVehicleModuleSocket`] for why a socket must
/// carry exactly one, with no implicit default. Factored into a named type
/// for the same reasons as [`ModuleInstancesQuery`].
type SocketsQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconVehicleModuleSocket,
        &'static Transform,
        Option<&'static LastBeaconVehicleFixedJoint>,
        Option<&'static LastBeaconVehicleHingeJoint>,
    ),
>;

/// This frame's not-yet-applied fusion decisions: an absorbed fusion root's
/// entity maps to `(new_parent, new_local_transform)`. Consulted by
/// [`find_current_fusion_root`] and [`effective_global_transform`] instead
/// of trusting live ECS state alone, since this system's own `Commands`
/// (reparenting, `RigidBody` removal) haven't flushed to the `World` yet
/// when a later connection in the same pass needs to resolve against an
/// earlier one's decision.
type PendingFusions = HashMap<Entity, (Entity, Transform)>;

/// Resolves every unresolved [`LastBeaconVehicleConnection`] once both named
/// module instances have finished loading: a weld fuses the two modules'
/// current fusion roots together, a hinge spawns a `RevoluteJoint` between
/// the wheel and the chassis-side module's current fusion root.
#[allow(clippy::too_many_arguments)]
pub fn wire_last_beacon_vehicle_connections(
    mut commands: Commands,
    connections: Query<
        (Entity, &LastBeaconVehicleConnection, &ChildOf),
        Without<LastBeaconVehicleConnectionResolved>,
    >,
    module_instances: ModuleInstancesQuery,
    children_query: Query<&Children>,
    child_of_query: Query<&ChildOf>,
    sockets: SocketsQuery,
    rigid_bodies: Query<(), With<RigidBody>>,
    transforms: Query<&Transform>,
    masses: Query<&Mass>,
) {
    let mut pending_fusions: PendingFusions = HashMap::new();
    let mut pending_mass_addition: HashMap<Entity, f32> = HashMap::new();

    for (connection_entity, connection, parent_link) in &connections {
        let vehicle_root = parent_link.parent();

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
                .and_then(|resolved| {
                    live_global_transform(resolved.entity, &transforms, &child_of_query)
                })
                .map(|global| global.translation());
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
                    &transforms,
                    &child_of_query,
                )
                .zip(effective_global_transform(
                    module_b.entity,
                    &pending_fusions,
                    &transforms,
                    &child_of_query,
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

        let is_hinge = socket_a.is_hinge || socket_b.is_hinge;
        // Hinges are resolved in the second pass below, once every weld this
        // frame has finished deciding `pending_fusions` -- a hinge must
        // always reference the *final* fusion root, never one that's about
        // to be absorbed into something bigger.
        if is_hinge {
            continue;
        }

        let Some(root_a) = find_current_fusion_root(
            module_a.entity,
            &pending_fusions,
            &child_of_query,
            &rigid_bodies,
        ) else {
            continue;
        };
        let Some(root_b) = find_current_fusion_root(
            module_b.entity,
            &pending_fusions,
            &child_of_query,
            &rigid_bodies,
        ) else {
            continue;
        };

        if root_a == root_b {
            // Already one compound body -- e.g. a weld that closes a loop
            // through modules already fused together some other way. There
            // is no relative motion to reconcile, so nothing to do.
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionResolved);
            continue;
        }

        let (
            Some(socket_a_global),
            Some(socket_b_global),
            Some(root_a_global),
            Some(root_b_global),
        ) = (
            effective_global_transform(
                socket_a.entity,
                &pending_fusions,
                &transforms,
                &child_of_query,
            ),
            effective_global_transform(
                socket_b.entity,
                &pending_fusions,
                &transforms,
                &child_of_query,
            ),
            effective_global_transform(root_a, &pending_fusions, &transforms, &child_of_query),
            effective_global_transform(root_b, &pending_fusions, &transforms, &child_of_query),
        )
        else {
            continue;
        };

        // root_a survives; root_b's whole fusion subtree (itself plus
        // anything already fused into it) is reparented under root_a.
        // Rotation is left exactly as authored -- matching this system's
        // existing convention, the `.bsn` author picks each module's
        // correct relative orientation, and only position coincidence is
        // automated -- so only translation is solved for here.
        let translation_correction = socket_a_global.translation() - socket_b_global.translation();
        let target_root_b_global = GlobalTransform::from(Transform {
            translation: root_b_global.translation() + translation_correction,
            rotation: root_b_global.rotation(),
            scale: root_b_global.compute_transform().scale,
        });
        let new_local_transform = target_root_b_global.reparented_to(&root_a_global);

        // Remove root_b's own RigidBody/Mass *before* reparenting it.
        // Avian3D's `ColliderHierarchyPlugin` installs an observer
        // (`on_body_removed`) that strips `ColliderOf` from every collider
        // currently assigned to a body whenever that body's `RigidBody` is
        // removed. Reparenting first would let a *different* observer
        // (`on_collider_body_changed`, reacting to the `ChildOf` change)
        // correctly reassign root_b's `ColliderOf` to root_a -- only for
        // `on_body_removed` to immediately wipe it out again once the
        // `RigidBody` removal it's also chained onto fires. Removing first
        // lets that cleanup happen against root_b's *old* (self-referential)
        // assignment, so reparenting's reassignment isn't clobbered.
        commands.entity(root_b).remove::<(RigidBody, Mass)>();
        commands
            .entity(root_b)
            .insert((ChildOf(root_a), new_local_transform));

        let root_b_total_mass = masses.get(root_b).map_or(0.0, |mass| mass.0)
            + pending_mass_addition.remove(&root_b).unwrap_or(0.0);
        *pending_mass_addition.entry(root_a).or_insert(0.0) += root_b_total_mass;

        pending_fusions.insert(root_b, (root_a, new_local_transform));
        commands
            .entity(connection_entity)
            .insert(LastBeaconVehicleConnectionResolved);
    }

    for (root, added_mass) in pending_mass_addition {
        let current_mass = masses.get(root).map_or(0.0, |mass| mass.0);
        commands
            .entity(root)
            .insert(Mass(current_mass + added_mass));
    }

    // Give any fusion decided this frame a chance to actually flush and let
    // Avian3D's own physics schedule observe the resulting compound body at
    // least once before a hinge joint gets attached to it -- spawning a
    // `RevoluteJoint` against a fusion root in the exact same frame it
    // absorbed new colliders (and possibly still has never been seen by
    // Avian's own internal body bookkeeping at all, for a brand new module)
    // risks the same class of "joint referencing a body before Avian has
    // registered it" issue `LastBeaconVehiclePlugin`'s system-ordering
    // comment already documents for `RigidBody` itself. Skipping hinge
    // resolution for one frame whenever fusion just happened is a strict
    // superset of that existing safety margin, and costs nothing beyond one
    // extra frame of delay -- `wire_last_beacon_vehicle_connections` runs
    // every frame, so any hinge connection deferred here resolves normally
    // on the very next call.
    if !pending_fusions.is_empty() {
        return;
    }

    for (connection_entity, connection, parent_link) in &connections {
        let vehicle_root = parent_link.parent();

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
            continue;
        };
        if module_a.is_pending || module_b.is_pending {
            continue;
        }

        let Ok(socket_a) = find_named_socket(
            module_a.entity,
            &children_query,
            &sockets,
            &connection.socket_a,
        ) else {
            continue;
        };
        let Ok(socket_b) = find_named_socket(
            module_b.entity,
            &children_query,
            &sockets,
            &connection.socket_b,
        ) else {
            continue;
        };

        let wheel_is_module_a = socket_a.is_hinge;
        let is_hinge = wheel_is_module_a || socket_b.is_hinge;
        if !is_hinge {
            continue;
        }

        let Some(root_a) = find_current_fusion_root(
            module_a.entity,
            &pending_fusions,
            &child_of_query,
            &rigid_bodies,
        ) else {
            continue;
        };
        let Some(root_b) = find_current_fusion_root(
            module_b.entity,
            &pending_fusions,
            &child_of_query,
            &rigid_bodies,
        ) else {
            continue;
        };

        let (Some(root_a_global), Some(root_b_global)) = (
            effective_global_transform(root_a, &pending_fusions, &transforms, &child_of_query),
            effective_global_transform(root_b, &pending_fusions, &transforms, &child_of_query),
        ) else {
            continue;
        };

        // The wheel side is never fused (a wheel's own socket is always a
        // hinge, never a weld), so its root is always itself and its own
        // local anchor/rotation need no adjustment. The chassis side's
        // socket may belong to a module that's since been absorbed
        // somewhere inside root_a/root_b's compound body, so its anchor is
        // computed relative to *that root*, not the original module.
        let (chassis_socket, chassis_root, chassis_root_global, wheel_local_anchor) =
            if wheel_is_module_a {
                (socket_b, root_b, root_b_global, socket_a.local_anchor)
            } else {
                (socket_a, root_a, root_a_global, socket_b.local_anchor)
            };
        let Some(chassis_socket_global) = effective_global_transform(
            chassis_socket.entity,
            &pending_fusions,
            &transforms,
            &child_of_query,
        ) else {
            continue;
        };
        let chassis_local_anchor = chassis_socket_global
            .reparented_to(&chassis_root_global)
            .translation;

        let (wheel_root, wheel_root_global, chassis_is_body1) = if wheel_is_module_a {
            (root_a, root_a_global, false)
        } else {
            (root_b, root_b_global, true)
        };

        let chassis_basis = chassis_root_global.rotation().inverse() * wheel_root_global.rotation();

        let translation_compliance = socket_a
            .hinge_translation_compliance
            .max(socket_b.hinge_translation_compliance);
        let rotation_compliance = socket_a
            .hinge_rotation_compliance
            .max(socket_b.hinge_rotation_compliance);

        let (body1, body2, local_anchor1, local_anchor2) = if chassis_is_body1 {
            (
                chassis_root,
                wheel_root,
                chassis_local_anchor,
                wheel_local_anchor,
            )
        } else {
            (
                wheel_root,
                chassis_root,
                wheel_local_anchor,
                chassis_local_anchor,
            )
        };
        let joint = RevoluteJoint::new(body1, body2)
            .with_local_anchor1(local_anchor1)
            .with_local_anchor2(local_anchor2)
            .with_hinge_axis(LAST_BEACON_VEHICLE_HINGE_AXIS)
            .with_point_compliance(translation_compliance)
            .with_align_compliance(rotation_compliance);
        let joint = if chassis_is_body1 {
            joint.with_local_basis1(chassis_basis)
        } else {
            joint.with_local_basis2(chassis_basis)
        };
        commands.spawn((joint, JointCollisionDisabled));

        commands
            .entity(connection_entity)
            .insert(LastBeaconVehicleConnectionResolved);
    }
}

/// Finds the entity that currently "owns" `entity`'s compound rigid body:
/// `entity` itself if it still has its own `RigidBody`, or whichever
/// ancestor does after however many fusions have absorbed it -- consulting
/// `pending_fusions` first (this frame's not-yet-flushed decisions) before
/// falling back to live `ChildOf`/`RigidBody` state. Returns `None` if
/// `entity` isn't resolvable yet (shouldn't happen for well-formed data
/// once a module is done loading, but defensively bounded rather than
/// looping forever on a malformed hierarchy).
fn find_current_fusion_root(
    entity: Entity,
    pending_fusions: &PendingFusions,
    child_of_query: &Query<&ChildOf>,
    rigid_bodies: &Query<(), With<RigidBody>>,
) -> Option<Entity> {
    let mut current = entity;
    for _ in 0..64 {
        if let Some(&(new_parent, _)) = pending_fusions.get(&current) {
            current = new_parent;
            continue;
        }
        if rigid_bodies.contains(current) {
            return Some(current);
        }
        current = child_of_query.get(current).ok()?.parent();
    }
    None
}

/// Computes `entity`'s current world-space [`GlobalTransform`] by walking up
/// its `ChildOf` ancestor chain and composing each ancestor's live
/// [`Transform`], rather than trusting Bevy's own [`GlobalTransform`]
/// component. Bevy only recomputes `GlobalTransform` once per frame, in
/// `PostUpdate` -- *after* this system (which runs in `Update`) has already
/// run. An entity whose `Transform`/`ChildOf` were authored this exact frame
/// (by `apply_pending_last_beacon_vehicle_module_instances`, which mutates
/// the `World` directly with no command buffering, so a module and its
/// sockets can go from not-existing to fully authored within a single
/// frame) would still report a stale, just-inserted-default `GlobalTransform`
/// if read directly -- even though its `Transform` is already correct.
/// Recomputing from `Transform` instead is always correct regardless of
/// propagation timing.
fn live_global_transform(
    entity: Entity,
    transforms: &Query<&Transform>,
    child_of_query: &Query<&ChildOf>,
) -> Option<GlobalTransform> {
    let transform = transforms.get(entity).ok()?;
    match child_of_query.get(entity) {
        Ok(child_of) => {
            let parent_global =
                live_global_transform(child_of.parent(), transforms, child_of_query)?;
            Some(parent_global.mul_transform(*transform))
        }
        Err(_) => Some(GlobalTransform::from(*transform)),
    }
}

/// Computes `entity`'s effective world-space [`GlobalTransform`], accounting
/// for any `pending_fusions` decision this exact frame -- since a
/// just-fused entity's live `Transform` won't reflect its new parent until
/// this system's `Commands` flush. Falls back to [`live_global_transform`]
/// for anything not touched this frame.
fn effective_global_transform(
    entity: Entity,
    pending_fusions: &PendingFusions,
    transforms: &Query<&Transform>,
    child_of_query: &Query<&ChildOf>,
) -> Option<GlobalTransform> {
    if let Some(&(new_parent, new_local_transform)) = pending_fusions.get(&entity) {
        let parent_global =
            effective_global_transform(new_parent, pending_fusions, transforms, child_of_query)?;
        Some(parent_global.mul_transform(new_local_transform))
    } else {
        live_global_transform(entity, transforms, child_of_query)
    }
}

#[derive(Clone, Copy)]
struct ResolvedModuleInstance {
    entity: Entity,
    is_pending: bool,
}

/// Finds a module instance named `instance_name` anywhere in `vehicle_root`'s
/// descendant subtree -- not just its direct children. A module absorbed by
/// an earlier fusion is no longer a direct child of the vehicle root (it's
/// been reparented under whatever module absorbed it, possibly several
/// fusions deep), but it must still be findable by name for any later
/// connection that references it, so this searches the whole subtree rather
/// than assuming a fixed nesting depth.
fn find_named_module_instance(
    vehicle_root: Entity,
    children_query: &Query<&Children>,
    module_instances: &ModuleInstancesQuery,
    instance_name: &str,
) -> Option<ResolvedModuleInstance> {
    children_query
        .iter_descendants(vehicle_root)
        .find_map(|descendant_entity| {
            let (entity, name, pending) = module_instances.get(descendant_entity).ok()?;
            (name.as_str() == instance_name).then_some(ResolvedModuleInstance {
                entity,
                is_pending: pending.is_some(),
            })
        })
}

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

/// Why [`find_named_socket`] couldn't resolve a named socket into a
/// [`ResolvedSocket`], so the caller can warn with an accurate reason.
enum SocketLookupError {
    /// No child of the module has a [`LastBeaconVehicleModuleSocket`] with
    /// this name.
    NotFound,
    /// A socket with this name exists, but carries neither
    /// [`LastBeaconVehicleFixedJoint`] nor [`LastBeaconVehicleHingeJoint`] --
    /// see [`LastBeaconVehicleModuleSocket`] for why there's no implicit
    /// default.
    NoJointType,
    /// A socket with this name exists, but carries *both*
    /// [`LastBeaconVehicleFixedJoint`] and [`LastBeaconVehicleHingeJoint`] --
    /// its `.bsn` entry needs to pick exactly one.
    ConflictingJointTypes,
}

impl std::fmt::Display for SocketLookupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(formatter, "no such socket exists on it"),
            Self::NoJointType => write!(
                formatter,
                "it carries neither LastBeaconVehicleFixedJoint nor LastBeaconVehicleHingeJoint"
            ),
            Self::ConflictingJointTypes => write!(
                formatter,
                "it carries both LastBeaconVehicleFixedJoint and LastBeaconVehicleHingeJoint"
            ),
        }
    }
}

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

/// A socket resolved by name: its own entity (so callers can query its
/// `GlobalTransform`), its local anchor offset (used directly only for a
/// hinge's never-fused wheel side), whether it's a hinge, and -- only
/// meaningful when it is -- its own preferred hinge compliance.
#[derive(Clone, Copy)]
struct ResolvedSocket {
    entity: Entity,
    local_anchor: Vec3,
    is_hinge: bool,
    hinge_translation_compliance: f32,
    hinge_rotation_compliance: f32,
}

fn find_named_socket(
    module_entity: Entity,
    children_query: &Query<&Children>,
    sockets: &SocketsQuery,
    socket_name: &str,
) -> Result<ResolvedSocket, SocketLookupError> {
    let module_children = children_query
        .get(module_entity)
        .map_err(|_| SocketLookupError::NotFound)?;
    let (socket_entity, transform, fixed_joint, hinge_joint) = module_children
        .iter()
        .find_map(|child_entity| {
            let (entity, socket, transform, fixed_joint, hinge_joint) =
                sockets.get(child_entity).ok()?;
            (socket.socket_name == socket_name).then_some((
                entity,
                transform,
                fixed_joint,
                hinge_joint,
            ))
        })
        .ok_or(SocketLookupError::NotFound)?;

    let (is_hinge, hinge_translation_compliance, hinge_rotation_compliance) =
        match (fixed_joint, hinge_joint) {
            (Some(_), None) => (false, 0.0, 0.0),
            (None, Some(hinge)) => (
                true,
                hinge.translation_compliance,
                hinge.rotation_compliance,
            ),
            (None, None) => return Err(SocketLookupError::NoJointType),
            (Some(_), Some(_)) => return Err(SocketLookupError::ConflictingJointTypes),
        };

    Ok(ResolvedSocket {
        entity: socket_entity,
        local_anchor: transform.translation,
        is_hinge,
        hinge_translation_compliance,
        hinge_rotation_compliance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::transform::TransformPlugin);
        // `JointCollisionDisabled`'s `on_add` hook looks up Avian3D's
        // `JointGraph` resource directly (see the joint-spawn comment in
        // `wire_last_beacon_vehicle_connections`) rather than through a
        // system, so it needs this resource to exist even though these
        // tests never load Avian3D's full physics plugin stack.
        app.init_resource::<avian3d::dynamics::joints::joint_graph::JointGraph>();
        app.add_systems(Update, wire_last_beacon_vehicle_connections);
        app
    }

    fn spawn_resolved_module(
        world: &mut World,
        name: &str,
        transform: Transform,
        socket_name: &str,
        socket_offset: Vec3,
        is_hinge: bool,
    ) -> Entity {
        let module_entity = world.spawn((Name::new(name.to_string()), transform)).id();
        let mut socket_entity = world.spawn((
            LastBeaconVehicleModuleSocket {
                socket_name: socket_name.to_string(),
            },
            Transform::from_translation(socket_offset),
        ));
        if is_hinge {
            socket_entity.insert(LastBeaconVehicleHingeJoint::default());
        } else {
            socket_entity.insert(LastBeaconVehicleFixedJoint::default());
        }
        let socket_entity = socket_entity.id();
        world.entity_mut(module_entity).add_child(socket_entity);
        // The wiring system only requires `With<LastBeaconVehicleModuleInstance>`
        // to recognize an entity as a module instance; content beyond that
        // doesn't matter for this test. `RigidBody` is included because
        // "resolved" here is meant to mean "fully materialized, ready to be
        // wired" -- matching what the `materialize_*` systems insert once a
        // module's content is actually applied.
        world.entity_mut(module_entity).insert((
            LastBeaconVehicleModuleInstance {
                asset_path: "unused.bsn".to_string(),
            },
            RigidBody::Dynamic,
        ));
        module_entity
    }

    #[test]
    fn a_fixed_connection_fuses_the_absorbed_module_under_the_surviving_one() {
        let mut app = test_app();
        let world = app.world_mut();

        // ModuleA's "front" socket sits at local +0.5. ModuleB's "root"
        // socket sits at local -1.0. ModuleB itself starts far away
        // (10, 0, 0) -- fusion must move it, not just weld it in place.
        let module_a = spawn_resolved_module(
            world,
            "ModuleA",
            Transform::IDENTITY,
            "front",
            Vec3::new(0.5, 0.0, 0.0),
            false,
        );
        let module_b = spawn_resolved_module(
            world,
            "ModuleB",
            Transform::from_xyz(10.0, 0.0, 0.0),
            "root",
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        );
        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b]);

        // Let `PostUpdate` establish real `GlobalTransform`s for the initial
        // spawn positions before the connection exists to resolve -- mirrors
        // the real pipeline, where a module is never "ready" (not pending)
        // on the very same frame its `Transform` was first set, since
        // loading its own `.bsn` content always takes at least one frame.
        app.update();

        let connection_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        app.world_mut()
            .entity_mut(vehicle_root)
            .add_child(connection_entity);

        app.update();
        // Let transform propagation apply the newly-computed local transform.
        app.update();

        assert!(
            app.world().get::<RigidBody>(module_a).is_some(),
            "the surviving module should keep its RigidBody"
        );
        assert!(
            app.world().get::<RigidBody>(module_b).is_none(),
            "the absorbed module should lose its RigidBody"
        );
        assert_eq!(
            app.world().get::<ChildOf>(module_b).map(ChildOf::parent),
            Some(module_a),
            "the absorbed module should be reparented under the surviving module"
        );
        assert!(app
            .world()
            .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
            .is_some());
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionFailed>(connection_entity)
                .is_none(),
            "a successfully-resolved connection must not also be marked Failed"
        );

        // Socket coincidence: ModuleA.front (world +0.5) should now coincide
        // with ModuleB.root (originally local -1.0 within ModuleB).
        let mut global_transforms = app.world_mut().query::<&GlobalTransform>();
        let module_b_global = *global_transforms.get(app.world(), module_b).unwrap();
        let module_b_root_socket_world = module_b_global.transform_point(Vec3::new(-1.0, 0.0, 0.0));
        assert!(
            module_b_root_socket_world.distance(Vec3::new(0.5, 0.0, 0.0)) < 1e-4,
            "ModuleB's root socket should coincide with ModuleA's front socket, got {module_b_root_socket_world:?}"
        );
    }

    #[test]
    fn a_module_absorbed_by_fusion_is_still_findable_by_a_later_connection() {
        // The bug this test guards against: naively searching only the
        // vehicle root's *direct* children for a named module used to make
        // an absorbed module unfindable by any *later* connection, since
        // fusion reparents it out of that direct-children set. ModuleB
        // welds to ModuleA first (reparenting ModuleB under ModuleA, now two
        // levels deep from the vehicle root), then a second connection
        // (spawned only after the first has resolved) still needs to find
        // ModuleB by name to weld ModuleC to it.
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
            "back",
            Vec3::ZERO,
            false,
        );
        let module_b_extra_socket = world
            .spawn((
                LastBeaconVehicleModuleSocket {
                    socket_name: "side".to_string(),
                },
                Transform::default(),
                LastBeaconVehicleFixedJoint::default(),
            ))
            .id();
        world.entity_mut(module_b).add_child(module_b_extra_socket);
        let module_c = spawn_resolved_module(
            world,
            "ModuleC",
            Transform::IDENTITY,
            "root",
            Vec3::ZERO,
            false,
        );

        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, module_c]);
        app.update();

        let first_connection = app
            .world_mut()
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "back".to_string(),
            })
            .id();
        app.world_mut()
            .entity_mut(vehicle_root)
            .add_child(first_connection);
        app.update();
        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(first_connection)
                .is_some(),
            "the first weld should have resolved"
        );
        assert_eq!(
            app.world().get::<ChildOf>(module_b).map(ChildOf::parent),
            Some(module_a),
            "ModuleB should now be nested under ModuleA, not a direct child of the vehicle root"
        );

        let second_connection = app
            .world_mut()
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleB".to_string(),
                socket_a: "side".to_string(),
                module_b: "ModuleC".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        app.world_mut()
            .entity_mut(vehicle_root)
            .add_child(second_connection);
        app.update();

        assert!(
            app.world()
                .get::<LastBeaconVehicleConnectionResolved>(second_connection)
                .is_some(),
            "ModuleB should still be findable by name after being absorbed by the first weld"
        );
        assert_eq!(
            app.world().get::<ChildOf>(module_c).map(ChildOf::parent),
            Some(module_a),
            "ModuleC should end up reparented directly under ModuleA (the ultimate root), \
             the same compound body ModuleB belongs to"
        );
    }

    #[test]
    fn fixed_joint_no_longer_spawns_any_joint_entity() {
        let mut app = test_app();
        let world = app.world_mut();

        let module_a = spawn_resolved_module(
            world,
            "ModuleA",
            Transform::IDENTITY,
            "front",
            Vec3::new(0.5, 0.0, 0.0),
            false,
        );
        let module_b = spawn_resolved_module(
            world,
            "ModuleB",
            Transform::IDENTITY,
            "root",
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        );
        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

        let mut fixed_joints = app.world_mut().query::<&FixedJoint>();
        assert_eq!(fixed_joints.iter(app.world()).count(), 0);
    }

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
        assert!(app
            .world()
            .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
            .is_none());
        assert!(
            failed.marker_position.is_some(),
            "both modules resolved, so a midpoint marker position should be set"
        );
    }

    #[test]
    fn three_fixed_connections_to_the_same_hub_fuse_into_one_compound_body() {
        // A "hub" module (ModuleHub) welds to three others in the same
        // frame -- exactly the topology that used to resonate under
        // Avian3D's XPBD solver. All four should end up as one compound
        // body rooted at ModuleHub.
        let mut app = test_app();
        let world = app.world_mut();

        let hub = spawn_resolved_module(
            world,
            "Hub",
            Transform::IDENTITY,
            "a",
            Vec3::new(1.0, 0.0, 0.0),
            false,
        );
        // Give the hub two more weld-only sockets at different offsets.
        for (name, offset) in [
            ("b", Vec3::new(-1.0, 0.0, 0.0)),
            ("c", Vec3::new(0.0, 1.0, 0.0)),
        ] {
            let socket = world
                .spawn((
                    LastBeaconVehicleModuleSocket {
                        socket_name: name.to_string(),
                    },
                    Transform::from_translation(offset),
                    LastBeaconVehicleFixedJoint::default(),
                ))
                .id();
            world.entity_mut(hub).add_child(socket);
        }

        let leaf_a = spawn_resolved_module(
            world,
            "LeafA",
            Transform::from_xyz(5.0, 0.0, 0.0),
            "root",
            Vec3::ZERO,
            false,
        );
        let leaf_b = spawn_resolved_module(
            world,
            "LeafB",
            Transform::from_xyz(-5.0, 0.0, 0.0),
            "root",
            Vec3::ZERO,
            false,
        );
        let leaf_c = spawn_resolved_module(
            world,
            "LeafC",
            Transform::from_xyz(0.0, 5.0, 0.0),
            "root",
            Vec3::ZERO,
            false,
        );

        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[hub, leaf_a, leaf_b, leaf_c]);

        // Settle real `GlobalTransform`s for the initial spawn positions
        // before any connection exists -- see the identical comment in
        // `a_fixed_connection_fuses_the_absorbed_module_under_the_surviving_one`.
        app.update();

        let connections: Vec<Entity> = [("a", "LeafA"), ("b", "LeafB"), ("c", "LeafC")]
            .into_iter()
            .map(|(hub_socket, leaf_name)| {
                app.world_mut()
                    .spawn(LastBeaconVehicleConnection {
                        module_a: "Hub".to_string(),
                        socket_a: hub_socket.to_string(),
                        module_b: leaf_name.to_string(),
                        socket_b: "root".to_string(),
                    })
                    .id()
            })
            .collect();
        app.world_mut()
            .entity_mut(vehicle_root)
            .add_children(&connections);

        app.update();
        // Let transform propagation apply the newly-computed local transforms.
        app.update();

        let mut rigid_bodies = app.world_mut().query::<(Entity, &RigidBody)>();
        let surviving_rigid_bodies: Vec<Entity> = rigid_bodies
            .iter(app.world())
            .map(|(entity, _)| entity)
            .collect();
        assert_eq!(
            surviving_rigid_bodies,
            vec![hub],
            "all three leaves should have been absorbed into the hub's compound body, \
             leaving Hub as the only RigidBody, even though all three connections \
             resolved in the same frame"
        );

        for leaf in [leaf_a, leaf_b, leaf_c] {
            assert_eq!(
                app.world().get::<ChildOf>(leaf).map(ChildOf::parent),
                Some(hub),
                "every leaf should be reparented directly under the hub"
            );
        }

        // Each leaf's "root" socket sits at its own origin, so each leaf's
        // final world position should exactly coincide with the hub socket
        // it welded to -- proving the fusion math is correct for all three
        // simultaneous, same-frame merges into one hub, not just structural
        // reparenting.
        let mut global_transforms = app.world_mut().query::<&GlobalTransform>();
        let expected_positions = [
            (leaf_a, Vec3::new(1.0, 0.0, 0.0)),
            (leaf_b, Vec3::new(-1.0, 0.0, 0.0)),
            (leaf_c, Vec3::new(0.0, 1.0, 0.0)),
        ];
        for (leaf, expected_position) in expected_positions {
            let leaf_global = *global_transforms.get(app.world(), leaf).unwrap();
            assert!(
                leaf_global.translation().distance(expected_position) < 1e-4,
                "expected {leaf:?} at {expected_position:?}, got {:?}",
                leaf_global.translation()
            );
        }
    }

    #[test]
    fn fused_module_masses_are_summed_into_the_surviving_root() {
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
        world.entity_mut(module_a).insert(Mass(40.0));
        let module_b = spawn_resolved_module(
            world,
            "ModuleB",
            Transform::IDENTITY,
            "root",
            Vec3::ZERO,
            false,
        );
        world.entity_mut(module_b).insert(Mass(10.0));

        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ModuleA".to_string(),
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

        let total_mass = app
            .world()
            .get::<Mass>(module_a)
            .expect("surviving root should have a Mass component")
            .0;
        assert_eq!(total_mass, 50.0);
        assert!(
            app.world().get::<Mass>(module_b).is_none(),
            "the absorbed module's own Mass should be removed, not left as stale data"
        );
    }

    #[test]
    fn a_socket_naming_neither_fixed_nor_hinge_joint_does_not_fuse_or_spawn_a_joint() {
        let mut app = test_app();
        let world = app.world_mut();

        // Deliberately spawns the socket without adding either
        // `LastBeaconVehicleFixedJoint` or `LastBeaconVehicleHingeJoint` --
        // an authoring mistake `wire_last_beacon_vehicle_connections` must
        // warn about and skip, not silently default to a weld for (see
        // `LastBeaconVehicleModuleSocket`'s docs for why there's no
        // implicit default).
        let module_a = world
            .spawn((Name::new("ModuleA"), Transform::default()))
            .id();
        let socket_a = world
            .spawn((
                LastBeaconVehicleModuleSocket {
                    socket_name: "front".to_string(),
                },
                Transform::default(),
            ))
            .id();
        world.entity_mut(module_a).add_child(socket_a);
        world.entity_mut(module_a).insert((
            LastBeaconVehicleModuleInstance {
                asset_path: "unused.bsn".to_string(),
            },
            RigidBody::Dynamic,
        ));

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
                socket_a: "front".to_string(),
                module_b: "ModuleB".to_string(),
                socket_b: "root".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[module_a, module_b, connection_entity]);

        app.update();

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
    }

    #[test]
    fn a_hinge_connection_spawns_a_revolute_joint_referencing_the_fusion_root() {
        // Chassis welds to ChassisExtra first (fusing them), then Wheel
        // hinges to a socket on ChassisExtra -- the resulting RevoluteJoint
        // must reference Chassis (the surviving root), not ChassisExtra
        // (which no longer has a RigidBody once fused).
        let mut app = test_app();
        let world = app.world_mut();

        let chassis = spawn_resolved_module(
            world,
            "Chassis",
            Transform::IDENTITY,
            "weld_socket",
            Vec3::new(1.0, 0.0, 0.0),
            false,
        );
        let chassis_extra = spawn_resolved_module(
            world,
            "ChassisExtra",
            Transform::from_xyz(5.0, 0.0, 0.0),
            "weld_socket",
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        );
        // ChassisExtra also carries the socket the wheel will hinge to --
        // stays `LastBeaconVehicleFixedJoint`-typed, matching the real
        // convention (only the *wheel's own* socket is ever Hinge-typed;
        // `is_hinge = socket_a.is_hinge || socket_b.is_hinge` still makes
        // the overall connection resolve as a hinge).
        let axle_socket = world
            .spawn((
                LastBeaconVehicleModuleSocket {
                    socket_name: "axle_mount".to_string(),
                },
                Transform::from_xyz(0.0, 0.0, 2.0),
                LastBeaconVehicleFixedJoint::default(),
            ))
            .id();
        world.entity_mut(chassis_extra).add_child(axle_socket);

        let wheel = spawn_resolved_module(
            world,
            "Wheel",
            Transform::from_xyz(5.0, 0.0, 2.0),
            "axle",
            Vec3::ZERO,
            true,
        );

        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let weld_connection = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "Chassis".to_string(),
                socket_a: "weld_socket".to_string(),
                module_b: "ChassisExtra".to_string(),
                socket_b: "weld_socket".to_string(),
            })
            .id();
        let hinge_connection = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "ChassisExtra".to_string(),
                socket_a: "axle_mount".to_string(),
                module_b: "Wheel".to_string(),
                socket_b: "axle".to_string(),
            })
            .id();
        world.entity_mut(vehicle_root).add_children(&[
            chassis,
            chassis_extra,
            wheel,
            weld_connection,
            hinge_connection,
        ]);

        // Frame 1 resolves the weld (fusing ChassisExtra into Chassis).
        // Hinge resolution deliberately defers whenever fusion happened that
        // same frame (see `wire_last_beacon_vehicle_connections`'s comment on
        // why), so frame 2 is what actually spawns the RevoluteJoint -- by
        // then it correctly targets Chassis, not the since-absorbed
        // ChassisExtra.
        app.update();
        app.update();

        let mut joints = app.world_mut().query::<&RevoluteJoint>();
        let joint = joints
            .iter(app.world())
            .next()
            .expect("a RevoluteJoint should have been spawned once the weld settled");
        assert!(
            joint.body1 == chassis || joint.body2 == chassis,
            "the hinge should reference Chassis (the surviving fusion root), not ChassisExtra"
        );
        assert!(
            joint.body1 == wheel || joint.body2 == wheel,
            "the hinge should still reference the wheel directly"
        );
    }

    #[test]
    fn a_hinge_connection_still_spawns_a_revolute_joint_when_the_wheel_is_named_first() {
        let mut app = test_app();
        let world = app.world_mut();

        let wheel = spawn_resolved_module(
            world,
            "Wheel",
            Transform::IDENTITY,
            "axle",
            Vec3::ZERO,
            true,
        );
        let chassis = spawn_resolved_module(
            world,
            "Chassis",
            Transform::IDENTITY,
            "corner",
            Vec3::new(1.0, 0.0, 1.0),
            false,
        );
        let vehicle_root = world.spawn(Transform::IDENTITY).id();
        let connection_entity = world
            .spawn(LastBeaconVehicleConnection {
                module_a: "Wheel".to_string(),
                socket_a: "axle".to_string(),
                module_b: "Chassis".to_string(),
                socket_b: "corner".to_string(),
            })
            .id();
        world
            .entity_mut(vehicle_root)
            .add_children(&[wheel, chassis, connection_entity]);

        app.update();

        let mut joints = app.world_mut().query::<&RevoluteJoint>();
        let joint = joints
            .iter(app.world())
            .next()
            .expect("a RevoluteJoint should have been spawned even with the wheel named first");
        assert!(joint.body1 == wheel || joint.body2 == wheel);
        assert!(joint.body1 == chassis || joint.body2 == chassis);
        assert_eq!(joint.hinge_axis, LAST_BEACON_VEHICLE_HINGE_AXIS);
    }

    #[test]
    fn a_connection_referencing_a_still_pending_module_does_not_resolve_yet() {
        let mut app = test_app();

        let (_module_a, module_b, connection_entity) = {
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
            world
                .entity_mut(module_b)
                .insert(LastBeaconVehicleModuleInstancePending {
                    scene_handle: Handle::default(),
                });
            let vehicle_root = world.spawn(Transform::IDENTITY).id();
            let connection_entity = world
                .spawn(LastBeaconVehicleConnection {
                    module_a: "ModuleA".to_string(),
                    socket_a: "front".to_string(),
                    module_b: "ModuleB".to_string(),
                    socket_b: "root".to_string(),
                })
                .id();
            world
                .entity_mut(vehicle_root)
                .add_children(&[module_a, module_b, connection_entity]);

            (module_a, module_b, connection_entity)
        };

        app.update();

        assert!(
            app.world().get::<RigidBody>(module_b).is_some(),
            "no fusion should happen while ModuleB is still pending"
        );
        assert!(app
            .world()
            .get::<LastBeaconVehicleConnectionResolved>(connection_entity)
            .is_none());

        app.world_mut()
            .entity_mut(module_b)
            .remove::<LastBeaconVehicleModuleInstancePending>();
        app.update();

        assert!(
            app.world().get::<RigidBody>(module_b).is_none(),
            "fusion should happen once ModuleB is no longer pending"
        );
    }
}
