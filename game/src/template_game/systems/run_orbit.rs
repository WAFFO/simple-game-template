use specs::{Join, Read, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use glm::Quat;

use crate::template_game::components::Orbit;

pub struct RunOrbit;

impl<'a> System<'a> for RunOrbit {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, WriteStorage<'a, Orbit>);

    fn run(&mut self, (delta, mut t_storage, mut orb_storage): Self::SystemData) {

        // Read implements DeRef
        let delta = delta.0;

        for (transform, orbit) in (&mut t_storage, &mut orb_storage).join() {
            orbit.angle += orbit.speed * delta;

            let orientation = Quat::from_angle_axis(orbit.angle, orbit.axis);
            let start = orbit.axis.perpendicular();

            transform.position = (orientation * start) * orbit.radius + orbit.center;
            transform.rotation = orientation;
        }
    }
}