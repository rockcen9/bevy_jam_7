#![allow(warnings)]
use crate::prelude::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<ColorPalette>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiColorName {
    LabelText,
    HeaderText,
    ButtonText,
    ButtonBackground,
    ButtonHoveredBackground,
    ButtonPressedBackground,
    ScreenBackground,
    FightButtonText,
    FightButtonBackground,
    FightButtonBorder,
}

/// Daifuku Delights 24 - A very delicate palette for pastel art
#[derive(Resource, Debug, Clone)]
pub struct ColorPalette {
    // Pink/Purple shades
    pub pink_dark: Color,
    pub pink_medium: Color,
    pub pink_light: Color,
    pub purple_light: Color,
    pub purple_lighter: Color,
    pub purple_lightest: Color,

    // Blue shades
    pub blue_lightest: Color,
    pub blue_lighter: Color,
    pub blue_light: Color,
    pub blue_medium: Color,
    pub blue_dark: Color,
    pub blue_darkest: Color,

    // Brown/Tan shades
    pub brown_dark: Color,
    pub brown_reddish: Color,
    pub brown_medium_red: Color,
    pub brown_medium: Color,
    pub brown_light: Color,
    pub tan_medium: Color,
    pub tan_light: Color,
    pub tan_lightest: Color,

    // Green shades
    pub green_yellow: Color,
    pub green_light: Color,
    pub green_medium: Color,
    pub green_dark: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Pink/Purple shades (hex values from daifuku-delights-24.txt)
            pink_dark: Color::srgba_u8(0xab, 0x65, 0x81, 0xff), // #ab6581
            pink_medium: Color::srgba_u8(0xbb, 0x72, 0x9f, 0xff), // #bb729f
            pink_light: Color::srgba_u8(0xca, 0x7f, 0xbc, 0xff), // #ca7fbc
            purple_light: Color::srgba_u8(0xd7, 0x9d, 0xd9, 0xff), // #d79dd9
            purple_lighter: Color::srgba_u8(0xe4, 0xbb, 0xf7, 0xff), // #e4bbf7
            purple_lightest: Color::srgba_u8(0xf1, 0xe4, 0xfd, 0xff), // #f1e4fd

            // Blue shades
            blue_lightest: Color::srgba_u8(0xbb, 0xd1, 0xee, 0xff), // #bbd1ee
            blue_lighter: Color::srgba_u8(0xa5, 0xb7, 0xe2, 0xff),  // #a5b7e2
            blue_light: Color::srgba_u8(0x8f, 0x9e, 0xd5, 0xff),    // #8f9ed5
            blue_medium: Color::srgba_u8(0x74, 0x8a, 0xb2, 0xff),   // #748ab2
            blue_dark: Color::srgba_u8(0x58, 0x77, 0x8e, 0xff),     // #58778e
            blue_darkest: Color::srgba_u8(0x3d, 0x63, 0x6b, 0xff),  // #3d636b

            // Brown/Tan shades
            brown_dark: Color::srgba_u8(0x5e, 0x4f, 0x5d, 0xff), // #5e4f5d
            brown_reddish: Color::srgba_u8(0x9c, 0x58, 0x64, 0xff), // #9c5864
            brown_medium_red: Color::srgba_u8(0xad, 0x6d, 0x66, 0xff), // #ad6d66
            brown_medium: Color::srgba_u8(0xbd, 0x82, 0x69, 0xff), // #bd8269
            brown_light: Color::srgba_u8(0xce, 0x97, 0x6b, 0xff), // #ce976b
            tan_medium: Color::srgba_u8(0xdb, 0xb5, 0x7a, 0xff), // #dbb57a
            tan_light: Color::srgba_u8(0xe7, 0xd3, 0x88, 0xff),  // #e7d388
            tan_lightest: Color::srgba_u8(0xf1, 0xdf, 0xc1, 0xff), // #f1dfc1

            // Green shades
            green_yellow: Color::srgba_u8(0xc3, 0xc3, 0x80, 0xff), // #c3c380
            green_light: Color::srgba_u8(0x9f, 0xb2, 0x78, 0xff),  // #9fb278
            green_medium: Color::srgba_u8(0x78, 0x9a, 0x73, 0xff), // #789a73
            green_dark: Color::srgba_u8(0x5b, 0x7f, 0x6f, 0xff),   // #5b7f6f
        }
    }
}
impl ColorPalette {
    pub fn get(&self, ui_type: UiColorName) -> Color {
        match ui_type {
            // Warm yellow-tan for readable labels
            UiColorName::LabelText => self.tan_light,
            // Brightest tan for prominent headers
            UiColorName::HeaderText => self.tan_lightest,
            // Dark brown for legible text on light buttons
            UiColorName::ButtonText => self.brown_dark,
            // Warm cream for button base
            UiColorName::ButtonBackground => self.tan_lightest,
            // Dreamy pink for hover state
            UiColorName::ButtonHoveredBackground => self.pink_light,
            // Slightly darker tan for pressed state
            UiColorName::ButtonPressedBackground => self.tan_medium,
            // Darkest color for non-distracting background
            UiColorName::ScreenBackground => self.brown_dark,
            // Fight button colors - using reddish tones for emphasis
            UiColorName::FightButtonText => self.purple_lightest,
            UiColorName::FightButtonBackground => self.brown_reddish,
            UiColorName::FightButtonBorder => self.pink_medium,
        }
    }
}
