use specs::Entity;
use specs::world::Builder;
use glm::{Vec3, Vec4, Quat};
use glm;

use goblin::engine::components::{Transform, Velocity, AngularVelocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;
use goblin::engine::FSize;
use goblin::Engine;
use goblin::create_entity;

use crate::template_game::components::Orbit;

pub fn create_solid(engine: &mut Engine, mesh: UUID, position: Vec3, rotation_start: Quat, scale: f32, rotation_axis: Vec3, rotation_angle: FSize) -> Entity {
    engine.create_entity()
        .with(Transform { position, rotation: rotation_start, scale: glm::vec3(scale, scale, scale) })
        .with(Velocity { position: glm::vec3(0.0, 0.0, 0.0) })
        .with(AngularVelocity { axis_normalized: rotation_axis.normalize(), angle: rotation_angle })
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build()
}

pub fn create_player(engine: &mut Engine, position: Vec3, mesh: UUID) -> Entity {
    let mut camera = Camera::default();
    camera.update();

    engine.create_entity()
        .with(Transform { position, rotation: Quat::identity(), scale: glm::vec3(0.5, 0.5, 0.5) })
        .with(Velocity { position: glm::vec3(0.0, 0.0, 0.0) })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: glm::vec4(1.0, 1.0, 1.0, 1.0) })
        .with(camera)
        .with(PlayerController)
        .build()
}

pub fn create_light(engine: &mut Engine, mesh: UUID, position: Vec3, scale: f32, rotation: Quat) {
    engine.create_entity()
        .with(Transform { position, rotation, scale: glm::vec3(scale, scale, scale) })
        .with(Velocity { position: glm::vec3(0.0, 0.0, 0.0) })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: glm::vec4(1.0, 1.0, 1.0, 1.0) })
        .build();
}

pub fn create_moon(engine: &mut Engine, mesh: UUID, center: Vec3, axis: Vec3, radius: f32, speed: f32) {
    engine.create_entity()
        .with(Transform { position: center, rotation: Quat::identity(), scale: glm::vec3(1.0, 1.0, 1.0) })
        .with(Orbit::new(axis, center, radius, speed))
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build();
}
