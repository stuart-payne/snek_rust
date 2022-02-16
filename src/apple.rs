use bevy::{prelude::*, ecs::schedule::ShouldRun};
use crate::{grid::GridItem, snek::Snek,};

#[derive(Component)]
pub struct Apple;
#[derive(Component)]
pub struct Eaten;
pub struct SpawnApple(pub bool);

pub fn spawn_apple(mut commands: Commands, query: Query<Entity, (With<GridItem>, Without<Snek>)>) {
    let grid_items = query.iter().map(|e| e).collect::<Vec<Entity>>();
    commands.entity(grid_items[fastrand::usize(0..(grid_items.len() - 1) as usize)]).insert(Apple);
}

pub fn should_spawn_apple(mut spawn_apple: ResMut<SpawnApple>) -> ShouldRun {
    match spawn_apple.0 {
        true => {
            spawn_apple.0 = false;
            ShouldRun::Yes
        },
        false => ShouldRun::No,
    }
}

pub fn set_eaten_apple_color(mut query:Query<&mut Sprite>) {
    for mut mat in query.iter_mut() {
        mat.color = Color::WHITE;
    }
}

pub fn set_apple_color(mut query: Query<&mut Sprite, With<Apple>>) {
   for mut mat in query.iter_mut() {
        mat.color = Color::GREEN;
   }
}
