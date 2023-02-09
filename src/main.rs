mod assetload;
use assetload::{AssetLoadPlugin, SpriteAssets, FONTS_AMOUNT};

use std::fs;

use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TerminalFont};
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    GameLoading,
    Running,
}

fn main() {
    App::new()
        .add_loopless_state(AppState::AssetLoading) // Starting state which leads to the plugin doing its job first
        .add_plugin(CustomDefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_enter_system(AppState::GameLoading, setup_game)
        .add_enter_system(AppState::GameLoading, setup_fonts)
        .add_system(run_game.run_in_state(AppState::GameLoading))
        .add_system(
            switch_font
                .run_in_state(AppState::Running)
                .label("Input")
                .before("Display"),
        )
        .add_system(update_font.run_in_state(AppState::Running).label("Display"))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct CurrentFont(u32);

fn setup_game(mut commands: Commands, sprite: Res<SpriteAssets>) {
    let terminal = Terminal::new([80, 50]);
    let terminal_entity = commands
        .spawn((TerminalBundle::from(terminal), AutoCamera))
        .id();
    commands
        .entity(terminal_entity)
        .insert(TerminalFont::Custom(sprite.acorn.clone()));

    commands.spawn(CurrentFont(0));
}

#[derive(Resource, Debug)]
struct FontNames {
    names: Vec<FontInfo>,
}

#[derive(Debug)]
struct FontInfo {
    size: String,
    name: String,
}

fn setup_fonts(mut commands: Commands) {
    let paths = fs::read_dir("./assets/images/").unwrap();

    let mut font_names: Vec<FontInfo> = Vec::new();
    for path in paths {
        let font_name = path.unwrap().file_name();
        if let Ok(font) = font_name.into_string() {
            let formatted_name: Vec<&str> = font.split(".").collect();
            font_names.push(FontInfo { size: formatted_name[1].to_string(), name: formatted_name[0].to_string()});
        }
    }
    // font_names.sort_unstable();
    font_names.sort_by(|a, b| a.name.cmp(&b.name));
    println!("{:#?}", font_names);
    commands.insert_resource(FontNames {names: font_names });
}

fn switch_font(keeb: Res<Input<KeyCode>>, mut position_q: Query<&mut CurrentFont>) {
    if let Ok(mut position) = position_q.get_single_mut() {
        if keeb.just_pressed(KeyCode::Space) {
            if position.0 == FONTS_AMOUNT - 1 {
                position.0 = 0;
                return;
            }
            position.0 += 1;
        }
        if keeb.just_pressed(KeyCode::Back) {
            if position.0 == 0 {
                position.0 = FONTS_AMOUNT - 1;
                return;
            }
            position.0 -= 1;
        }
    }
}

fn update_font(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    position_q: Query<&CurrentFont>,
    mut terminal_q: Query<(Entity, &mut Terminal)>,
    keeb: Res<Input<KeyCode>>,
    fonts: Res<FontNames>,
) {
    if !keeb.any_just_pressed([KeyCode::Back, KeyCode::Space, KeyCode::F5]) {
        return;
    }
    let font_choice = match position_q.get_single() {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };
    let mut terminal = match terminal_q.get_single_mut() {
        Ok(t) => t,
        Err(_) => {
            panic!("More than one terminal or zero");
        }
    };
    terminal.1.clear();

    let font_asset = match font_choice.0 {
        x if x == 0 => sprites.acorn.clone(),
        x if x == 1 => sprites.anikki.clone(),
        x if x == 2 => sprites.cgathick.clone(),
        x if x == 3 => sprites.cgathin.clone(),
        x if x == 4 => sprites.cheepicus.clone(),
        x if x == 5 => sprites.jdpage.clone(),
        x if x == 6 => sprites.ln.clone(),
        x if x == 7 => sprites.lordnightmare.clone(),
        x if x == 8 => sprites.pastiche.clone(),
        x if x == 9 => sprites.potash.clone(),
        x if x == 10 => sprites.rde.clone(),
        x if x == 11 => sprites.yayo.clone(),
        x if x == 12 => sprites.zaratustra.clone(),
        _ => sprites.acorn.clone(),
    };
    commands
        .entity(terminal.0)
        .insert(TerminalFont::Custom(font_asset));

    terminal.1.put_string(
        [0, 1].pivot(Pivot::TopLeft),
        "Press [SPACEBAR]/[BACKSPACE] to page through fonts",
    );
    terminal
        .1
        .put_string([0, 2], format!("CurrentFont is {}", font_choice.0));
    terminal.1.put_string(
        [0, 7].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼",
    );
    terminal.1.put_string(
        [0, 9].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞",
    );
    terminal.1.put_string(
        [0, 3], format!("Font Name: {}", fonts.names[font_choice.0 as usize].name)
        );

    terminal.1.put_string([5, 5], "☺".fg(Color::GREEN));
}

// Gets the state out of Game Loading once everything is finished
fn run_game(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::Running));
}

struct CustomDefaultPlugins;
impl Plugin for CustomDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 680.,
                        height: 680.,
                        title: "Font Previewer".to_string(),
                        resizable: false,
                        // Web only settings
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}
