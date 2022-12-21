use crate::card_type::CardType;

#[derive(PartialEq, Eq)]
pub enum CaseType {
    Ke(CardType),
    Shun(CardType),
    Gang(CardType),
    AnGang(CardType),
}
