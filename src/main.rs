mod colours;

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::IteratorRandom;

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
        .init_resource::<FontSpec>()
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred, spawn_tiles).chain(),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        FontSpec {
            family: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

#[derive(Component)]
struct TileText;

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

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

fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>, font_spec: Res<FontSpec>) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: colours::TILE,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_coordinate(pos.x),
                    board.cell_position_to_coordinate(pos.y),
                    1.0,
                ),
                ..default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            "2",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    })
                    .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}
