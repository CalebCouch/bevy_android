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
};

use crate::theme::icons::Icon;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};
use crate::NavigateTo;
use crate::PageState;
use crate::menu_action;

use crate::settings::{
    OnSettingsMenuScreen,
    settings_menu_setup
};


#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn menu_setup(mut menu_state: ResMut<NextState<PageState>>) {
    menu_state.set(PageState::Main);
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>,) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    let home_button = CustomButton::new(
                        "Bitcoin Home >",
                        None,
                        ButtonStyle::Primary,
                        true,
                        ButtonWidth::Hug,
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
                        ButtonComponent::spawn_button(child, &asset_server, &fonts, home_button);
                    });

                    let settings_button = CustomButton::new(
                        "Settings",
                        None,
                        ButtonStyle::Primary,
                        true,
                        ButtonWidth::Hug,
                        ButtonSize::Large,
                        NavigateTo::Settings,
                    );

                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|child| {
                        ButtonComponent::spawn_button(child, &asset_server, &fonts, settings_button);
                    });
                    
                    let exit_button = CustomButton::new(
                        "Exit",
                        None,
                        ButtonStyle::Primary,
                        true,
                        ButtonWidth::Hug,
                        ButtonSize::Large,
                        NavigateTo::Settings,
                    );

                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|child| {
                        ButtonComponent::spawn_button(child, &asset_server, &fonts, exit_button);
                    });
                    
                });
        });
}

