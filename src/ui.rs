mod styles;

use crate::{colours, FontSpec, Game, RunState};
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui,))
            .add_systems(Update, (scoreboard, button_interaction, button_text));
    }
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

fn setup_ui(mut commands: Commands, font_spec: Res<FontSpec>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Two-oh-four-eight",
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Auto,
                        width: Val::Auto,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(20.0),
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // begin scorebox
                    parent
                        .spawn(NodeBundle {
                            style: styles::score_container_style(),
                            background_color: BackgroundColor(colours::SCORE_BOX),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Score",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                                ScoreDisplay,
                            ));
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn(NodeBundle {
                            style: styles::score_container_style(),
                            background_color: BackgroundColor(colours::SCORE_BOX),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Best",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );

                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                                BestScoreDisplay,
                            ));
                        });
                    // end best scorebox
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(130.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: Query<&mut Text, (With<ScoreDisplay>, Without<BestScoreDisplay>)>,
    mut query_best_scores: Query<&mut Text, (With<BestScoreDisplay>, Without<ScoreDisplay>)>,
) {
    let mut text = query_scores.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut text = query_best_scores.single_mut();
    text.sections[0].value = game.score_best.to_string();
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    run_state: Res<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *color = colours::button::PRESSED.into();

                match run_state.get() {
                    RunState::Playing => {
                        next_state.set(RunState::GameOver);
                    }

                    RunState::GameOver => {
                        next_state.set(RunState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = colours::button::HOVERED.into();
            }
            Interaction::None => {
                *color = colours::button::NORMAL.into();
            }
        }
    }
}

fn button_text(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query.single();
    let first_child_entity = children
        .first()
        .expect("expect button to have a first child");

    let mut text = text_query.get_mut(*first_child_entity).unwrap();

    match run_state.get() {
        RunState::Playing => {
            text.sections[0].value = "End Game".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
    }
}
