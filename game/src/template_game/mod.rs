
use specs::System;
use specs::RunNow;

use goblin::engine::Engine;
use goblin::engine::systems::UpdatePosition;
use goblin::game::Game;

pub(self) mod systems;

use systems::*;

// systems need lifetime...
// might come bite us in the butt...
// could try the old way...
// ew...
pub struct Template<'a> {
    systems: Vec<RunNow<'a>>,
}

impl Template {
    pub fn new(engine: &mut Engine) -> Template {
        let mut systems : Vec<RunNow> = Vec::new();
        systems.push(UpdatePosition);
        systems.push(Orbit);
        Template{
            systems
        }
    }
}

impl Game for Template {
    fn tick(&mut self, &mut core: Engine) {
        ()
    }
}