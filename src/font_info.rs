use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::AppState;

pub const FONTS_AMOUNT: u32 = 51;

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
    _5x5,
    _6x6,
    _7x7,
    _8x8,
    _9x9,
    _10x10,
    _11x11,
    _12x12,
    _13x13,
    _14x14,
    _15x15,
}

impl FontInfo {
    fn from(name: String, size: FontSize) -> Self {
        Self { name, size }
    }
}

fn setup_fonts(mut commands: Commands) {
    let font_names: Vec<FontInfo> = vec![
        FontInfo::from("Acorn".to_string(), FontSize::_8x8),
        FontInfo::from("Alloy".to_string(), FontSize::_12x12),
        FontInfo::from("Anikki".to_string(), FontSize::_8x8),
        FontInfo::from("Anikki Square".to_string(), FontSize::_10x10),
        FontInfo::from("Buddy".to_string(), FontSize::_10x10),
        FontInfo::from("Buddy Graphical".to_string(), FontSize::_10x10),
        FontInfo::from("CGAThick".to_string(), FontSize::_8x8),
        FontInfo::from("CGAThin".to_string(), FontSize::_8x8),
        FontInfo::from("Cheepicus".to_string(), FontSize::_8x8),
        FontInfo::from("Cheepicus".to_string(), FontSize::_12x12),
        FontInfo::from("Cheepicus".to_string(), FontSize::_14x14),
        FontInfo::from("Cheepicus".to_string(), FontSize::_15x15),
        FontInfo::from("Curses".to_string(), FontSize::_9x9),
        FontInfo::from("DB".to_string(), FontSize::_12x12),
        FontInfo::from("Ddw".to_string(), FontSize::_10x10),
        FontInfo::from("Dullard Exponent".to_string(), FontSize::_12x12),
        FontInfo::from("Geti".to_string(), FontSize::_6x6),
        FontInfo::from("Haberdash".to_string(), FontSize::_12x12),
        FontInfo::from("Herrbdog".to_string(), FontSize::_7x7),
        FontInfo::from("Herrbdog".to_string(), FontSize::_12x12),
        FontInfo::from("Jdpage".to_string(), FontSize::_8x8),
        FontInfo::from("Kein".to_string(), FontSize::_5x5),
        FontInfo::from("Kren".to_string(), FontSize::_13x13),
        FontInfo::from("LN".to_string(), FontSize::_8x8),
        FontInfo::from("Lord Nightmare".to_string(), FontSize::_6x6),
        FontInfo::from("Markvii".to_string(), FontSize::_12x12),
        FontInfo::from("Mkv".to_string(), FontSize::_6x6),
        FontInfo::from("Mkv Solid".to_string(), FontSize::_12x12),
        FontInfo::from("Nice".to_string(), FontSize::_12x12),
        FontInfo::from("Nobbins".to_string(), FontSize::_6x6),
        FontInfo::from("Nostalgia".to_string(), FontSize::_9x9),
        FontInfo::from("Pastiche".to_string(), FontSize::_8x8),
        FontInfo::from("Paul".to_string(), FontSize::_10x10),
        FontInfo::from("Potash".to_string(), FontSize::_8x8),
        FontInfo::from("Potash".to_string(), FontSize::_10x10),
        FontInfo::from("RDE".to_string(), FontSize::_8x8),
        FontInfo::from("Smooth Walls".to_string(), FontSize::_9x9),
        FontInfo::from("Taffer".to_string(), FontSize::_10x10),
        FontInfo::from("Talryth".to_string(), FontSize::_15x15),
        FontInfo::from("Terbert".to_string(), FontSize::_7x7),
        FontInfo::from("Terbert".to_string(), FontSize::_10x10),
        FontInfo::from("Terminus".to_string(), FontSize::_11x11),
        FontInfo::from("Tileset Unknown".to_string(), FontSize::_12x12),
        FontInfo::from("Tocky Square".to_string(), FontSize::_10x10),
        FontInfo::from("Unknown".to_string(), FontSize::_12x12),
        FontInfo::from("Vidumec".to_string(), FontSize::_15x15),
        FontInfo::from("Yayo".to_string(), FontSize::_8x8),
        FontInfo::from("Yayo".to_string(), FontSize::_13x13),
        FontInfo::from("Zaratustra".to_string(), FontSize::_8x8),
        FontInfo::from("Zaratustra".to_string(), FontSize::_5x5),
        FontInfo::from("Zesty".to_string(), FontSize::_12x12),
    ];

    // Who needs sorting when you can type it out
    // font_names.sort_by(|a, b| a.name.cmp(&b.name));

    commands.insert_resource(FontNames { names: font_names });
}
