use crate::card_type::CardType;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RiverType {
    Normal(CardType),
    Drawing(CardType),
}
