mod deck;
mod agent;
pub mod misc;
use agent::{Agent, Condition};

pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    let mut deck = Deck::default_deck();
    deck.print_deck();
    deck = deck.push_new_card(Card::anticipate_your_foe_card());
    deck = deck.push_new_card(Card::recite_litany_of_respite_card());
    deck.print_deck();
    let mut player = Agent::player_start();
    player = player.add_condition(Condition::on_fire());
    player.print_agent();
    player = player.add_condition(Condition::on_fire());
    player.print_agent();
    //println!(player.body)
}
