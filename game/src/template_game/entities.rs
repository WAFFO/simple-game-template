use goblin::Engine;
use goblin::create_entity;
use specs::{EntityBuilder, Entity};
use specs::world::Builder;
use goblin::engine::components::{Transform, Velocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;
use goblin::glm::{Vec3, Vec4};

use std::f32::consts::PI;
use crate::template_game::components::Orbit;

pub fn create_solid(engine: &mut Engine, mesh: UUID, position: Vec3, scale: f32, rotation: Vec3) {
    create_entity!(
        engine,
        Transform { position, rotation: Vec3::default(), scale: Vec3::all(scale) },
        Velocity { position: Vec3::zero(), rotation },
        StaticMesh { mesh_id: mesh },
        Solid
    );
}

pub fn create_player(engine: &mut Engine, position: Vec3, mesh: UUID) -> Entity {
    let mut camera = Camera::default();
    camera.update();

    create_entity!(
        engine,
        Transform { position: position, rotation: Vec3::default(), scale: Vec3::all(0.5) },
        Velocity { position: Vec3::zero(), rotation: Vec3::zero() },
        StaticMesh { mesh_id: mesh },
        Light { color: Vec4::one() },
        camera,
        PlayerController
    )
}

pub fn create_light(engine: &mut Engine, mesh: UUID, position: Vec3, scale: f32, rotation: Vec3) {
    engine.create_entity()
        .with(Transform { position, rotation: Vec3::default(), scale: Vec3::all(scale) })
        .with(Velocity { position: Vec3::zero(), rotation })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vec4::one() })
        .build();
}

pub fn create_moon(engine: &mut Engine, mesh: UUID, center: Vec3, axis: Vec3, radius: f32, speed: f32) {
    engine.create_entity()
        .with(Transform { position: center, rotation: Vec3::default(), scale: Vec3::one() })
        .with(Orbit::new(axis, center, radius, speed))
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build();
}
