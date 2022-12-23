//! Case types and their associated methods if any.
//! 
//! Cases are known as 面子 in Chinese, which represents the cards being well formed, shown to other players but not in the river.

use crate::card_type::CardType;

/// The `CaseType` type, or known as 面子 in Chinese. See [the module level documentation](self) for more.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CaseType {
    /// The 刻子 type. Contains a [CardType] indicating the card of 刻子.
    Ke(CardType),
    /// The 顺子 type. Contains a [CardType] indicating the starting number card of 顺子.
    Shun(CardType),
    /// The 杠子 type. Contains a [CardType] indicating the card of 杠子.
    ///
    /// This type represents both 大明杠 and 加杠, but not 暗杠.
    Gang(CardType),
    /// The 杠子 type specially for 暗杠. Contains a [CardType] indicating the card of 杠子.
    AnGang(CardType),
}
