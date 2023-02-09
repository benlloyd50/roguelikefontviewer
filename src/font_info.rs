use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::AppState;

pub const FONTS_AMOUNT: u32 = 13;

pub struct FontInfoPlugin;

impl Plugin for FontInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::GameLoading, setup_fonts);
    }
}

#[derive(Component)]
pub struct CurrentFont(pub u32);

#[derive(Resource, Debug)]
pub struct FontNames {
    pub names: Vec<FontInfo>,
}

#[derive(Debug)]
pub struct FontInfo {
    pub name: String,
    size: FontSize,
}

#[derive(Debug)]
enum FontSize {
    _6x6,
    _8x8,
}

impl FontInfo {
    fn from(name: String, size: FontSize) -> Self {
        Self { name, size }
    }
}

fn setup_fonts(mut commands: Commands) {
    let mut font_names: Vec<FontInfo> = vec![
        FontInfo::from("Acorn".to_string(), FontSize::_8x8),
        FontInfo::from("Anikki".to_string(), FontSize::_8x8),
        FontInfo::from("CGAThick".to_string(), FontSize::_8x8),
        FontInfo::from("CGAThin".to_string(), FontSize::_8x8),
        FontInfo::from("Cheepicus".to_string(), FontSize::_8x8),
        FontInfo::from("Jdpage".to_string(), FontSize::_8x8),
        FontInfo::from("LN".to_string(), FontSize::_8x8),
        FontInfo::from("LordNightmare".to_string(), FontSize::_6x6),
        FontInfo::from("Pastiche".to_string(), FontSize::_8x8),
        FontInfo::from("Potash".to_string(), FontSize::_8x8),
        FontInfo::from("RDE".to_string(), FontSize::_8x8),
        FontInfo::from("Yayo".to_string(), FontSize::_8x8),
        FontInfo::from("Zaratustra".to_string(), FontSize::_8x8),
    ];

    font_names.sort_by(|a, b| a.name.cmp(&b.name));

    commands.insert_resource(FontNames { names: font_names });
}
