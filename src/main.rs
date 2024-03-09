mod colours;
mod ui;

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use std::{cmp::Ordering, collections::HashMap, ops::Range};

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
        .add_plugins(ui::GameUIPlugin)
        .init_resource::<FontSpec>()
        .init_resource::<Game>()
        .init_state::<RunState>()
        .add_systems(Startup, (setup, spawn_board, apply_deferred).chain())
        .add_systems(OnEnter(RunState::Playing), (game_reset, spawn_tiles))
        .add_systems(
            Update,
            (
                render_tile_points,
                board_shift,
                render_tiles,
                new_tile_handler,
                end_game,
            )
                .run_if(in_state(RunState::Playing)),
        )
        .add_event::<NewTileEvent>()
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
#[derive(Default, Resource)]
struct Game {
    score: u32,
    score_best: u32,
}

#[derive(Component)]
struct TileText;

#[derive(Debug, Component, PartialEq)]
struct Points {
    value: u32,
}

#[derive(Debug, Component, PartialEq, Eq, Hash)]
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
        spawn_tile(&mut commands, board, &font_spec, pos);
    }
}

fn spawn_tile(commands: &mut Commands, board: &Board, font_spec: &Res<FontSpec>, pos: Position) {
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
                2.0,
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
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                })
                .insert(TileText);
        })
        .insert(Points { value: 2 })
        .insert(pos);
}

fn render_tiles(
    mut tiles: Query<(&mut Transform, &Position), Changed<Position>>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();

    for (mut transform, pos) in tiles.iter_mut() {
        transform.translation.x = board.cell_position_to_coordinate(pos.x);
        transform.translation.y = board.cell_position_to_coordinate(pos.y);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts.get_mut(*entity).expect("expected Text to exist.");
            let text_section = text
                .sections
                .first_mut()
                .expect("expect first section to be accessible as a mutable");
            text_section.value = points.value.to_string();
        }
    }
}

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::ArrowLeft => Ok(BoardShift::Left),
            KeyCode::ArrowUp => Ok(BoardShift::Up),
            KeyCode::ArrowRight => Ok(BoardShift::Right),
            KeyCode::ArrowDown => Ok(BoardShift::Down),
            _ => Err("not a valid board_shift key"),
        }
    }
}

impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShift::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            BoardShift::Right => match Ord::cmp(&b.y, &a.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
            BoardShift::Up => match Ord::cmp(&b.x, &a.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
            BoardShift::Down => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
        }
    }

    fn set_col(&self, board_size: u8, pos: &mut Mut<Position>, idx: u8) {
        match self {
            BoardShift::Left => {
                pos.x = idx;
            }
            BoardShift::Right => {
                pos.x = board_size - 1 - idx;
            }
            BoardShift::Up => {
                pos.y = board_size - 1 - idx;
            }
            BoardShift::Down => {
                pos.y = idx;
            }
        }
    }

    fn get_row(&self, pos: &Position) -> u8 {
        match self {
            BoardShift::Left | BoardShift::Right => pos.y,
            BoardShift::Up | BoardShift::Down => pos.x,
        }
    }
}

fn board_shift(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let shift_direction = input
        .get_just_pressed()
        .find_map(|key_code| BoardShift::try_from(key_code).ok());

    if let Some(shift) = shift_direction {
        tile_writer.send(NewTileEvent);
        if game.score_best < game.score {
            game.score_best = game.score;
        }

        let mut it = tiles
            .iter_mut()
            .sorted_by(|a, b| shift.sort(&a.1, &b.1))
            .peekable();
        let mut column: u8 = 0;

        let board = query_board.single();

        while let Some(mut tile) = it.next() {
            shift.set_col(board.size, &mut tile.1, column);
            if let Some(tile_next) = it.peek() {
                if shift.get_row(&tile.1) != shift.get_row(&tile_next.1) {
                    // Not in the same row, no merge
                    column = 0;
                } else if tile.2.value != tile_next.2.value {
                    // Not the same value, no merge
                    column = column + 1;
                } else {
                    // Merge
                    // Remove tile from it so we don't evaluate it again
                    let real_next_tile = it
                        .next()
                        .expect("A peeked tile should always exist when next() is called");
                    tile.2.value = tile.2.value + real_next_tile.2.value;

                    // Add to total score
                    game.score += tile.2.value;

                    // Despawn the tile from the board
                    commands.entity(real_next_tile.0).despawn_recursive();

                    if let Some(future) = it.peek() {
                        if shift.get_row(&tile.1) != shift.get_row(&future.1) {
                            // Next tile starts next row
                            column = 0;
                        } else {
                            // Next tile in same row, increment column
                            column = column + 1;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Event)]
struct NewTileEvent;

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();

    for _event in tile_reader.read() {
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = (0..board.size)
            .cartesian_product(0..board.size)
            .filter_map(|tile_pos| {
                let new_pos = Position {
                    x: tile_pos.0,
                    y: tile_pos.1,
                };

                match tiles.iter().find(|pos| **pos == new_pos) {
                    None => Some(new_pos),
                    Some(_) => None,
                }
            })
            .choose(&mut rng);

        if let Some(pos) = possible_position {
            spawn_tile(&mut commands, board, &font_spec, pos)
        }
    }
}

fn end_game(
    tiles: Query<(&Position, &Points)>,
    query_board: Query<&Board>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    let board = query_board.single();

    if tiles.iter().len() == 16 {
        let map: HashMap<&Position, &Points> = tiles.iter().collect();
        let neighbor_points = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let board_range: Range<i8> = 0..(board.size as i8);

        let has_move = tiles.iter().any(|(Position { x, y }, value)| {
            neighbor_points
                .iter()
                .filter_map(|(x2, y2)| {
                    let new_x = *x as i8 - x2;
                    let new_y = *y as i8 - y2;

                    if !board_range.contains(&new_x) || !board_range.contains(&new_y) {
                        return None;
                    }

                    map.get(&Position {
                        x: new_x.try_into().unwrap(),
                        y: new_y.try_into().unwrap(),
                    })
                })
                .any(|&v| v == value)
        });

        if has_move == false {
            dbg!("game over!");
            run_state.set(RunState::GameOver);
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum RunState {
    #[default]
    Playing,
    GameOver,
}

fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }

    game.score = 0;
}
