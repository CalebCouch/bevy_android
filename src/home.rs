use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

use super::despawn_screen;

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

use crate::menu_plugin;

use crate::NavigateTo;
use crate::theme::icons::Icon;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};

// ==== Components ==== //

use crate::components::{
    balance_display::balance_display,
    navigator::sidebar_navigator,
};


#[derive(Component)]
pub struct OnHomeScreen;

pub fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        OnHomeScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                balance_display(parent, &fonts);
            });

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                }
            ))
            .with_children(|parent| {
    
                let send = primary_default("Send", true);
                let receive = primary_default("Receive", true);
    
                parent.spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|child| {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, send);
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, receive);
                });
            });
        });
    });
}
