use cgmath::{Vector3, InnerSpace};
use specs::{Component, VecStorage};

use goblin::engine::FS;

pub struct Orbit {
    pub axis: Vector3<FS>,
    pub center: Vector3<FS>,
    pub radius: FS,
    pub speed: FS,
    pub angle: FS,
}

impl Orbit {
    pub fn new(axis_of_rotation: Vector3<FS>, center: Vector3<FS>, radius: f32, speed: f32) -> Orbit {
        Orbit {
            axis: axis_of_rotation.normalize(),
            center,
            radius,
            speed,
            angle: 0.0,
        }
    }
}

impl Component for Orbit {
    type Storage = VecStorage<Self>;
}

impl Default for Orbit {
    fn default() -> Self {
        Orbit {
            axis: Vector3::new(0.0, 1.0, 0.0),
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 0.0,
            speed: 0.0,
            angle: 0.0,
        }
    }
}