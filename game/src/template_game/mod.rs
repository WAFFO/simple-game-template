use wasm_bindgen::prelude::*;

use specs::Entity;

use goblin::engine::Engine;
use goblin::engine::systems::{UpdatePosition, UpdateRotation};
use goblin::game::Game;
use goblin::javascript;

pub(self) mod systems;
pub(self) mod components;
pub(self) mod entities;

use systems::*;
use glm::{Vec3, Quat};
use goblin::engine::input::Mouse;
use goblin::engine::input::EventType::{Move, Scroll};
use crate::template_game::components::Orbit;


#[wasm_bindgen]
pub struct Template {
    player: Entity,
}

impl Template {
    pub fn new(core: &mut Engine) -> Template {
        javascript::log("Creating new Template game...");

        // meshes
        let mesh_box = core.load_mesh("debug_color_box");
        let mesh_d20 = core.load_mesh("debug_d20");

        // register custom components
        core.register_component::<Orbit>();
        core.register_commit();

        // create entities
        let player = entities::create_player(
            core,
            Vec3::new(0.0, 0.0, 0.0),
            mesh_box,
        );

        let size = 3;
        let l =  size*-1 + 1;
        for i in l..size {
            for k in l..size {
                for m in l..size {
                    if i != 0 || k != 0 || m != 0 {
                        entities::create_solid(
                            core,
                            mesh_box,
                            Vec3::new(6.0 * i as f32, 6.0 * k as f32, 6.0 * m as f32),
                            Quat::new(1.0, 0.0, 0.0, 0.0),
                            1.0,
                            Vec3::new(1.0, 1.0, -0.5).normalize(),
                            1.0,
                        );
                    }
                }
            }
        }

        entities::create_moon(
            core,
            mesh_d20,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new( 0.0,1.0, 0.75),
            23.0,
            0.2,
        );

        Template {
            player,
        }
    }
}

impl Game for Template {
    fn tick(&mut self, core: &mut Engine) {
        let mut run_input = RunInput;
        let mut update_position = UpdatePosition;
        let mut update_rotation = UpdateRotation;
        let mut run_orbit = RunOrbit;
        core.run_system(&mut run_input);
        core.run_system(&mut update_position);
        core.run_system(&mut update_rotation);
        core.run_system(&mut run_orbit);
    }

    fn event_mouse_move(&mut self, core: &mut Engine, mouse: Mouse ) {
        if mouse.left() {
            let mut update_camera = RunCamera { mouse, event: Move };
            core.run_system(&mut update_camera);
        }
    }

    fn event_mouse_scroll(&mut self, core: &mut Engine, scroll: f32, mouse: Mouse ) {
        let mut update_camera = RunCamera { mouse, event: Scroll };
        core.run_system(&mut update_camera);
    }
}