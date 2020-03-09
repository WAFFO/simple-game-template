
use specs::{Join, Read, ReadStorage, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use goblin::glm::{Vec3, Vec4, translate, rotate_x, rotate_y, rotate_z, scale};

use crate::template_game::components::Orbit;

pub struct RunOrbit;

impl<'a> System<'a> for RunOrbit {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, WriteStorage<'a, Orbit>);

    fn run(&mut self, (delta, mut t_storage, mut orb_storage): Self::SystemData) {

        // Read implements DeRef
        let delta = delta.0;

        for (transform, orbit) in (&mut t_storage, &mut orb_storage).join() {
            orbit.angle += orbit.speed * delta;
            let matrix =
                translate(orbit.center)
                    * rotate_x(orbit.axis[0])
                    * rotate_y(orbit.axis[1])
                    * rotate_z(orbit.axis[2]);
            let vector = Vec4::new(
                orbit.angle.cos() * orbit.radius,
                0.0,
                orbit.angle.sin() * orbit.radius,
                0.0,
            );
            transform.position = (matrix * vector).xyz();
        }
    }
}