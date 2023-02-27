use bevy::prelude::*;

use crate::element::Element;
use crate::resources::CursorOnUI;

pub fn setup_ui(mut commands: Commands) {
    let elements = vec![Element::Sand, Element::Water, Element::Stone];

    for (_i, element) in elements.iter().enumerate() {
        commands
            .spawn((NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(34.), Val::Px(34.)),
                    margin: UiRect {
                        left: Val::Px(10.),
                        top: Val::Px(10.),
                        ..default()
                    },
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(30.), Val::Px(30.)),
                            ..default()
                        },
                        background_color: element.color().into(),
                        ..default()
                    },
                    *element,
                ));
            });
    }
}

pub fn cursor_on_ui(
    mut cursor_on_ui: ResMut<CursorOnUI>,
    query: Query<&Interaction>,
) {
    for i in query.iter() {
        if let Interaction::Clicked | Interaction::Hovered = i {
            cursor_on_ui.0 = true;
            return;
        }
    }

    cursor_on_ui.0 = false;
}

pub fn handle_buttons(
    mut query: Query<(&Interaction, &Element), With<Button>>,
    mut current_element: ResMut<Element>,
) {
    for (interaction, element) in &mut query {
        if let Interaction::Clicked = *interaction {
            *current_element = *element;
        }
    }
}
