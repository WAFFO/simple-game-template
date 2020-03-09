
use specs::{Join, Read, ReadStorage, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use goblin::engine::input::{Mouse, EventType, KeyBoard};

pub struct RunCamera {
    pub mouse: Mouse,
    pub event: EventType,
}

impl<'a> System<'a> for RunCamera {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Camera>, ReadStorage<'a, PlayerController>);

    fn run(&mut self, (delta, mut c_storage, pc_storage): Self::SystemData) {

        let delta = delta.0;

        for camera in (&mut c_storage).join() {
            if self.event == EventType::Move && self.mouse.left() {
                camera.add_yaw(self.mouse.move_x() * delta);
                camera.add_pitch(self.mouse.move_y() * delta);
            }
            else if self.event == EventType::Scroll {
                let s = self.mouse.move_s();

                if s != 0.0 {
                    camera.add_pole_arm(s/s.abs());
                }
            }
        }
    }
}