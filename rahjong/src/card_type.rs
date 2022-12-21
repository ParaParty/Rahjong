#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum CardType {
    Wan(NumType),
    Tiao(NumType),
    Tong(NumType),
    Zi(ZiType),
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum NumType {
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

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ZiType {
    Jian(JianType),
    Feng(FengType),
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum JianType {
    Bai,
    Fa,
    Zhong,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum FengType {
    Dong,
    Nan,
    Xi,
    Bei,
}

impl CardType {
    pub fn next(&self) -> Self {
        match *self {
            Self::Wan(ref n) => Self::Wan(n.next()),
            Self::Tiao(ref n) => Self::Tiao(n.next()),
            Self::Tong(ref n) => Self::Tong(n.next()),
            Self::Zi(ref zi) => Self::Zi(zi.next()),
        }
    }
}

impl NumType {
    pub fn next(&self) -> Self {
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

impl ZiType {
    pub fn next(&self) -> Self {
        match *self {
            Self::Jian(ref jian) => Self::Jian(jian.next()),
            Self::Feng(ref feng) => Self::Feng(feng.next()),
        }
    }
}

impl JianType {
    pub fn next(&self) -> Self {
        match *self {
            Self::Bai => Self::Fa,
            Self::Fa => Self::Zhong,
            Self::Zhong => Self::Bai,
        }
    }
}

impl FengType {
    pub fn next(&self) -> Self {
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
