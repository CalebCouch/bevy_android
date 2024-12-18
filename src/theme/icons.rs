#![allow(unused)]
use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum Icon {
    Exit,
    Left,
    Right,
    Wallet,
    Message,
}

impl Icon {
    pub fn new(self, asset_server: &Res<AssetServer>) -> ImageNode {
        let choice = match self {
            Icon::Exit => "exit.png",
            Icon::Left => "left.png",
            Icon::Right => "right.png",
            Icon::Wallet => "wallet.png",
            Icon::Message => "message.png",
        };
        ImageNode::new(asset_server.load(choice))
    }

    // pub fn spawn_icon(&self, commands: &mut Commands, asset_server: &AssetServer, size: f32) -> Entity {
    //     let texture_handle = asset_server.load(self.texture_path());

    //     commands.spawn_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(size, size)), 
    //             ..Default::default()
    //         },
    //         texture: texture_handle,
    //         ..Default::default()
    //     }).id()
    // }
}
