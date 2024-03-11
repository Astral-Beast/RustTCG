mod agent;
mod deck;
pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    let mut deck = Deck::default_deck();
    deck.print_deck();
    deck = deck.push_new_card(Card::anticipate_your_foe_card());
    deck = deck.push_new_card(Card::recite_litany_of_respite());
    deck.print_deck();
}
