use crate::card_type::CardType;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CaseType {
    Ke(CardType),
    Shun(CardType),
    Gang(CardType),
    AnGang(CardType),
}
