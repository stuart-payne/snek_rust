use bevy::ecs::event::Events;
use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::state::GameState;

pub struct Gameover;

pub fn gameover_event_manager(mut events: ResMut<Events<Gameover>>) {
    events.update();
}

pub fn gameover_handler(mut commands: Commands, mut gameover: EventReader<Gameover>) {
    for _ in gameover.iter() {
        println!("Gameover!");
        commands.insert_resource(NextState(GameState::Gameover));
    }
}
