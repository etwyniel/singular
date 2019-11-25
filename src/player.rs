use wasm_bindgen::prelude::*;
use std::default::Default;
use crate::card::Card;
use serde_derive::Serialize;

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct Player {
    name: Option<String>,
    hand: Vec<Card>,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        Player {
            name: Some(name.to_string()),
            hand: Vec::new(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    pub fn draw(&mut self, card: Card) {
        self.hand.push(card);
    }

    #[wasm_bindgen(getter)]
    pub fn hand(&self) -> JsValue {
        JsValue::from_serde(&self.hand).unwrap()
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: None,
            hand: Vec::new(),
        }
    }
}
