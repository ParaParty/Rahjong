use std::collections::BTreeMap;

use rand::seq::SliceRandom;

use crate::card_type::{CardType, FengType, JianType, NumType, ZiType};

#[derive(Default)]
pub struct Cards {
    pub card_mountain: Vec<CardType>,
    pub dong_hand: BTreeMap<CardType, u8>,
    pub nan_hand: BTreeMap<CardType, u8>,
    pub xi_hand: BTreeMap<CardType, u8>,
    pub bei_hand: BTreeMap<CardType, u8>,
    pub dong_river: Vec<CardType>,
    pub nan_river: Vec<CardType>,
    pub xi_river: Vec<CardType>,
    pub bei_river: Vec<CardType>,
    pub active_player: FengType,
}

fn init() -> Vec<CardType> {
    [
        CardType::Wan(NumType::One),
        CardType::Wan(NumType::Two),
        CardType::Wan(NumType::Three),
        CardType::Wan(NumType::Four),
        CardType::Wan(NumType::Five),
        CardType::Wan(NumType::Six),
        CardType::Wan(NumType::Seven),
        CardType::Wan(NumType::Eight),
        CardType::Wan(NumType::Nine),
        CardType::Tiao(NumType::One),
        CardType::Tiao(NumType::Two),
        CardType::Tiao(NumType::Three),
        CardType::Tiao(NumType::Four),
        CardType::Tiao(NumType::Five),
        CardType::Tiao(NumType::Six),
        CardType::Tiao(NumType::Seven),
        CardType::Tiao(NumType::Eight),
        CardType::Tiao(NumType::Nine),
        CardType::Tong(NumType::One),
        CardType::Tong(NumType::Two),
        CardType::Tong(NumType::Three),
        CardType::Tong(NumType::Four),
        CardType::Tong(NumType::Five),
        CardType::Tong(NumType::Six),
        CardType::Tong(NumType::Seven),
        CardType::Tong(NumType::Eight),
        CardType::Tong(NumType::Nine),
        CardType::Zi(ZiType::Jian(JianType::Bai)),
        CardType::Zi(ZiType::Jian(JianType::Fa)),
        CardType::Zi(ZiType::Jian(JianType::Zhong)),
        CardType::Zi(ZiType::Feng(FengType::Dong)),
        CardType::Zi(ZiType::Feng(FengType::Nan)),
        CardType::Zi(ZiType::Feng(FengType::Xi)),
        CardType::Zi(ZiType::Feng(FengType::Bei)),
    ]
    .repeat(4)
}

fn shuffle(cards: &mut [CardType]) {
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
}

fn deal(cards: &mut Vec<CardType>) -> BTreeMap<CardType, u8> {
    cards
        .split_off(cards.len() - 13)
        .into_iter()
        .fold(BTreeMap::new(), |mut map, card| {
            *map.entry(card).or_default() += 1;
            map
        })
}

impl Cards {
    pub fn new() -> Self {
        let mut cards = init();
        shuffle(&mut cards);

        Self {
            dong_hand: deal(&mut cards),
            nan_hand: deal(&mut cards),
            xi_hand: deal(&mut cards),
            bei_hand: deal(&mut cards),
            card_mountain: cards,
            ..Default::default()
        }
    }

    pub fn draw(&mut self) -> Option<CardType> {
        let res = self.card_mountain.pop()?;
        match self.active_player {
            FengType::Dong => *self.dong_hand.entry(res).or_default() += 1,
            FengType::Nan => *self.nan_hand.entry(res).or_default() += 1,
            FengType::Xi => *self.xi_hand.entry(res).or_default() += 1,
            FengType::Bei => *self.bei_hand.entry(res).or_default() += 1,
        }
        Some(res)
    }

    pub fn play(&mut self, card: CardType) -> bool {
        let hand = match self.active_player {
            FengType::Dong => &mut self.dong_hand,
            FengType::Nan => &mut self.nan_hand,
            FengType::Xi => &mut self.xi_hand,
            FengType::Bei => &mut self.bei_hand,
        };
        let num = if let Some(n) = hand.get_mut(&card) {
            n
        } else {
            return false;
        };
        if *num == 1 {
            hand.remove(&card);
        } else {
            *num -= 1;
        }
        true
    }
}
