#[macro_use]
extern crate serde_derive;

mod player;
mod card;
mod game;

use wasm_bindgen::prelude::*;
use game::Game;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
