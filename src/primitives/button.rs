#![allow(unused)]

use bevy::{image::ImageSampler, prelude::*};

use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};

use bevy::ui::BackgroundColor;
use bevy::color::palettes::basic::*;
use bevy::color::palettes::css::*;

use crate::NavigateTo;

#[derive(Copy, Clone, Component, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost,
}

#[derive(Copy, Clone, Component, PartialEq)]
pub enum InteractiveState {
    Default,
    Selected,
    Hover,
    Disabled,
}

pub enum ButtonSize {
    Medium,
    Large,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

pub struct CustomButton {
    label: String,
    icon: Option<Icon>,
    style: ButtonStyle,
    enabled: bool,
    width_style: ButtonWidth,
    size: ButtonSize,
    action: NavigateTo,
}

impl CustomButton {
    pub fn new(
        label: &str,
        icon: Option<Icon>,
        style: ButtonStyle,
        enabled: bool,
        width_style: ButtonWidth,
        size: ButtonSize,
        action: NavigateTo,
    ) -> Self {
        Self {
            label: label.to_string(),
            icon,
            style,
            enabled,
            width_style,
            size,
            action,
        }
    }
}

// #[derive(Component)]
// pub struct NavigationButton(AppState);


// pub fn button_navigation_system(
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &NavigationButton
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut state: ResMut<State<AppState>>,
// ) {
//     for (interaction, nav_button) in &mut interaction_query {
//         if *interaction == Interaction::Pressed {
//             state.set(Box::new(nav_button.0.clone())).unwrap();
//         }
//     }
// }

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Option<&ButtonStyle>,
            &InteractiveState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
   // mut style_query: Query<&ButtonStyle>,
) {
    for (interaction, mut color, mut border_color, children, button_style, state) in &mut interaction_query {
        if *state != InteractiveState::Disabled {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                }
            }
        }
    }
}

pub struct ButtonComponent;

impl ButtonComponent {
    pub fn spawn_button(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        fonts: &Res<FontResources>,
        data: CustomButton,
    ) {
        let status: InteractiveState = if data.enabled == true {InteractiveState::Default} else {InteractiveState::Disabled};
        let colors: ButtonColor = ButtonColor::new(data.style, status);
        let font = fonts.style.label.clone();

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match data.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 16.0, 4.0, fonts.size.md)
        };

        parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline),
            BorderRadius::MAX,
            BackgroundColor(colors.background),
            data.action,
        ))
        .insert(data.style)
        .insert(status)
        .with_children(|button| {
            if let Some(icon) = data.icon {
                button.spawn((
                    Icon::new(data.icon.unwrap(), asset_server),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        margin: UiRect::right(Val::Px(icon_pad)), 
                        ..default()
                    },
                ));
            }
            button.spawn((
                Text::new(data.label),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(colors.label),
            ));     
        });
    }
}

pub fn primary_default(label: &str, enabled: bool) -> CustomButton{
    return CustomButton::new(
        label,
        None,
        ButtonStyle::Primary,
        enabled,
        ButtonWidth::Expand,
        ButtonSize::Large,
        NavigateTo::BackToMainMenu,
    );
}
