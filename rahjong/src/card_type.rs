//! Card types and their associated methods.
//!
//! The core of this module is the [CardType] enum, which, as its name suggests,
//! is the type of a card, contianing both the suit and the rank.

/// The `CardType` type. See [the module level documentation](self) for more.
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum CardType {
    /// The 万 suit. Contains a [RankType] indicating the rank of the card.
    Wan(RankType),
    /// The 条 suit. Contains a [RankType] indicating the rank of the card.
    Tiao(RankType),
    /// The 筒 suit. Contains a [RankType] indicating the rank of the card.
    Tong(RankType),
    /// The 字 suit. Contains a [ZiType] indicating the concrete type of 字.
    Zi(ZiType),
}

/// The `RankType` type. Represents the rank of a card.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum RankType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// The `ZiType` suit. Represents the type of 字, which includes both 箭牌 and 风牌.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ZiType {
    /// The 箭 type. Contains a [JianType] indicating the rank of the card.
    Jian(JianType),
    /// The 风 type. Contains a [FengType] indicating the rank of the card.
    Feng(FengType),
}

/// The `JianType` type. Represents the rank of a card.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum JianType {
    /// The 白 type.
    Bai,
    /// The 发 type.
    Fa,
    /// The 中 type.
    Zhong,
}

/// The `FengType` type. Represents the rank of a card.
/// Also used to represent the player.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum FengType {
    /// The 东 type.
    /// Also the 东 player.
    Dong,
    /// The 南 type.
    /// Also the 南 player.
    Nan,
    /// The 西 type.
    /// Also the 西 player.
    Xi,
    /// The 北 type.
    /// Also the 北 player.
    Bei,
}

/// The trait implemented by card types
/// which contains a function
/// indicating the cycled next card type.
pub trait Next {
    /// The function which indicates the cycled next card type.
    fn next(&self) -> Self;
}

impl Next for CardType {
    fn next(&self) -> Self {
        match *self {
            Self::Wan(ref n) => Self::Wan(n.next()),
            Self::Tiao(ref n) => Self::Tiao(n.next()),
            Self::Tong(ref n) => Self::Tong(n.next()),
            Self::Zi(ref zi) => Self::Zi(zi.next()),
        }
    }
}

impl Next for RankType {
    fn next(&self) -> Self {
        match *self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::One,
        }
    }
}

impl Next for ZiType {
    fn next(&self) -> Self {
        match *self {
            Self::Jian(ref jian) => Self::Jian(jian.next()),
            Self::Feng(ref feng) => Self::Feng(feng.next()),
        }
    }
}

impl Next for JianType {
    fn next(&self) -> Self {
        match *self {
            Self::Bai => Self::Fa,
            Self::Fa => Self::Zhong,
            Self::Zhong => Self::Bai,
        }
    }
}

impl Next for FengType {
    fn next(&self) -> Self {
        match *self {
            Self::Dong => Self::Nan,
            Self::Nan => Self::Xi,
            Self::Xi => Self::Bei,
            Self::Bei => Self::Dong,
        }
    }
}

impl Default for FengType {
    fn default() -> Self {
        Self::Dong
    }
}
