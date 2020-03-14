use goblin::Engine;
use goblin::create_entity;
use specs::Entity;
use specs::world::Builder;
use goblin::engine::components::{Transform, Velocity, AngularVelocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;
use goblin::glm::{Vec3, Vec4, Quat, FSize};

use crate::template_game::components::Orbit;

pub fn create_solid(engine: &mut Engine, mesh: UUID, position: Vec3, rotation_start: Quat, scale: f32, rotation_axis: Vec3, rotation_angle: FSize) -> Entity {
    engine.create_entity()
        .with(Transform { position, rotation: rotation_start, scale: Vec3::all(scale) })
        .with(Velocity { position: Vec3::zero() })
        .with(AngularVelocity { axis: rotation_axis, angle: rotation_angle })
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build()
}

pub fn create_player(engine: &mut Engine, position: Vec3, mesh: UUID) -> Entity {
    let mut camera = Camera::default();
    camera.update();

    engine.create_entity()
        .with(Transform { position, rotation: Quat::identity(), scale: Vec3::all(0.5) })
        .with(Velocity { position: Vec3::zero() })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vec4::one() })
        .with(camera)
        .with(PlayerController)
        .build()
}

pub fn create_light(engine: &mut Engine, mesh: UUID, position: Vec3, scale: f32, rotation: Quat) {
    engine.create_entity()
        .with(Transform { position, rotation, scale: Vec3::all(scale) })
        .with(Velocity { position: Vec3::zero() })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vec4::one() })
        .build();
}

pub fn create_moon(engine: &mut Engine, mesh: UUID, center: Vec3, axis: Vec3, radius: f32, speed: f32) {
    engine.create_entity()
        .with(Transform { position: center, rotation: Quat::default(), scale: Vec3::one() })
        .with(Orbit::new(axis, center, radius, speed))
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build();
}
