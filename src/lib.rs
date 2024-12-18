#![allow(unused)]

mod menu;
mod home;
mod settings;

pub mod primitives { pub mod button; }
pub mod theme { pub mod icons; pub mod color; pub mod fonts; }
pub mod components { pub mod balance_display; pub mod navigator; }

use bevy::prelude::*;

use theme::{
    color::Colors,
    fonts::setup_fonts
};

use crate::primitives::button::button_system;
use crate::menu::OnMainMenuScreen;
use crate::menu::main_menu_setup;
use crate::menu::menu_setup;

use crate::settings::OnSettingsMenuScreen;
use crate::settings::settings_menu_setup;

use crate::home::{OnHomeScreen, home_setup};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Colors::tapa().shade1000)) 
        .add_plugins((menu_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub enum NavigateTo {
    Home,
    Settings,
    BackToMainMenu,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    Main,
    Home,
    Settings,
    #[default]
    Disabled,
}

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<PageState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(PageState::Main), main_menu_setup)
        .add_systems(OnExit(PageState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(PageState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(PageState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        .add_systems(OnEnter(PageState::Home), home_setup)
        .add_systems(
            OnExit(PageState::Home),
            despawn_screen::<OnHomeScreen>,
        )
        .add_systems(PreStartup, setup_fonts)
        .add_systems(Update, button_system)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &NavigateTo),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<PageState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                NavigateTo::Home => menu_state.set(PageState::Home),
                NavigateTo::Settings => menu_state.set(PageState::Settings),
                NavigateTo::BackToMainMenu => menu_state.set(PageState::Main)
            }
        }
    }
}