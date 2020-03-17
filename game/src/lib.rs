extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate console_error_panic_hook;
extern crate specs;

use wasm_bindgen::prelude::*;
use std::panic;

extern crate goblin;

use goblin::create_app;
use goblin::Engine;
use goblin::Game;

pub mod template_game;

use template_game::Template;


//make application struct here, based on our game: Template
create_app!(Template);

// MAIN
#[wasm_bindgen]
pub fn run(canvas_id : String) -> Result<Application, JsValue> {
    // hook panics to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut engine = Engine::new(canvas_id)?;
    let game = Template::new(&mut engine);

    let app = Application::new(engine, game);

    Ok(app)
}