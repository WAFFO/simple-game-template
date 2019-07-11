use goblin::Engine;
use goblin::create_entity;
use specs::{EntityBuilder, Entity};
use specs::world::Builder;
use goblin::engine::components::{Transform, Velocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;
use goblin::math::{Vert3, Vert4};

use std::f32::consts::PI;
use crate::template_game::components::Orbit;

pub fn create_solid(engine: &mut Engine, mesh: UUID, position: Vert3, scale: f32, rotation: Vert3) {
    create_entity!(
        engine,
        Transform { position, rotation: Vert3::default(), scale: Vert3::all(scale) },
        Velocity { position: Vert3::zero(), rotation },
        StaticMesh { mesh_id: mesh },
        Solid
    );
}

pub fn create_player(engine: &mut Engine, position: Vert3, mesh: UUID) -> Entity {
    let mut camera = Camera::default();
    camera.update();

    create_entity!(
        engine,
        Transform { position: position, rotation: Vert3::default(), scale: Vert3::all(0.5) },
        Velocity { position: Vert3::zero(), rotation: Vert3::zero() },
        StaticMesh { mesh_id: mesh },
        Light { color: Vert4::one() },
        camera,
        PlayerController
    )
}

pub fn create_light(engine: &mut Engine, mesh: UUID, position: Vert3, scale: f32, rotation: Vert3) {
    engine.create_entity()
        .with(Transform { position, rotation: Vert3::default(), scale: Vert3::all(scale) })
        .with(Velocity { position: Vert3::zero(), rotation })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vert4::one() })
        .build();
}

pub fn create_moon(engine: &mut Engine, mesh: UUID, center: Vert3, axis: Vert3, radius: f32, speed: f32) {
    engine.create_entity()
        .with(Transform { position: center, rotation: Vert3::default(), scale: Vert3::one() })
        .with(Orbit::new(axis, center, radius, speed))
        .with(StaticMesh { mesh_id: mesh })
        .with(Solid)
        .build();
}
