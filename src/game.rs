mod event;

use event::Event;
use crate::card::{Color, Card, CardType, build_deck};
use crate::player::Player;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;
use std::default::Default;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
struct HostData {
    draw: Vec<Card>,
    discard: Vec<Card>,
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
    own_id: u32,
    current_player: usize,
    is_host: bool,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayResult {
    InvalidCard,
    CardPlayed,
    Nothing,
    GameOver,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(player: Player, is_host: bool) -> Self {
        let mut rng = rand::thread_rng();
        let mut draw = build_deck();
        draw.shuffle(&mut rng);
        let last = draw.pop().unwrap();
        let own_id = player.id();
        Game {
            last,
            direction: Direction::Clockwise,
            discard: Vec::new(),
            draw,
            draw_count: None,
            rng,
            players: vec![player],
            own_id,
            current_player: 0,
            is_host,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        if self.players.iter().find(|p| p.id() == player.id()).is_none() {
            self.players.push(player);
            self.players.sort_by_key(|p| p.id());
        }
    }

    pub fn remove_player(&mut self, id: u32) {
        if let Some((index, _)) = self.players.iter().enumerate().find(|(_, player)| player.id() == id) {
            let player = self.players.remove(index);
            self.draw.extend(player.hand);
            self.draw.shuffle(&mut self.rng);
            self.current_player %= self.players.len();
        }
    }

    pub fn reset(&mut self) {
        for player in self.players.iter_mut() {
            player.hand.clear();
        }
        self.draw = build_deck();
        self.draw.shuffle(&mut self.rng);
        self.draw_count = None;
        self.discard.clear();
        self.last = self.draw.pop().unwrap();
        self.current_player = 0;
        self.direction = Direction::Clockwise;
    }

    #[wasm_bindgen(getter)]
    pub fn current_player(&self) -> u32 {
        self.players[self.current_player].id()
    }

    pub fn play_index(&mut self, card_index: usize) -> PlayResult {
        let pndx = self.current_player;
        let card = match self.players[pndx].hand.get(card_index) {
            Some(card) => *card,
            None => return PlayResult::InvalidCard,
        };
        let res = self.play(card);
        if res != PlayResult::InvalidCard {
            self.players[pndx].hand.remove(card_index);
        }
        res
    }

    pub fn play(&mut self, card: Card) -> PlayResult {
        use CardType::*;
        if !card.compatible_with(self.last) || (self.draw_count.is_some() && !card.ty().can_be_stacked()) {
            return PlayResult::InvalidCard;
        }
        match card.ty() {
            PlusTwo => {
                self.draw_count = Some(self.draw_count.unwrap_or(0) + 2);
            }
            Skip => {
                if self.draw_count.is_none() {
                    self.end_turn();
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
        if self.players[self.current_player].hand.is_empty() {
            PlayResult::GameOver
        } else {
            PlayResult::CardPlayed
        }
    }

    pub fn shuffle(&mut self) {
        self.draw.extend(self.discard.drain(..));
        self.draw.shuffle(&mut self.rng);
    }

    pub fn color(&self) -> Color {
        self.last.color()
    }

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    #[wasm_bindgen(getter)]
    pub fn last(&self) -> Card {
        self.last
    }

    #[wasm_bindgen(getter)]
    pub fn draw_count(&self) -> u64 {
        self.draw_count.unwrap_or_default()
    }

    pub fn draw_one(&mut self) -> Card {
        if self.draw.is_empty() {
            self.shuffle();
        }
        let card = self.draw.pop().unwrap_or_else(|| Card::random(&mut self.rng));
        // self.players[self.current_player].draw(card);
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

    pub fn handle_event(&mut self, event: JsValue) -> PlayResult {
        if event.is_null() {
            return PlayResult::Nothing;
        }
        let event = event.into_serde().unwrap();
        match &event {
            Event::Init { last } => {
                self.last = *last;
            },
            Event::PlayCard { card, player, card_index } if *player == self.players[self.current_player].id() => {
                let res = self.play(*card);
                if res != PlayResult::InvalidCard {
                    if let Some(player) = self.players.iter_mut().find(|p| p.id() == *player) {
                        player.hand.remove(*card_index);
                        if player.hand.is_empty() {
                            return PlayResult::GameOver;
                        } else {
                            self.end_turn();
                        }
                    }
                }
                return res;
            }
            Event::EndTurn => self.end_turn(),
            _ => (),
        }
        PlayResult::Nothing
    }

    pub fn handle_host_event(&mut self, event: JsValue) {
        if event.is_null() {
            return;
        }
        let event = event.into_serde().unwrap();
        match event {
            Event::Deal { player, count } if player != self.own_id => {
                self.player_do(player, |p| p.hand.extend(std::iter::repeat(Card::default()).take(count as usize)));
                if !self.is_host {
                    for _ in 0..count {
                        self.draw_one();
                    }
                }
                if let Some(_) = self.draw_count.take() {
                    self.end_turn();
                }
            }
            Event::DrawResponse(cards) => {
                let count = cards.len();
                self.player_do(self.own_id, |p| p.hand.extend(cards));
                if !self.is_host {
                    for _ in 0..count {
                        self.draw_one();
                    }
                }
                if let Some(_) = self.draw_count.take() {
                    self.end_turn();
                }
            }
            _ => (),
        }
    }

    pub fn draw_len(&self) -> usize {
        self.draw.len()
    }

    pub fn discard_len(&self) -> usize {
        self.discard.len()
    }

    pub fn own_hand(&self) -> JsValue {
        self.players.iter().find(|p| p.id() == self.own_id).unwrap().hand()
    }

    pub fn init_event(&self) -> JsValue {
        JsValue::from_serde(&Event::Init { last: self.last }).unwrap()
    }

    pub fn end_turn_event(&self) -> JsValue {
        JsValue::from_serde(&Event::EndTurn).unwrap()
    }

    pub fn set_wild_color(&mut self, card_index: usize, color: Color) {
        let player = self.player_mut(self.own_id);
        let card = &mut player.hand[card_index];
        if card.is_wild() {
            card.color = color;
        }
    }

    pub fn play_card_event(&self, card_index: usize) -> JsValue {
        let player = self.players.iter().find(|p| p.id() == self.own_id).unwrap();
        let card = player.hand[card_index];
        JsValue::from_serde(&Event::PlayCard { card, player: player.id(), card_index }).unwrap()
    }

    pub fn deal_event(&self) -> JsValue {
        let count = self.draw_count.unwrap_or(1) as u32;
        let player = self.players[self.current_player].id();
        JsValue::from_serde(&Event::Deal { player, count }).unwrap()
    }

    pub fn draw_request(&self) -> JsValue {
        JsValue::from_serde(&Event::DrawRequest).unwrap()
    }

    pub fn draw_response(&mut self) -> JsValue {
        let count = self.draw_count.unwrap_or(1) as usize;
        let cards = std::iter::repeat_with(|| self.draw_one()).take(count).collect();
        JsValue::from_serde(&Event::DrawResponse(cards)).unwrap()
    }
}

impl Game {
    pub fn draw_pile(&self) -> &[Card] {
        &self.draw
    }

    pub fn player_do<F: FnOnce(&mut Player)>(&mut self, id: u32, f: F) {
        if let Some(p) = self.players.iter_mut().find(|p| p.id() == id) {
            f(p);
        }
    }

    pub fn player(&self, id: u32) -> &Player {
        self.players.iter().find(|p| p.id() == id).unwrap()
    }

    pub fn player_mut(&mut self, id: u32) -> &mut Player {
        self.players.iter_mut().find(|p| p.id() == id).unwrap()
    }
}
