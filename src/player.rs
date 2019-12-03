use wasm_bindgen::prelude::*;
use std::default::Default;
use crate::card::Card;
use serde_derive::Serialize;

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct Player {
    name: Option<String>,
    id: u32,
    pub(crate) hand: Vec<Card>,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, id: u32) -> Self {
        Player {
            name: Some(name.to_string()),
            id,
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

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
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
            id: 0,
            hand: Vec::new(),
        }
    }
}
