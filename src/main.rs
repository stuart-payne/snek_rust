use bevy::ecs::event::Events;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod apple;
mod constants;
mod events;
mod game_scene;
mod grid;
mod input;
mod rand;
mod snek;
mod state;

#[derive(Component)]
struct Rock;

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(game_scene::GameScene)
        .add_loopless_state(state::GameState::Game)
        .insert_resource(input::InputMaps(Vec::new()))
        .add_startup_system(input::init_input)
        .add_startup_system(camera_setup)
        .add_event::<events::Gameover>()
        .add_system(events::gameover_handler.after("tick"))
        .add_system(events::gameover_event_manager)
        .add_system(input::input_system)
        .run();
}
