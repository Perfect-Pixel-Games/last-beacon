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
        (
            Entity,
            &Name,
            Option<&LastBeaconVehicleModuleInstancePending>,
        ),
        With<LastBeaconVehicleModuleInstance>,
    >,
    children_query: Query<&Children>,
    sockets: Query<(&LastBeaconVehicleModuleSocket, &Transform)>,
) {
    for (connection_entity, connection, parent_link) in &connections {
        let Ok(sibling_entities) = children_query.get(parent_link.parent()) else {
            continue;
        };

        let module_a =
            find_named_module_instance(sibling_entities, &module_instances, &connection.module_a);
        let module_b =
            find_named_module_instance(sibling_entities, &module_instances, &connection.module_b);

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

        let Some(socket_a) = find_named_socket(
            module_a.entity,
            &children_query,
            &sockets,
            &connection.socket_a,
        ) else {
            warn!(
                "LastBeaconVehicleConnection on {connection_entity:?} names socket `{}` that does not exist on module `{}`; skipping.",
                connection.socket_a, connection.module_a
            );
            commands
                .entity(connection_entity)
                .insert(LastBeaconVehicleConnectionResolved);
            continue;
        };
        let Some(socket_b) = find_named_socket(
            module_b.entity,
            &children_query,
            &sockets,
            &connection.socket_b,
        ) else {
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
        (
            Entity,
            &Name,
            Option<&LastBeaconVehicleModuleInstancePending>,
        ),
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

        let (_module_a, module_b, connection_entity) = {
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

            (module_a, module_b, connection_entity)
        };

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
        app.world_mut()
            .entity_mut(module_b)
            .remove::<LastBeaconVehicleModuleInstancePending>();
        app.update();

        let mut joints = app.world_mut().query::<&FixedJoint>();
        assert_eq!(joints.iter(app.world()).count(), 1);
    }
}
