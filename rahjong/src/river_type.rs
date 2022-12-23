//! River types and their associated methods if any.
//! 
//! River cards are cards that being played in the game, stored in the river array.

use crate::card_type::CardType;

/// The `RiverType` type. See [the module level documentation](self) for more.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RiverType {
    /// The card being played normally.
    /// Also used to indicate that the card is being played when checking completion.
    Normal(CardType),
    /// The card being played as the drawing hand indicator.
    /// Also used to indicate that the card is being drawn when checking completion.
    Drawing(CardType),
}
