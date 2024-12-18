use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;

pub fn balance_display(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
){
    let font = fonts.style.label.clone();
    let font_size = fonts.size.title;
    let colors = Display::new();

    //font_size: txtL <= 4 ? 'title' : txtL <= 7 ? 'h1' : 'h2',

    parent.spawn((
        Text::new("$0.00"),
        TextFont {
            font,
            font_size,
            ..default()
        },
        TextColor(colors.text_heading),
    ));  
}