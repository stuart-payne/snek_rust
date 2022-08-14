use crate::{apple, events, grid, input, snek, state};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::{collections::VecDeque, time::Duration};

pub struct GameScene;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub struct Tick;

impl Plugin for GameScene {
    fn build(&self, app: &mut App) {
        let mut tick = SystemStage::parallel();
        tick.add_system(snek::move_snek.run_in_state(state::GameState::Game));

        let mut after_tick = SystemStage::parallel();
        after_tick.add_system(snek::clean_up_popped.run_in_state(state::GameState::Game));

        app.insert_resource(state::TimeSinceTick(0.0))
            .insert_resource(grid::Grid(Vec::new()))
            .insert_resource(snek::SnekQueue(VecDeque::new()))
            .insert_resource(input::LastInput(grid::Direction::Up))
            .insert_resource(input::CurrentInput(grid::Direction::Up))
            .insert_resource(snek::SnekHead(grid::Vec2Int::new(0, 0)))
            .insert_resource(apple::SpawnApple(true))
            .add_stage_before(
                CoreStage::Update,
                Tick,
                FixedTimestepStage::new(Duration::from_millis(1000))
                    .with_stage(tick)
                    .with_stage(after_tick),
            )
            .add_enter_system_set(
                state::GameState::Game,
                SystemSet::new()
                    .with_system(grid::grid_piece_startup_system.label("grid_startup"))
                    .with_system(snek::init_snek.label("snek_init").after("grid_startup")),
            )
            .add_exit_system_set(
                state::GameState::Game,
                SystemSet::new()
                    .with_system(snek::clear_snek.label("clear_snek"))
                    .with_system(grid::clear_grid)
                    .with_system(snek::clean_up_popped.after("clear_snek")),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(state::GameState::Game)
                    .with_system(snek::set_snek_black)
                    .with_system(
                        apple::spawn_apple
                            .run_if(apple::should_spawn_apple)
                            .label("apple"),
                    )
                    .with_system(
                        apple::set_apple_color
                    )
                    .with_system(
                        snek::move_snek
                            .run_if(state::tick)
                            .label("tick"),
                    )
                    .into(),
            )
            .add_event::<events::Gameover>();
    }
}
