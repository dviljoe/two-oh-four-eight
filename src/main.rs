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
    size_px: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let size_px = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board { size, size_px }
    }

    fn sprite_size(&self) -> Vec2 {
        Vec2::new(self.size_px, self.size_px)
    }

    fn cell_position_to_coordinate(&self, pos: u8) -> f32 {
        let bottom_left = (-self.size_px / 2.0) + (0.5 * TILE_SIZE);
        bottom_left + (f32::from(pos) * TILE_SIZE) + (f32::from(pos + 1) * TILE_SPACER)
    }
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colours::BOARD,
                custom_size: Some(board.sprite_size()),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            let tiles = (0..board.size).cartesian_product(0..board.size);

            for tile in tiles {
                let tile_x = board.cell_position_to_coordinate(tile.0);
                let tile_y = board.cell_position_to_coordinate(tile.1);

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
