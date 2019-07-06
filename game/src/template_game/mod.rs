use wasm_bindgen::prelude::*;

use specs::System;
use specs::RunNow;

use goblin::engine::Engine;
use goblin::engine::systems::UpdatePosition;
use goblin::game::Game;

pub(self) mod systems;

use systems::*;


#[wasm_bindgen]
pub struct Template {
}

impl Template {
    pub fn new(engine: &mut Engine) -> Template {
        Template{
        }
    }
}

impl Game for Template {
    fn tick(&mut self, core: &mut Engine) {
        ()
    }
}