use crate::card::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Init { last: Card },
    PlayCard { card: Card, player: u32, card_index: usize },
    Deal { player: u32, count: u32 },
    DrawResponse(Vec<Card>),
    DrawRequest,
    EndTurn,
}
