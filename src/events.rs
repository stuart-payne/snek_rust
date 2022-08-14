use bevy::ecs::event::Events;
use bevy::prelude::*;

pub struct Gameover;

pub fn gameover_event_manager(mut events: ResMut<Events<Gameover>>) {
    events.update();
}

pub fn gameover_handler(mut gameover: EventReader<Gameover>) {
    for _ in gameover.iter() {
        println!("Gameover!");
    }
}
