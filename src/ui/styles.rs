use bevy::prelude::*;

pub fn score_container_style() -> Style {
    Style {
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..Default::default()
    }
}
