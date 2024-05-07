mod agent;
mod deck;
mod hex;
mod misc;
use agent::{Agent, Condition};
use bevy::prelude::*;
use hex::build_hexes;

pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    App::new()
        .add_plugins(misc::MyRenderPlugin)
        .add_systems(Startup, (misc::setup, build_hexes))
        //.add_systems(Update, my_cursor_system)
        .run();
}

//TODO Actual Test Cases
fn test_func() {
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
}

// fn my_cursor_system(
//     windows: Query<&Window>,
//     camera_q: Query<(&Camera, &GlobalTransform)>,
//     click: Query<&Interaction, Changed<Interaction>>,
// ) {
//     let window = windows.single();
//     let (camera, camera_transform) = camera_q.single();
//     for  {
//         match click {
//             Interaction::Pressed => {
//                 if let Some(world_position) = window
//                     .cursor_position()
//                     .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//                 {
//                     eprintln!("World coords: {} / {}", world_position.x, world_position.y);
//                 }
//             }
//         }
//     }
// }