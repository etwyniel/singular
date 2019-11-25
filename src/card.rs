use rand::Rng;
use wasm_bindgen::prelude::*;
use std::fmt::{Display, Formatter, self};
use serde_derive::{Deserialize, Serialize};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

impl Color {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        use Color::*;
        match rng.gen_range(0, 4) {
            0 => Red,
            1 => Green,
            2 => Yellow,
            3 => Blue,
            _ => unreachable!(),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum CardType {
    N0 = 0,
    N1 = 1,
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
    PlusTwo = 10,
    Skip = 11,
    Reverse = 12,
    Wild = 13,
    PlusFour = 14,
}

use CardType::*;

impl CardType {
    pub fn compatible_with(self, last: CardType) -> bool {
        self.is_wild() || self == last
    }

    pub fn is_wild(self) -> bool {
        match self {
            Wild | PlusFour => true,
            _ => false,
        }
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let n = rng.gen_range(0.0f32, 12.0);
        match n.floor() as u8 {
            0 => return N0,
            1 => return N1,
            2 => return N2,
            3 => return N3,
            4 => return N4,
            5 => return N5,
            6 => return N6,
            7 => return N7,
            8 => return N8,
            9 => return N9,
            _ => (),
        }
        if n < 10.5 {
            return PlusTwo;
        }
        if n < 11.0 {
            return Skip;
        }
        if n < 11.5 {
            return Reverse
        }
        if n < 11.75 {
            return Wild;
        }
        PlusFour
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Card {
    ty: CardType,
    color: Color,
}

impl Card {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Card {
            ty: CardType::random(rng),
            color: Color::random(rng),
        }
    }
}

#[wasm_bindgen]
impl Card {
    pub fn compatible_with(self, other: Card) -> bool {
        self.ty.compatible_with(other.ty) || self.color == other.color
    }

    pub fn color(self) -> Color {
        self.color
    }

    pub fn is_wild(self) -> bool {
        self.ty.is_wild()
    }

    pub fn ty(self) -> CardType {
        self.ty
    }

    #[wasm_bindgen(constructor)]
    pub fn from_jsvalue(value: &JsValue) -> Self {
        value.into_serde().unwrap()
    }

    pub fn display(self) -> String {
        format!("{}", self)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.ty {
            Wild => write!(f, "Wildcard"),
            PlusFour => write!(f, "+4"),
            PlusTwo => write!(f, "{:?} +2", self.color),
            Skip => write!(f, "{:?} skip", self.color),
            Reverse => write!(f, "{:?} reverse", self.color),
            number => write!(f, "{:?} {}", self.color, number as u8),
        }
    }
}

#[test]
fn compatible() {
    use Color::*;
    // assert!(Wild.compatible_with(Number(2, Red), Red));
}
