
use specs::{Join, Read, ReadStorage, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use goblin::engine::input::{Mouse, EventType, KeyBoard};
use goblin::glm::Vec3;

pub struct RunInput;

impl<'a> System<'a> for RunInput {
    type SystemData = (
        Read<'a, KeyBoard>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, PlayerController>
    );

    fn run(&mut self, (board, mut v_storage, c_storage, pc_storage): Self::SystemData) {
        let sprint: f32 = if board[16] { 2.5 } else { 1.0 };
        for (vel, camera, _) in (&mut v_storage, &c_storage, &pc_storage).join() {
            let forward : Vec3 = camera.forward();
            let right : Vec3 = camera.right();

            vel.position = Vec3::zero();

            if board[87] {
                vel.position -= forward * 5.0 * sprint;
            }
            if board[83] {
                vel.position += forward * 5.0 * sprint;
            }
            if board[65] {
                vel.position += right * 5.0 * sprint;
            }
            if board[68] {
                vel.position -= right * 5.0 * sprint;
            }

            //vel.position.normalize();

            //pos.rotation = vel.position.normalize();
        }
    }
}