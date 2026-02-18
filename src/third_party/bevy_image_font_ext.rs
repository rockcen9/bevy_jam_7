use bevy_image_font::ImageFontPlugin;

use crate::prelude::*;

// use bevy_image_font::{
//     ImageFontPlugin, ImageFontText, LetterSpacing, atlas_sprites::ImageFontSpriteText,
// };

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(ImageFontPlugin);
    // app.add_systems(Startup, setup);
}

// fn setup(mut commands: Commands, assets: Res<AssetServer>, palette: ResMut<ColorPalette>) {
//     commands.spawn((
//         ImageFontSpriteText::default()
//             .color(palette.brown_medium_red)
//             .letter_spacing(LetterSpacing::Pixel(2)),
//         ImageFontText::default()
//             .text("32")
//             .font(assets.load("image_font/example_variable_width_font.image_font.ron"))
//             .font_height(36.0),
//         Transform::from_translation(Vec3::new(0.5, 1000., 0.)),
//     ));
// }
