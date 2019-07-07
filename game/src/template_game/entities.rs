use goblin::Engine;
use goblin::create_entity;
use specs::{EntityBuilder, Entity};
use specs::world::Builder;
use goblin::engine::components::{Transform, Velocity, StaticMesh, Solid, Camera, Light, PlayerController};
use goblin::engine::mesh_manager::UUID;
use goblin::math::{Vert3, Vert4};

use std::f32::consts::PI;

pub fn create_solid(engine: &mut Engine, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) {
    create_entity!(
        engine,
        Transform { translation, rotation: Vert3::default(), scale: Vert3::all(scale) },
        Velocity { translation: Vert3::zero(), rotation },
        StaticMesh { mesh_id: mesh },
        Solid
    );
}

pub fn create_player(engine: &mut Engine, position: Vert3, mesh: UUID) -> Entity {
    let mut camera = Camera {
        rotation: Vert3::zero(),
        pitch: 0.0,
        yaw: PI,
        pole_arm: 0.1,
    };
    camera.update();
    create_entity!(
        engine,
        Transform { translation: position, rotation: Vert3::default(), scale: Vert3::all(1.0) },
        Velocity { translation: Vert3::zero(), rotation: Vert3::zero() },
        StaticMesh { mesh_id: mesh },
        Light { color: Vert4::one() },
        camera,
        PlayerController
    )
}

pub fn create_light(engine: &mut Engine, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) {
    engine.create_entity()
        .with(Transform { translation, rotation: Vert3::default(), scale: Vert3::all(scale) })
        .with(Velocity { translation: Vert3::zero(), rotation })
        .with(StaticMesh { mesh_id: mesh })
        .with(Light { color: Vert4::one() })
        .build();
}
