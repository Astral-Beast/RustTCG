mod deck;
pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    generic_card();
    let mut deck = Deck::default_deck();
    deck.print_deck();
    deck = deck.push_new_card(Card::anticipate_your_foe_card());
    deck = deck.push_new_card(Card::recite_litany_of_respite());
    deck.print_deck();
}

fn generic_card() {
    let mut card = Card::stab_card();
    card.print_card();
    card = card.push_new_attribute(Attribute::Poison(2, 3));
    card.print_card();
}
