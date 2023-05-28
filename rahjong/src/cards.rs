//! Card states of the game and their associated methods.
//!
//! The core of this module is the [Cards] struct, which contains the states of the game.

use std::collections::{BTreeMap, HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::{
    card_type::{CardType, FengType, JianType, Next, RankType, ZiType},
    case_type::CaseType,
    river_type::RiverType,
};

type Hand = BTreeMap<CardType, u8>;

type River = Vec<RiverType>;

type Open = Vec<CaseType>;

type SituationChecker = fn(hand: &Hand, river: &River, open: &Open, draw: CardType) -> bool;

/// Checks if a completion is met.
pub struct Completion {
    /// Situations that are needed for the completion.
    ///
    /// All situations are required.
    pub required: Vec<&'static str>,
    /// Situations that prevents the completion.
    ///
    /// All situations must not be met.
    pub forbidden: Vec<&'static str>,
    /// The fan(番数) of the completion.
    pub fan: u16,
    /// The name of the completion.
    pub name: &'static str,
    pub valid: bool,
}

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
    pub dong_river: River,
    /// The cards in player 南's river, shown to other players, known as 牌河 in Chinese.
    pub nan_river: River,
    /// The cards in player 西's river, shown to other players, known as 牌河 in Chinese.
    pub xi_river: River,
    /// The cards in player 北's river, shown to other players, known as 牌河 in Chinese.
    pub bei_river: River,
    /// The cards not in player 东's river, shown to other players, known as 副露 in Chinese.
    pub dong_open: Open,
    /// The cards not in player 南's river, shown to other players, known as 副露 in Chinese.
    pub nan_open: Open,
    /// The cards not in player 西's river, shown to other players, known as 副露 in Chinese.
    pub xi_open: Open,
    /// The cards not in player 北's river, shown to other players, known as 副露 in Chinese.
    pub bei_open: Open,
    /// The player who should play a card.
    pub active_player: FengType,
    /// Functions used to indicate the situations of a player,
    /// including the name of the situation,
    /// and whether the situation is met.
    pub situation_checkers: HashMap<&'static str, SituationChecker>,
    /// Functions used to indicate if the current state satisfies a complete(known as 和牌 in Chinese) condition.
    pub completion_checkers: Vec<Completion>,
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

/// Remove a card from hand.
///
/// Returns if the hand contained the card.
fn remove_from_hand(hand: &mut Hand, card: CardType) -> bool {
    match hand.get_mut(&card) {
        Some(1) => {
            hand.remove(&card);
        }
        Some(0) | None => return false,
        Some(count) => {
            *count -= 1;
        }
    }
    true
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
    /// After this call,
    /// the hand of the active player will have been given the drawn card.
    pub fn draw(&mut self) -> Option<CardType> {
        let res = self.card_mountain.pop()?;
        *self.current_hand_mut().entry(res).or_default() += 1;
        Some(res)
    }

    /// Play a card. If the player want to be in the drawing hand(听牌) state,
    /// the card should be in `RiverType::Drawing`, otherwise `RiverType::Normal`.
    /// The card is automatically added to the player's river.
    ///
    /// Returns whether the card was in hand.
    pub fn play(&mut self, discard: RiverType) -> bool {
        let hand = self.current_hand_mut();
        if !remove_from_hand(
            hand,
            match discard {
                RiverType::Drawing(c) | RiverType::Normal(c) => c,
            },
        ) {
            return false;
        }
        self.current_river_mut().push(discard);
        true
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
    /// Returns whether the call succeeded.
    ///
    /// Will not draw a card.
    pub fn call(
        &mut self,
        case: CaseType,
        side: FengType,
        discard: CardType,
        mut hitchhiker: Vec<CardType>,
    ) -> bool {
        match case {
            CaseType::Shun(start)
                if hitchhiker
                    .iter()
                    .all(|c| self.hand_mut(side).contains_key(c))
                    && {
                        hitchhiker.push(discard);
                        hitchhiker.sort_unstable();
                        hitchhiker.len() == 3
                            && hitchhiker[0].next() == hitchhiker[1]
                            && hitchhiker[1].next() == hitchhiker[2]
                            && hitchhiker[0] < hitchhiker[2]
                            && hitchhiker[0] == start
                    } =>
            {
                let hand = self.hand_mut(side);
                for c in hitchhiker {
                    remove_from_hand(hand, c);
                }

                self.open_mut(side).push(case);
                self.active_player = side;

                true
            }
            CaseType::Ke(card)
                if hitchhiker.len() == 2
                    && hitchhiker.iter().all(|&c| c == card)
                    && card == discard
                    && self.hand_mut(side).get(&card).copied().unwrap_or(0) >= 2 =>
            {
                let hand = self.hand_mut(side);

                remove_from_hand(hand, card);
                remove_from_hand(hand, card);

                self.open_mut(side).push(case);
                self.active_player = side;

                true
            }
            CaseType::Gang(card)
                if hitchhiker.len() == 3
                    && hitchhiker.iter().all(|&c| c == card)
                    && card == discard
                    && self.hand_mut(side).get(&card).copied().unwrap_or(0) >= 3 =>
            {
                let hand = self.hand_mut(side);
                hand.remove(&card);
                self.open_mut(side).push(case);
                self.active_player = side;

                true
            }
            CaseType::Gang(card)
                if hitchhiker.len() == 4
                    && hitchhiker.iter().all(|&c| c == card)
                    && card == discard
                    && self.current_hand().contains_key(&card)
                    && side == self.active_player =>
            {
                if let Some(case) = self
                    .open_mut(side)
                    .into_iter()
                    .find(|&&mut o| o == CaseType::Ke(card))
                {
                    *case = CaseType::Gang(card);
                    self.current_hand_mut().remove(&card);
                    true
                } else {
                    false
                }
            }
            CaseType::AnGang(card)
                if hitchhiker.len() == 4
                    && hitchhiker.into_iter().all(|c| c == card)
                    && card == discard
                    && self.current_hand().get(&card).copied().unwrap_or(0) == 4
                    && side == self.active_player =>
            {
                let hand = self.current_hand_mut();
                hand.remove(&card);
                self.current_open_mut().push(case);

                true
            }
            _ => false,
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

    /// Checks if side wins.
    ///
    /// Returns the completions.
    pub fn win(&self, side: FengType, last_card: CardType) -> impl Iterator<Item = &Completion> {
        let situations: HashSet<_> = self
            .situation_checkers
            .iter()
            .filter(|(_, f)| {
                f(
                    self.hand(side),
                    self.river(side),
                    self.open(side),
                    last_card,
                )
            })
            .map(|t| *t.0)
            .collect();
        self.completion_checkers.iter().filter(move |item| {
            item.required.iter().all(|r| situations.contains(r))
                && !item.forbidden.iter().any(|f| situations.contains(f))
        })
    }

    // /// Checks if the active player can make themselves drawing hand(听牌).
    // ///
    // /// Returns an array of cards that after which being played
    // /// can lead to drawing hand(听牌) state.
    // ///
    // /// The returned array has been sorted and deduplicated.
    // pub fn check_drawing_hand(&self) -> Vec<CardType> {
    //     let mut discards = self
    //         .drawing_hand_checkers
    //         .iter()
    //         .map(|f| f(self.current_hand(), self.current_open()))
    //         .flatten()
    //         .collect::<Vec<_>>();
    //     discards.sort_unstable();
    //     discards.dedup();
    //     discards
    // }

    // /// Checks if the active player's hand is complete(自摸和牌).
    // ///
    // /// The caller should provide the card the active player just has drawn.
    // ///
    // /// Returns true if any completion checkers returns true, otherwise false.
    // pub fn check_zi_mo(&self, card: CardType) -> bool {
    //     self.completion_checkers.iter().any(|f| {
    //         f(
    //             self.current_hand(),
    //             self.current_open(),
    //             RiverType::Drawing(card),
    //         )
    //     })
    // }

    // /// Checks if any other player will complete(荣和)
    // /// given the card the active player has just played.
    // ///
    // /// The caller should provide the card the active player just has played.
    // ///
    // /// Returns the players who would complete given the card being played.
    // pub fn check_dian_pao(&self, card: CardType) -> Vec<FengType> {
    //     let mut cur_side = self.active_player;
    //     let mut res = Vec::new();
    //     for _ in 0..3 {
    //         cur_side = cur_side.next();
    //         if self.completion_checkers.iter().any(|f| {
    //             f(
    //                 self.hand(cur_side),
    //                 self.open(cur_side),
    //                 RiverType::Normal(card),
    //             )
    //         }) {
    //             res.push(cur_side);
    //         }
    //     }
    //     res
    // }
}
