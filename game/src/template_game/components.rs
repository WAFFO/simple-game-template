use glm::Vec3;
use specs::{Component, VecStorage};

use goblin::engine::FSize;

pub struct Orbit {
    pub axis: Vec3,
    pub center: Vec3,
    pub radius: FSize,
    pub speed: FSize,
    pub angle: FSize,
}

impl Orbit {
    pub fn new(axis_of_rotation: Vec3, center: Vec3, radius: FSize, speed: FSize) -> Orbit {
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
            axis: glm::vec3(0.0, 1.0, 0.0),
            center: glm::vec3(0.0, 0.0, 0.0),
            radius: 0.0,
            speed: 0.0,
            angle: 0.0,
        }
    }
}