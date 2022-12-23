//! Card states of the game and their associated methods.
//!
//! The core of this module is the [Cards] struct, which contains the states of the game.

use std::collections::BTreeMap;

use rand::seq::SliceRandom;

use crate::{
    card_type::{CardType, FengType, JianType, Next, RankType, ZiType},
    case_type::CaseType,
    river_type::RiverType,
};

/// A type alias of hand cards.
type Hand = BTreeMap<CardType, u8>;

/// The struct containing card states of the game.
#[derive(Default)]
pub struct Cards {
    /// The cards in mountain, known as 牌山 in Chinese.
    pub card_mountain: Vec<CardType>,
    /// The cards in player 东's hand, not shown to other players, known as 手牌 in Chinese, sorted by default.
    pub dong_hand: Hand,
    /// The cards in player 南's hand, not shown to other players, known as 手牌 in Chinese, sorted by default.
    pub nan_hand: Hand,
    /// The cards in player 西's hand, not shown to other players, known as 手牌 in Chinese, sorted by default.
    pub xi_hand: Hand,
    /// The cards in player 北's hand, not shown to other players, known as 手牌 in Chinese, sorted by default.
    pub bei_hand: Hand,
    /// The cards in player 东's river, shown to other players, known as 牌河 in Chinese.
    pub dong_river: Vec<RiverType>,
    /// The cards in player 南's river, shown to other players, known as 牌河 in Chinese.
    pub nan_river: Vec<RiverType>,
    /// The cards in player 西's river, shown to other players, known as 牌河 in Chinese.
    pub xi_river: Vec<RiverType>,
    /// The cards in player 北's river, shown to other players, known as 牌河 in Chinese.
    pub bei_river: Vec<RiverType>,
    /// The cards not in player 东's river, shown to other players, known as 副露 in Chinese.
    pub dong_open: Vec<CaseType>,
    /// The cards not in player 南's river, shown to other players, known as 副露 in Chinese.
    pub nan_open: Vec<CaseType>,
    /// The cards not in player 西's river, shown to other players, known as 副露 in Chinese.
    pub xi_open: Vec<CaseType>,
    /// The cards not in player 北's river, shown to other players, known as 副露 in Chinese.
    pub bei_open: Vec<CaseType>,
    /// The player who should play a card.
    pub active_player: FengType,
    /// Functions used to indicate the cards, after which being played,
    /// can lead to drawing hand(known as 听牌 in Chinese) state.
    ///
    /// When these functions are invoked, states are that the hand contains the card just being drawn.
    /// The functions are provided with the hand(手牌) and the open(副露),
    /// and each should return the cards, after which being played,
    /// can lead to drawing hand(听牌) state.
    pub drawing_hand_checkers: Vec<fn(&Hand, &Vec<CaseType>) -> Vec<CardType>>,
    /// Functions used to indicate if the current state satisfies a complete(known as 和牌 in Chinese) condition.
    ///
    /// When these functions are invoked,
    /// if the `RiverType` is `Drawing`,
    /// it indicates that the card is drawn from the mountain,
    /// and states are that the hand contains the card being drawn,
    /// or else the `RiverType` is `Normal`,
    /// it indicates that the card is played by other players,
    /// and states are that the hand **DOES NOT** contain the card just being played.
    ///
    /// The functions are provided with the hand(手牌), the open(副露),
    /// and the card just being drawn or played.
    /// These functions should return a bool, each indicating whether a complete(和牌) condition is met.
    pub completion_checkers: Vec<fn(&Hand, &Vec<CaseType>, RiverType) -> bool>,
}

/// Initialize the mountain without shuffle.
fn init() -> Vec<CardType> {
    [
        CardType::Wan(RankType::One),
        CardType::Wan(RankType::Two),
        CardType::Wan(RankType::Three),
        CardType::Wan(RankType::Four),
        CardType::Wan(RankType::Five),
        CardType::Wan(RankType::Six),
        CardType::Wan(RankType::Seven),
        CardType::Wan(RankType::Eight),
        CardType::Wan(RankType::Nine),
        CardType::Tiao(RankType::One),
        CardType::Tiao(RankType::Two),
        CardType::Tiao(RankType::Three),
        CardType::Tiao(RankType::Four),
        CardType::Tiao(RankType::Five),
        CardType::Tiao(RankType::Six),
        CardType::Tiao(RankType::Seven),
        CardType::Tiao(RankType::Eight),
        CardType::Tiao(RankType::Nine),
        CardType::Tong(RankType::One),
        CardType::Tong(RankType::Two),
        CardType::Tong(RankType::Three),
        CardType::Tong(RankType::Four),
        CardType::Tong(RankType::Five),
        CardType::Tong(RankType::Six),
        CardType::Tong(RankType::Seven),
        CardType::Tong(RankType::Eight),
        CardType::Tong(RankType::Nine),
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

/// Shuffle the mountain using [rand].
fn shuffle(cards: &mut [CardType]) {
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
}

/// Deal out 13 cards to each player.
fn deal(cards: &mut Vec<CardType>) -> Hand {
    cards
        .split_off(cards.len() - 13)
        .into_iter()
        .fold(BTreeMap::new(), |mut map, card| {
            *map.entry(card).or_default() += 1;
            map
        })
}

/// Remove cards in hand that have a zero count for convenience.
fn clean_hand(hand: &mut Hand) {
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
    /// Returns the hand of the current player by mut ref.
    pub fn current_hand_mut(&mut self) -> &mut Hand {
        self.hand_mut(self.active_player)
    }

    /// Returns the hand of a specified player by mut ref.
    pub fn hand_mut(&mut self, side: FengType) -> &mut Hand {
        match side {
            FengType::Dong => &mut self.dong_hand,
            FengType::Nan => &mut self.nan_hand,
            FengType::Xi => &mut self.xi_hand,
            FengType::Bei => &mut self.bei_hand,
        }
    }

    /// Returns the hand of the current player by ref.
    pub fn current_hand(&self) -> &Hand {
        self.hand(self.active_player)
    }

    /// Returns the hand of a specified player by ref.
    pub fn hand(&self, side: FengType) -> &Hand {
        match side {
            FengType::Dong => &self.dong_hand,
            FengType::Nan => &self.nan_hand,
            FengType::Xi => &self.xi_hand,
            FengType::Bei => &self.bei_hand,
        }
    }

    /// Returns the river of the current player by mut ref.
    pub fn current_river_mut(&mut self) -> &mut Vec<RiverType> {
        self.river_mut(self.active_player)
    }

    /// Returns the river of a specified player by mut ref.
    pub fn river_mut(&mut self, side: FengType) -> &mut Vec<RiverType> {
        match side {
            FengType::Dong => &mut self.dong_river,
            FengType::Nan => &mut self.nan_river,
            FengType::Xi => &mut self.xi_river,
            FengType::Bei => &mut self.bei_river,
        }
    }

    /// Returns the river of the current player by ref.
    pub fn current_river(&self) -> &Vec<RiverType> {
        self.river(self.active_player)
    }

    /// Returns the river of a specified player by ref.
    pub fn river(&self, side: FengType) -> &Vec<RiverType> {
        match side {
            FengType::Dong => &self.dong_river,
            FengType::Nan => &self.nan_river,
            FengType::Xi => &self.xi_river,
            FengType::Bei => &self.bei_river,
        }
    }

    /// Returns the open of the current player by mut ref.
    pub fn current_open_mut(&mut self) -> &mut Vec<CaseType> {
        self.open_mut(self.active_player)
    }

    /// Returns the open of a specified player by mut ref.
    pub fn open_mut(&mut self, side: FengType) -> &mut Vec<CaseType> {
        match side {
            FengType::Dong => &mut self.dong_open,
            FengType::Nan => &mut self.nan_open,
            FengType::Xi => &mut self.xi_open,
            FengType::Bei => &mut self.bei_open,
        }
    }

    /// Returns the open of the current player by ref.
    pub fn current_open(&self) -> &Vec<CaseType> {
        self.open(self.active_player)
    }

    /// Returns the open of a specified player by ref.
    pub fn open(&self, side: FengType) -> &Vec<CaseType> {
        match side {
            FengType::Dong => &self.dong_open,
            FengType::Nan => &self.nan_open,
            FengType::Xi => &self.xi_open,
            FengType::Bei => &self.bei_open,
        }
    }

    /// Creates a new [Cards], which contains well initialized states.
    ///
    /// The cards in mountain have been shuffled,
    /// players have been dealt,
    /// and the current active player is 东.
    ///
    /// Note that the banker **HAVE NOT** draw a card to play.
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

    /// Tries to draw a card from mountain.
    /// Returns `None` if there are no more cards in mountain,
    /// or else the card been drawn.
    ///
    /// The hand of the active player will be given the drawn card.
    pub fn draw(&mut self) -> Option<CardType> {
        let res = self.card_mountain.pop()?;
        *self.current_hand_mut().entry(res).or_default() += 1;
        Some(res)
    }

    /// Play a card. If the player want to be in the drawing hand(听牌) state,
    /// the card should be in `RiverType::Drawing`, otherwise `RiverType::Normal`.
    /// The card is automatically added to the player's river.
    ///
    /// Note that it's the caller's duty to ensure the card exists in the active player's hand,
    /// otherwise this function panics.
    ///
    /// # Panics
    ///
    /// This functions panics if the card does not *exist* in the active player's hand.
    ///
    /// Here, the word *exist* means `self.current_hand().contains_key(&card)` returns `false`,
    /// where the `card` is the `CardType` in `RiverType`.
    pub fn play(&mut self, discard: RiverType) {
        let hand = self.current_hand_mut();
        *hand
            .get_mut(&match discard {
                RiverType::Normal(card) | RiverType::Drawing(card) => card,
            })
            .unwrap() -= 1;
        clean_hand(hand);
        self.current_river_mut().push(discard);
    }

    /// Checks if the active player can 暗杠.
    ///
    /// Returns the cards that can do an 暗杠.
    pub fn check_an_gang(&self) -> Vec<CardType> {
        let mut res = Vec::new();
        for (&card, &num) in self.current_hand() {
            if num == 4 {
                res.push(card);
            }
        }
        res
    }

    /// Checks if the active player can 加杠.
    ///
    /// Returns the cards that can do an 加杠.
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

    /// Do a call action, also known as 鸣牌 in Chinese.
    ///
    /// The caller should provide the case(面子) of the call, the player who will do the call, and the card being played, as shown in the function arguments.
    ///
    /// After the call action, the active player, the hand and open of the caller player will be changed if needed.
    ///
    /// Note that the card is not removed from the original player's river.
    ///
    /// # Panics
    ///
    /// It's the caller's duty to ensure that the call action is legal.
    /// Otherwise this function may misbehave and panic.
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

    /// Checks if any other player than the active player can do a call action.
    ///
    /// Returns an array of the case type of the call
    /// and the player who can do it.
    pub fn check_call(&self, card: CardType) -> Vec<(FengType, CaseType)> {
        let mut res = Vec::new();

        let next_side = self.active_player.next();
        let next_hand = self.hand(next_side);
        let lastlast = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num >= RankType::Three =>
            {
                next_hand.keys().copied().find(|c| c.next().next() == card)
            }
            _ => None,
        };
        let last = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num >= RankType::Two =>
            {
                next_hand.keys().copied().find(|c| c.next() == card)
            }
            _ => None,
        };
        let next = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num <= RankType::Eight =>
            {
                next_hand.get_key_value(&card.next()).map(|v| *v.0)
            }
            _ => None,
        };
        let nextnext = match card {
            CardType::Wan(num) | CardType::Tiao(num) | CardType::Tong(num)
                if num <= RankType::Seven =>
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

    /// Checks if the active player can make themselves drawing hand(听牌).
    ///
    /// Returns an array of cards that after which being played
    /// can lead to drawing hand(听牌) state.
    ///
    /// The returned array has been sorted and deduplicated.
    pub fn check_drawing_hand(&self) -> Vec<CardType> {
        let mut discards = self
            .drawing_hand_checkers
            .iter()
            .map(|f| f(self.current_hand(), self.current_open()))
            .flatten()
            .collect::<Vec<_>>();
        discards.sort_unstable();
        discards.dedup();
        discards
    }

    /// Checks if the active player's hand is complete(自摸和牌).
    ///
    /// The caller should provide the card the active player just has drawn.
    ///
    /// Returns true if any completion checkers returns true, otherwise false.
    pub fn check_zi_mo(&self, card: CardType) -> bool {
        self.completion_checkers.iter().any(|f| {
            f(
                self.current_hand(),
                self.current_open(),
                RiverType::Drawing(card),
            )
        })
    }

    /// Checks if any other player will complete(荣和)
    /// given the card the active player has just played.
    /// 
    /// The caller should provide the card the active player just has played.
    /// 
    /// Returns the players who would complete given the card being played.
    pub fn check_dian_pao(&self, card: CardType) -> Vec<FengType> {
        let mut cur_side = self.active_player;
        let mut res = Vec::new();
        for _ in 0..3 {
            cur_side = cur_side.next();
            if self.completion_checkers.iter().any(|f| {
                f(
                    self.hand(cur_side),
                    self.open(cur_side),
                    RiverType::Normal(card),
                )
            }) {
                res.push(cur_side);
            }
        }
        res
    }
}
