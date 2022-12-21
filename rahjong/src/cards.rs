use std::collections::BTreeMap;

use rand::seq::SliceRandom;

use crate::{
    card_type::{CardType, FengType, JianType, NumType, ZiType},
    case_type::CaseType,
};

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
    pub dong_open: Vec<CaseType>,
    pub nan_open: Vec<CaseType>,
    pub xi_open: Vec<CaseType>,
    pub bei_open: Vec<CaseType>,
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

fn clean_hand(hand: &mut BTreeMap<CardType, u8>) {
    for card in hand
        .iter()
        .filter(|v| *v.1 == 0)
        .map(|v| *v.0)
        .collect::<Vec<_>>()
    {
        hand.remove(&card);
    }
}

impl Cards {
    pub fn current_hand_mut(&mut self) -> &mut BTreeMap<CardType, u8> {
        self.hand_mut(self.active_player)
    }

    pub fn hand_mut(&mut self, side: FengType) -> &mut BTreeMap<CardType, u8> {
        match side {
            FengType::Dong => &mut self.dong_hand,
            FengType::Nan => &mut self.nan_hand,
            FengType::Xi => &mut self.xi_hand,
            FengType::Bei => &mut self.bei_hand,
        }
    }

    pub fn current_hand(&self) -> &BTreeMap<CardType, u8> {
        self.hand(self.active_player)
    }

    pub fn hand(&self, side: FengType) -> &BTreeMap<CardType, u8> {
        match side {
            FengType::Dong => &self.dong_hand,
            FengType::Nan => &self.nan_hand,
            FengType::Xi => &self.xi_hand,
            FengType::Bei => &self.bei_hand,
        }
    }

    pub fn current_river_mut(&mut self) -> &mut Vec<CardType> {
        self.river_mut(self.active_player)
    }

    pub fn river_mut(&mut self, side: FengType) -> &mut Vec<CardType> {
        match side {
            FengType::Dong => &mut self.dong_river,
            FengType::Nan => &mut self.nan_river,
            FengType::Xi => &mut self.xi_river,
            FengType::Bei => &mut self.bei_river,
        }
    }

    pub fn current_river(&self) -> &Vec<CardType> {
        self.river(self.active_player)
    }

    pub fn river(&self, side: FengType) -> &Vec<CardType> {
        match side {
            FengType::Dong => &self.dong_river,
            FengType::Nan => &self.nan_river,
            FengType::Xi => &self.xi_river,
            FengType::Bei => &self.bei_river,
        }
    }

    pub fn current_open_mut(&mut self) -> &mut Vec<CaseType> {
        self.open_mut(self.active_player)
    }

    pub fn open_mut(&mut self, side: FengType) -> &mut Vec<CaseType> {
        match side {
            FengType::Dong => &mut self.dong_open,
            FengType::Nan => &mut self.nan_open,
            FengType::Xi => &mut self.xi_open,
            FengType::Bei => &mut self.bei_open,
        }
    }

    pub fn current_open(&self) -> &Vec<CaseType> {
        self.open(self.active_player)
    }

    pub fn open(&self, side: FengType) -> &Vec<CaseType> {
        match side {
            FengType::Dong => &self.dong_open,
            FengType::Nan => &self.nan_open,
            FengType::Xi => &self.xi_open,
            FengType::Bei => &self.bei_open,
        }
    }

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

    pub fn play(&mut self, card: CardType) {
        let hand = self.current_hand_mut();
        *hand.get_mut(&card).unwrap() -= 1;
        clean_hand(hand);
    }

    pub fn check_an_gang(&self) -> Vec<CardType> {
        let mut res = Vec::new();
        for (&card, &num) in self.current_hand() {
            if num == 4 {
                res.push(card);
            }
        }
        res
    }

    pub fn check_jia_gang(&self) -> Vec<CardType> {
        let mut res = Vec::new();
        for &case in self.current_open() {
            match case {
                CaseType::Ke(card) if self.current_hand().contains_key(&card) => {
                    res.push(card);
                }
                _ => {}
            }
        }
        res
    }

    pub fn call(&mut self, case: CaseType, side: FengType, discard: CardType) {
        match case {
            CaseType::Shun(start) => {
                let hand = self.hand_mut(side);
                let mid = start.next();
                let end = mid.next();

                if start != discard {
                    *hand.get_mut(&start).unwrap() -= 1;
                }
                if mid != discard {
                    *hand.get_mut(&mid).unwrap() -= 1;
                }
                if end != discard {
                    *hand.get_mut(&end).unwrap() -= 1;
                }

                clean_hand(hand);

                self.open_mut(side).push(case);
                self.active_player = side;
            }
            CaseType::Ke(card) => {
                let hand = self.hand_mut(side);

                *hand.get_mut(&card).unwrap() -= 2;

                clean_hand(hand);

                self.open_mut(side).push(case);
                self.active_player = side;
            }
            CaseType::Gang(card) => {
                if let Some(index) = self
                    .current_open()
                    .into_iter()
                    .position(|case| *case == CaseType::Ke(card))
                {
                    self.current_open_mut()[index] = case;
                    self.draw();
                } else {
                    let hand = self.hand_mut(side);
                    hand.remove(&card);
                    self.open_mut(side).push(case);
                    self.active_player = side;
                    self.draw();
                }
            }
            CaseType::AnGang(card) => {
                let hand = self.current_hand_mut();
                hand.remove(&card);
                self.current_open_mut().push(case);
                self.draw();
            }
        }
    }

    pub fn check_call(&self, card: CardType) -> Vec<(FengType, CaseType)> {
        let mut res = Vec::new();

        let next_side = self.active_player.next();
        let next_hand = self.hand(next_side);
        let lastlast = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num >= NumType::Three =>
            {
                next_hand.keys().copied().find(|c| c.next().next() == card)
            }
            _ => None,
        };
        let last = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num >= NumType::Two =>
            {
                next_hand.keys().copied().find(|c| c.next() == card)
            }
            _ => None,
        };
        let next = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num <= NumType::Eight =>
            {
                next_hand.get_key_value(&card.next()).map(|v| *v.0)
            }
            _ => None,
        };
        let nextnext = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num <= NumType::Seven =>
            {
                next_hand.get_key_value(&card.next().next()).map(|v| *v.0)
            }
            _ => None,
        };
        if last.is_some() && lastlast.is_some() {
            res.push((next_side, CaseType::Shun(lastlast.unwrap())));
        }
        if last.is_some() && next.is_some() {
            res.push((next_side, CaseType::Shun(last.unwrap())));
        }
        if next.is_some() && nextnext.is_some() {
            res.push((next_side, CaseType::Shun(card)));
        }

        let hands = [
            (next_side, next_hand),
            (next_side.next(), self.hand(next_side.next())),
            (next_side.next().next(), self.hand(next_side.next().next())),
        ];

        for (side, hand) in hands {
            let num = hand.get(&card).copied().unwrap_or_default();
            if num >= 3 {
                res.push((side, CaseType::Gang(card)));
            }
            if num >= 2 {
                res.push((side, CaseType::Ke(card)));
            }
        }

        res
    }
}
