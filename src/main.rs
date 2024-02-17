mod colours;

use bevy::prelude::*;
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048: Two-oh-four-eight".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_board))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
struct Board {
    size: u8,
}

fn spawn_board(mut commands: Commands) {
    let board = Board { size: 4 };
    let board_size = f32::from(board.size) * TILE_SIZE + f32::from(board.size + 1) * TILE_SPACER;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colours::BOARD,
                custom_size: Some(Vec2::new(board_size, board_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            let tile_offset = (-board_size / 2.0) + (0.5 * TILE_SIZE);
            let tiles = (0..board.size).cartesian_product(0..board.size);

            for tile in tiles {
                let tile_x = tile_offset
                    + f32::from(tile.0) * TILE_SIZE
                    + f32::from(tile.0 + 1) * TILE_SPACER;
                let tile_y = tile_offset
                    + f32::from(tile.1) * TILE_SIZE
                    + f32::from(tile.1 + 1) * TILE_SPACER;

                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colours::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(tile_x, tile_y, 1.0),
                    ..default()
                });
            }
        })
        .insert(board);
}
