use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::render::settings::*;
use bevy::render::*;
use std::fmt;

pub fn print_vec<T>(vec: &Vec<T>)
where
    T: fmt::Display, //requires Display to be implemented for the type T in the vector
{
    for att in vec {
        println!("{}", att);
    }
}

pub struct MyRenderPlugin;
/// This plugin is copied from https://github.com/bevyengine/bevy/issues/9975 to remove an issue that was throwing an obscene number of
/// errors while the program was running
impl Plugin for MyRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1920.0, 1080.0).into(),
                        title: "Game".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
