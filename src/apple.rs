use crate::{grid::GridItem, snek::Snek};
use bevy::prelude::*;

#[derive(Component)]
pub struct Apple;
#[derive(Component)]
pub struct Eaten;
pub struct SpawnApple(pub bool);

pub fn spawn_apple(mut commands: Commands, query: Query<Entity, (With<GridItem>, Without<Snek>)>) {
    let grid_items = query.iter().map(|e| e).collect::<Vec<Entity>>();
    if grid_items.len() == 0 {
        println!("EARLY EXIT");
        return;
    }
    commands
        .entity(grid_items[fastrand::usize(0..(grid_items.len()) as usize)])
        .insert(Apple);
}

pub fn should_spawn_apple(mut spawn_apple: ResMut<SpawnApple>) -> bool {
    match spawn_apple.0 {
        true => {
            spawn_apple.0 = false;
            true
        }
        false => false,
    }
}

// pub fn set_eaten_apple_color(
//     mut commands: Commands,
//     mut query: Query<(&mut Sprite, Entity), With<Eaten>>,
// ) {
//     for (mut mat, entity) in query.iter_mut() {
//         mat.color = Color::WHITE;
//         commands.entity(entity).remove::<Eaten>();
//     }
// }

pub fn set_apple_color(mut query: Query<&mut Sprite, With<Apple>>) {
    for mut mat in query.iter_mut() {
        if mat.color != Color::GREEN {
            mat.color = Color::GREEN;
        }
    }
}
