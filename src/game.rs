use crate::card::{Color, Card, CardType};
use crate::player::Player;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;
use std::default::Default;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    last: Card,
    direction: Direction,
    discard: Vec<Card>,
    draw: Vec<Card>,
    draw_count: Option<u64>,
    rng: rand::rngs::ThreadRng,
    players: Vec<Player>,
    current_player: usize,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayResult {
    InvalidCard,
    Nothing,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Game::new_with_players(vec![Player::default()])
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn play(&mut self, card: Card) -> PlayResult {
        use CardType::*;
        if !card.compatible_with(self.last) {
            return PlayResult::InvalidCard;
        }
        match card.ty() {
            PlusTwo => {
                self.draw_count = Some(self.draw_count.unwrap_or(0) + 2);
            }
            Skip => {
                if self.draw_count.is_none() {
                    // TODO actually skip
                }
            }
            Reverse => {
                self.direction = match self.direction {
                    Direction::Clockwise => Direction::CounterClockwise,
                    Direction::CounterClockwise => Direction::Clockwise,
                };
            }
            PlusFour => {
                self.draw_count = Some(self.draw_count.unwrap_or(0) + 4);
            }
            _ => (),
        }
        self.discard.push(self.last);
        self.last = card;
        PlayResult::Nothing
    }

    pub fn shuffle(&mut self) {
        self.draw.extend(self.discard.drain(..));
        self.draw.shuffle(&mut self.rng);
    }

    pub fn color(&self) -> Color {
        self.last.color()
    }

    #[wasm_bindgen(getter)]
    pub fn last(&self) -> Card {
        self.last
    }

    pub fn draw_count(&self) -> u64 {
        self.draw_count.unwrap_or_default()
    }

    pub fn draw_one(&mut self) -> Card {
        if self.draw.is_empty() {
            self.shuffle();
        }
        let card = self.draw.pop().unwrap_or_else(|| Card::random(&mut self.rng));
        self.players[self.current_player].draw(card);
        card
    }

    pub fn end_turn(&mut self) {
        self.current_player = (self.current_player + match self.direction {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => self.players.len() - 1,
        }) % self.players.len();
    }

    #[wasm_bindgen(getter)]
    pub fn players(&self) -> JsValue {
        JsValue::from_serde(&self.players).unwrap()
    }
}

impl Game {
    pub fn draw_pile(&self) -> &[Card] {
        &self.draw
    }

    pub fn new_with_players(players: Vec<Player>) -> Self {
        let mut rng = rand::thread_rng();
        let last = Card::random(&mut rng);
        let draw = std::iter::repeat_with(|| Card::random(&mut rng)).take(100).collect();
        Game {
            last,
            direction: Direction::Clockwise,
            discard: Vec::new(),
            draw,
            draw_count: None,
            rng,
            players,
            current_player: 0,
        }
    }
}
