use wasm_bindgen::prelude::*;

use specs::System;
use specs::RunNow;

use goblin::engine::Engine;
use goblin::engine::systems::UpdatePosition;
use goblin::game::Game;

pub(self) mod systems;
pub(self) mod entities;

use systems::*;
use goblin::math::Vert3;


#[wasm_bindgen]
pub struct Template {

}

impl Template {
    pub fn new(engine: &mut Engine) -> Template {
        let mesh_box = engine.load_mesh("debug_color_box");

        let size = 3;
        let l =  size*-1 + 1;
        for i in l..size {
            for k in l..size {
                for m in l..size {
                    if i != 0 || k != 0 || m != 0 {
                        entities::create_solid(
                            engine,
                            mesh_box,
                            Vert3::new(6.0 * i as f32, 6.0 * k as f32, 6.0 * m as f32),
                            1.0,
                            Vert3::new(1.0, 0.0, -0.45)
                        );
                    }
                }
            }
        }

        entities::create_light(
            engine,
            mesh_box,
            Vert3::new(0.0, 0.0, 0.0),
            0.5,
            Vert3::new( 1.0, 0.0, -0.45 ),
        );

        use std::f32::consts::PI;
        entities::create_camera(
            engine,
            0.0,
            PI,
            Vert3::zero(),
        );
        Template {}
    }
}

impl Game for Template {
    fn tick(&mut self, core: &mut Engine) {
        let mut update_position = UpdatePosition;
        core.run_system(&mut update_position);
    }
}