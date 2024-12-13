use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::Color32;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginPass` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

fn hex(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color32::from_rgb(r, g, b)
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        let button = egui::Button::new("Custom Button")
            .fill(hex("#f3474d"))
            .rounding(egui::Rounding::same(24.0))
            .min_size(egui::Vec2::new(0.0, 48.0));

        if ui.add_sized([ui.available_width(), 48.0], button).clicked() {
             println!("Custom button clicked!");
        }
    });
}
