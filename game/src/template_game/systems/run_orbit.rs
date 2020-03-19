use cgmath::{Quaternion, Rotation3, Vector3, Rad};
use specs::{Join, Read, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;

use crate::template_game::components::Orbit;

pub struct RunOrbit;

impl<'a> System<'a> for RunOrbit {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, WriteStorage<'a, Orbit>);

    fn run(&mut self, (delta, mut t_storage, mut orb_storage): Self::SystemData) {

        // Read implements DeRef
        let delta = delta.0;

        for (transform, orbit) in (&mut t_storage, &mut orb_storage).join() {
            orbit.angle += orbit.speed * delta;

            let orientation = Quaternion::from_axis_angle(orbit.axis, Rad(orbit.angle));
            let start =
                if orbit.axis.z < orbit.axis.x {
                    Vector3::new(orbit.axis.y, -orbit.axis.x, 0.0)
                }
                else {
                    Vector3::new(0.0, -orbit.axis.z, orbit.axis.y)
                };

            transform.position = (orientation.clone() * start) * orbit.radius + orbit.center;
            transform.rotation = orientation;
        }
    }
}