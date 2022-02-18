use crate::grid::Vec2Int;
use crate::{grid, input, rand, constants, apple, events};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Snek;
pub struct SnekQueue(pub VecDeque<Entity>);
pub struct SnekHead(pub Vec2Int);
#[derive(Component)]
pub struct Popped;

pub fn set_snek_color(
    mut query: Query<&mut Sprite, (With<Snek>, Without<Popped>)>,
) {
    for mut mat in query.iter_mut() {
        mat.color = Color::BLACK;
    }
}

pub fn clear_snek(mut snek: ResMut<SnekQueue>) {
    snek.0.clear();
}

pub fn init_snek(
    mut commands: Commands,
    grid: Res<grid::Grid>,
    mut snek: ResMut<SnekQueue>,
    mut snek_head: ResMut<SnekHead>,
    mut last_input: ResMut<input::LastInput>
) {
    let x_len = grid.0.len();
    let y_len = grid.0[0].len();
    let starting_pos = rand::get_rand_coord(0..(x_len - 1) as i32, 0..(y_len - 1) as i32);
    let starting_piece = grid.0[starting_pos.x as usize][starting_pos.y as usize];
    let spawning_dir = rand::get_rand_direction();
    let spawning_offset = grid::translate_direction(spawning_dir);
    commands.entity(starting_piece).insert(Snek);
    snek.0.push_front(starting_piece);

    for num in 1..constants::SNEK_START_AMOUNT {
        let new_pos = grid::get_wrapped_position(starting_pos,spawning_offset * num, &grid); 
        let ent = grid.0[new_pos.x as usize][new_pos.y as usize];
        commands.entity(ent).insert(Snek);
        snek.0.push_back(ent);
    }
    snek_head.0 = starting_pos;
    last_input.0 = grid::get_opposite_direction(spawning_dir);
}

pub fn clean_up_popped(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, With<Snek>, With<Popped>)>,
) {
    for (entity, mut mat, _, _) in query.iter_mut() {
        mat.color = Color::WHITE;
        commands.entity(entity).remove::<Snek>().remove::<Popped>();
    }
}

pub fn move_snek(
    grid: Res<grid::Grid>,
    current_input: Res<input::CurrentInput>,
    mut last_input: ResMut<input::LastInput>,
    mut commands: Commands,
    mut snek: ResMut<SnekQueue>,
    mut snek_head: ResMut<SnekHead>,
    mut spawn_apple: ResMut<apple::SpawnApple>,
    mut ev_gameover: EventWriter<events::Gameover>, 
    query: Query<Entity, With<apple::Apple>>,
) {
    let dir = grid::translate_direction(current_input.0);
    last_input.0 = current_input.0;
    snek_head.0 = grid::get_wrapped_position(dir, snek_head.0, &grid);
    let new_snek_head = grid.0[snek_head.0.x as usize][snek_head.0.y as usize];

    for entity in snek.0.iter() {
        if &new_snek_head == entity {
            ev_gameover.send(events::Gameover);
            return;
        }
    }

    commands.entity(new_snek_head).insert(Snek);
    snek.0.push_front(new_snek_head);

    for entity in query.iter() {
        if new_snek_head == entity {
            commands.entity(entity).remove::<apple::Apple>().insert(apple::Eaten);
            spawn_apple.0 = true;
        }
    }

    if !spawn_apple.0 {
        let popped_entity = snek.0.pop_back().expect("failed to pop entity");
        commands.entity(popped_entity).insert(Popped);
    }
}
