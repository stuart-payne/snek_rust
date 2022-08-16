use crate::{apple, constants};
use bevy::prelude::*;
use std::ops::{Add, Mul};

#[derive(Component)]
pub struct GridItem;
pub struct Grid(pub Vec<Vec<Entity>>);

#[derive(Copy, Clone)]
pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

impl Vec2Int {
    pub fn new(x: i32, y: i32) -> Vec2Int {
        Vec2Int { x, y }
    }
}

impl Add for Vec2Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vec2Int {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Bundle)]
pub struct GridPiece {
    grid_item: GridItem,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn grid_piece_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
) {
    println!("GRID INIT");
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("sprites/").unwrap();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    while x < constants::GRID_WIDTH {
        grid.0.push(Vec::new());
        while y < constants::GRID_HEIGHT {
            let x_pos = ((0 - constants::GRID_WIDTH / 2) + x) * constants::PIXELS_PER_UNIT;
            let y_pos = ((0 - constants::GRID_HEIGHT / 2) + y) * constants::PIXELS_PER_UNIT;
            let vec = Vec3::new(x_pos as f32, y_pos as f32, 0.0);
            let sprite = SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                texture: asset_server.load("sprites/WhiteSquare.png"),
                transform: Transform {
                    translation: vec,
                    ..Default::default()
                },
                ..Default::default()
            };

            let entity = commands
                .spawn()
                .insert_bundle(GridPiece {
                    sprite,
                    grid_item: GridItem,
                })
                .id();
            grid.0[x as usize].push(entity);

            y = y + 1;
        }
        x = x + 1;
        y = 0;
    }
}

pub fn despawn_grid(mut commands: Commands,mut grid: ResMut<Grid>, query: Query<Entity, With<GridItem>>,) {
    let len = grid.0.len();
    for mut row in grid.0.drain(0..len) {
        row.drain(0..row.len());
    }
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn translate_direction(dir: Direction) -> Vec2Int {
    match dir {
        Direction::Up => Vec2Int::new(0, 1),
        Direction::Right => Vec2Int::new(1, 0),
        Direction::Down => Vec2Int::new(0, -1),
        Direction::Left => Vec2Int::new(-1, 0),
    }
}

pub fn get_opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

pub fn get_wrapped_position(position: Vec2Int, dir_vec: Vec2Int, grid: &Res<Grid>) -> Vec2Int {
    let mut new_pos = position + dir_vec;
    new_pos.x = wrap(new_pos.x, 0, grid.0.len() as i32);
    new_pos.y = wrap(new_pos.y, 0, grid.0[0].len() as i32);
    new_pos
}

fn wrap(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        return max - value.abs();
    }

    if value >= max {
        return min + (value - max);
    }
    value
}

