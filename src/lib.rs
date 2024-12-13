use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    log::info!("Started");

    App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugins(EguiPlugin)
    .insert_resource(ClearColor(Color::rgb(214.0 / 255.0, 204.0 / 255.0, 185.0 / 255.0)))
    .run();

    log::info!("FINISHED ERROR");
  //loop {
  //    println!("Hello, world!");
  //    log::info!("HI");
  //}
}
