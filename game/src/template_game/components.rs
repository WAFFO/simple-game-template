use goblin::glm::Vec3;
use specs::{Component, VecStorage};

#[derive(Default)]
pub struct Orbit {
    pub axis: Vec3,
    pub center: Vec3,
    pub radius: f32,
    pub speed: f32,
    pub angle: f32,
}

impl Orbit {
    pub fn new(axis_of_rotation: Vec3, center: Vec3, radius: f32, speed: f32) -> Orbit {
        Orbit {
            axis: axis_of_rotation,
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