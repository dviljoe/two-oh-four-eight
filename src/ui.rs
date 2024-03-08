mod styles;

use crate::{colours, FontSpec, Game};
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui,))
            .add_systems(Update, (scoreboard,));
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

fn scoreboard(game: Res<Game>, mut query_scores: Query<&mut Text, With<ScoreDisplay>>) {
    let mut text = query_scores.single_mut();
    text.sections[0].value = game.score.to_string();
}
