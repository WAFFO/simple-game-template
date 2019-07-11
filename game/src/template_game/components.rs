use goblin::math::{Vert3, Vert4};
use specs::{Component, VecStorage};

#[derive(Default)]
pub struct Orbit {
    pub axis: Vert3,
    pub center: Vert3,
    pub radius: f32,
    pub speed: f32,
    pub angle: f32,
}

impl Orbit {
    pub fn new(axis_of_rotation: Vert3, center: Vert3, radius: f32, speed: f32) -> Orbit {
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