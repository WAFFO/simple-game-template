use cgmath::{Vector3, Vector4, Quaternion, One, Zero};
use specs::Entity;
use specs::world::Builder;

use goblin::{Engine, create_entity};
use goblin::engine::FS;
use goblin::engine::components::{Transform, Velocity, AngularVelocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;

use crate::template_game::components::Orbit;

pub fn create_solid(engine: &mut Engine, mesh: UUID, position: Vector3<FS>, rotation_start: Quaternion<FS>, scale: FS, rotation_axis: Vector3<FS>, rotation_angle: FS) -> Entity {
    engine.create_entity()
        .with(Transform { position, rotation: rotation_start, scale: Vector3::new(scale, scale, scale) })
        .with(Velocity { position: Vector3::zero() })
        .with(AngularVelocity { axis: rotation_axis, angle: rotation_angle })
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build()
}

pub fn create_player(engine: &mut Engine, position: Vector3<FS>, mesh: UUID) -> Entity {
    let mut camera = Camera::default();
    camera.update();

    engine.create_entity()
        .with(Transform { position, rotation: Quaternion::one(), scale: Vector3::new(0.5, 0.5, 0.5) })
        .with(Velocity { position: Vector3::zero() })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vector4::new(1.0, 1.0, 1.0, 1.0) })
        .with(camera)
        .with(PlayerController)
        .build()
}

pub fn create_light(engine: &mut Engine, mesh: UUID, position: Vector3<FS>, scale: FS, rotation: Quaternion<FS>) {
    engine.create_entity()
        .with(Transform { position, rotation, scale: Vector3::new(scale, scale, scale) })
        .with(Velocity { position: Vector3::zero() })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vector4::new(1.0, 1.0, 1.0, 1.0) })
        .build();
}

pub fn create_moon(engine: &mut Engine, mesh: UUID, center: Vector3<FS>, axis: Vector3<FS>, radius: FS, speed: FS) {
    engine.create_entity()
        .with(Transform { position: center, rotation: Quaternion::one(), scale: Vector3::new(1.0, 1.0, 1.0) })
        .with(Orbit::new(axis, center, radius, speed))
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build();
}
