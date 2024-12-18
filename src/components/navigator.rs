use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    button_system,
    primary_default,
};

use crate::theme::icons::Icon;

use crate::NavigateTo;

pub fn sidebar_navigator (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>, 
) {

    let font = fonts.style.label.clone();
    let font_size = fonts.size.title;
    let colors = Display::new();

    parent.spawn((
        Node {
            width: Val::Px(250.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        }
    ))
    .with_children(|parent| {

        let wallet = CustomButton::new(
            "Bitcoin",
            Some(Icon::Wallet),
            ButtonStyle::Ghost,
            true,
            ButtonWidth::Expand,
            ButtonSize::Large,
            NavigateTo::Home,
        );

        parent.spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|child| {
            ButtonComponent::spawn_button(child, &asset_server, &fonts, wallet);
        });
    });
}