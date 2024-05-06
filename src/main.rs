mod deck;
mod agent;
mod hex;
pub mod misc;
use agent::{Agent, Condition};
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use bevy::sprite::MaterialMesh2dBundle;

pub use crate::deck::{Attribute, Card, Deck};

fn main() {
    
    App::new()
                .add_plugins(MyRenderPlugin)
                .add_systems(Startup, setup)
                .run();
    
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts\\F25_Bank_Printer.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 10.0,
        color: Color::WHITE,
    };
    let text_justification = JustifyText::Center;
    commands.spawn(Camera2dBundle::default());
    let hexes = hex::build_hexes();
    for i in hexes{
        //i.print_hex();
        commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(RegularPolygon::new(1.0, 6)).into(),
        transform: Transform::default().with_scale(Vec3::splat(hex::HEX_OUTER_RADIUS)).with_translation(i.position),
        material: materials.add(Color::PURPLE),
        ..default()
    });
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                    format!("{}\n{}\n({},{},{})", i.position.x, i.position.y, i.index.x,i.index.y, i.index.z),
                    text_style.clone(),
                    ),

                ], 
                justify:text_justification,
                ..Default::default()
            
            },
            
            
            transform: Transform::default().with_translation(Vec3::new(i.position.x,i.position.y,1.0)),
            ..default()
            },
            ));
}
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
