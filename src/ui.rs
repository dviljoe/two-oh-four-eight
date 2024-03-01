use bevy::prelude::*;

use crate::{colours, FontSpec};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui,));
    }
}

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

            // parent
            //     .spawn(NodeBundle {
            //         style: Style {
            //             height: Val::Auto,
            //             width: Val::Auto,
            //             justify_content: JustifyContent::Center,
            //             row_gap: Val::Px(20.0),
            //             column_gap: Val::Px(20.0),
            //             ..default()
            //         },
            //         ..default()
            //     })
            //     .with_children(|parent| {
            //         //scorebox
            //         parent.spawn(NodeBundle {
            //             style: {},
            //             background_color: BackgroundColor(colours::SCORE_BOX),
            //             ..default()
            //         })
            //     });
        });
}
