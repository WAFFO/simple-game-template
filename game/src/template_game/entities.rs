use goblin::Engine;
use specs::EntityBuilder;
use specs::world::Builder;
use goblin::engine::components::{Transform, Velocity, StaticMesh, Solid, Camera, Light};
use goblin::engine::mesh_manager::UUID;
use goblin::math::{Vert3, Vert4};

pub fn create_solid(engine: &mut Engine, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) {
    engine.create_entity(&|enty: EntityBuilder| {
        enty.with(Transform { translation, rotation: Vert3::default(), scale: Vert3::all(scale) })
            .with(Velocity { translation: Vert3::zero(), rotation })
            .with(StaticMesh { mesh_id: mesh })
            .with(Solid)
    });
}

pub fn create_camera(engine: &mut Engine, pitch: f32, yaw: f32, target: Vert3) {
    engine.create_entity(&|enty: EntityBuilder| {
        let mut camera = Camera { rotation: Vert3::zero(), target, pitch, yaw, pole_arm: 0.1 };
        camera.update();
        enty.with(camera)
    });
}

pub fn create_light(engine: &mut Engine, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) {
    engine.create_entity(&|enty: EntityBuilder| {
        enty.with(Transform { translation, rotation: Vert3::default(), scale: Vert3::all(scale) })
            .with(Velocity { translation: Vert3::zero(), rotation })
            .with(StaticMesh { mesh_id: mesh })
            .with(Light { color: Vert4::one() })
    });
}
