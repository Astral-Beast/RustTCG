use std::fmt;
use bevy::prelude::*;
pub fn print_vec<T>(vec: &Vec<T>)
where
    T: fmt::Display, //requires Display to be implemented for the type T in the vector
{
    for att in vec {
        println!("{}", att);
    }
}

pub fn setup(
    mut commands: Commands,
    
) {
    commands.spawn(Camera2dBundle::default()); 
}