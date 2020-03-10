
use std::f32::consts::PI;
use specs::{Join, Read, ReadStorage, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use goblin::engine::input::{Mouse, EventType, KeyBoard};
use goblin::glm::{Vec3, Vec4};
use goblin::glm;

use crate::template_game::components::Orbit;

// systems
mod run_orbit;
mod run_camera;
mod run_input;

pub use run_orbit::RunOrbit;
pub use run_camera::RunCamera;
pub use run_input::RunInput;
