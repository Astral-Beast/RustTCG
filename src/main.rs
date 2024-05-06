mod deck;
mod agent;
mod hex;
pub mod misc;
use agent::{Agent, Condition};
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use hex::build_hexes;


pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    
    App::new()
                .add_plugins(MyRenderPlugin)
                .add_systems(Startup, misc::setup)
                .add_systems(Startup, build_hexes)
                .run();
    
}


//TODO Actual Test Cases
fn test_func(){
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

pub struct MyRenderPlugin;
/// This plugin is copied from https://github.com/bevyengine/bevy/issues/9975 to remove an issue that was throwing an obscene number of
/// errors while the program was running
impl Plugin for MyRenderPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(DefaultPlugins.set(RenderPlugin {
                        render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }).set(WindowPlugin {
                    primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(),
                    title: "Game".to_string(),
                    ..default()
                    }),
                ..default()})
            
            );
	}
}
