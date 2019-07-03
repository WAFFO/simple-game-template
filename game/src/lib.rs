extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate console_error_panic_hook;
extern crate specs;

use wasm_bindgen::prelude::*;
use std::panic;

extern crate goblin;

use goblin::Application;
use goblin::Engine;
use goblin::Game;

pub mod template_game;

use template_game::Template;

// MAIN
#[wasm_bindgen]
pub fn run() -> Result<Application, JsValue> {
    // hook panics to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut engine = Engine::new()?;
    let game = Box::new(Template::new(&mut engine));

    let mut app = Application::new(engine, game);

    Ok(app)
}