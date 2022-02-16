use crate::grid::{ Direction, get_opposite_direction };
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

pub struct LastInput(pub Direction);
pub struct CurrentInput(pub Direction);
pub struct InputMap(pub KeyCode, pub Direction);
pub struct InputMaps(pub Vec<InputMap>);

pub fn init_input(mut input_maps: ResMut<InputMaps>) {
    input_maps.0 = Vec::from([
        InputMap(KeyCode::W, Direction::Up),
        InputMap(KeyCode::D, Direction::Right),
        InputMap(KeyCode::S, Direction::Down),
        InputMap(KeyCode::A, Direction::Left),
    ]);
}

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    last_input: ResMut<LastInput>,
    mut current_input: ResMut<CurrentInput>,
    input_maps: Res<InputMaps>,
) {
    for input_map in input_maps.0.iter() {
        if keyboard_input.just_pressed(input_map.0) {
            // do not set input if it is in the opposite direction of the 
            if get_opposite_direction(input_map.1) == last_input.0 {
                return;
            }
            current_input.0 = input_map.1;
            println!("{:#?}", input_map.1);
        }
    }
}
