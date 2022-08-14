use crate::constants;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Game,
    Gameover,
}

pub struct TimeSinceTick(pub f32);

pub fn tick(mut time_since_tick: ResMut<TimeSinceTick>, time: Res<Time>) -> bool {
    time_since_tick.0 = time_since_tick.0 + time.delta_seconds();
    if time_since_tick.0 > constants::TICK_RATE {
        time_since_tick.0 = time_since_tick.0 % constants::TICK_RATE;
        true
    } else {
        false
    }
}
