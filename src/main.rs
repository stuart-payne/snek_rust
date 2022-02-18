use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use std::collections::VecDeque;

mod constants;
mod grid;
mod input;
mod snek;
mod rand;
mod apple;
mod events;


#[derive(Component)]
struct Rock;
struct TimeSinceTick(f32);
struct RunStartup(bool);

fn camera_setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.window_origin = WindowOrigin::Center;
    commands.spawn_bundle(camera_bundle);
}


fn tick(mut time_since_tick: ResMut<TimeSinceTick>, time: Res<Time>) -> ShouldRun {
    time_since_tick.0 = time_since_tick.0 + time.delta_seconds();
   if time_since_tick.0 > constants::TICK_RATE {
        time_since_tick.0 = time_since_tick.0 % constants::TICK_RATE;
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn should_run_startup(mut startup: ResMut<RunStartup>) -> ShouldRun {
    match startup.0 {
        false => return ShouldRun::No,
        true => {
            startup.0 = false;
            ShouldRun::Yes
        }
    }
}

fn end_startup(mut startup: ResMut<RunStartup>) {
    startup.0 = false;
}

fn flag_startup(mut startup: ResMut<RunStartup>) {
    startup.0 = true;
}

fn run_cleanup(mut gameover_events: EventReader<events::Gameover>) -> ShouldRun {
    for _ in gameover_events.iter() {
        return ShouldRun::Yes;
    }
    ShouldRun::No
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TimeSinceTick(0.0))
        .insert_resource(grid::Grid(Vec::new()))
        .insert_resource(snek::SnekQueue(VecDeque::new()))
        .insert_resource(input::LastInput(grid::Direction::Up))
        .insert_resource(input::CurrentInput(grid::Direction::Up))
        .insert_resource(input::InputMaps(Vec::new()))
        .insert_resource(snek::SnekHead(grid::Vec2Int::new(0, 0)))
        .insert_resource(apple::SpawnApple(true))
        .insert_resource(RunStartup(true))
        .add_startup_system(input::init_input.system())
        .add_startup_system(camera_setup.system())
        .add_event::<events::Gameover>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(should_run_startup)
                .label("startup")
                .with_system(grid::grid_piece_startup_system.label("grid_startup"))
                .with_system(snek::init_snek.after("grid_startup"))
                .with_system(end_startup)
        )
        .add_system_set(
            SystemSet::new()
                .label("core")
                .with_system(snek::set_snek_color.system())
                .with_system(input::input_system.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(tick.system())
                .label("tick")
                .with_system(snek::move_snek),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_cleanup)
                .label("cleanup")
                .with_system(grid::clear_grid)
                .with_system(snek::clear_snek)
                .with_system(flag_startup)
        )
        .add_system(snek::clean_up_popped.system().after("tick"))
        .add_system(apple::spawn_apple.system().label("apple").with_run_criteria(apple::should_spawn_apple.system()))
        .add_system(apple::set_apple_color.after("apple"))
        .add_system(events::gameover_handler.after("tick"))
        .run();
}
