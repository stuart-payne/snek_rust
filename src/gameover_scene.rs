use std::time::Duration;

use crate::state;
use bevy::prelude::*;
use iyes_loopless::state::NextState;
use iyes_loopless::prelude::*;

pub struct GameoverScene;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub struct RestartTimer;

fn restart_game(mut commands: Commands) {
    println!("HI");
    commands.insert_resource(NextState(state::GameState::Game));
}

impl Plugin for GameoverScene{
    fn build(&self, app:&mut App) {
        let mut restart_timer = SystemStage::parallel();
        restart_timer.add_system(restart_game.run_in_state(state::GameState::Gameover));
        app
            .add_stage_before(
                CoreStage::Update,
                RestartTimer,
                FixedTimestepStage::new(Duration::from_millis(4000)).with_stage(restart_timer)
            );
    }
}
