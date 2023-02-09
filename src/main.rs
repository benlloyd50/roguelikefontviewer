mod assetload;
use assetload::{AssetLoadPlugin, SpriteAssets};
mod font_info;
use font_info::{CurrentFont, FontInfoPlugin, FontNames, FONTS_AMOUNT};

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
        .add_plugin(FontInfoPlugin)
        .add_enter_system(AppState::GameLoading, setup_game)
        .add_system(run_game.run_in_state(AppState::GameLoading))
        .add_system(
            switch_font
                .run_in_state(AppState::Running)
                .label("Input")
                .before("Logic"),
        )
        .add_system(update_font.run_in_state(AppState::Running).label("Logic"))
        .add_system(
            render_screen
                .run_in_state(AppState::Running)
                .label("Render")
                .after("Logic"),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

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

fn render_screen(
    mut terminal_q: Query<&mut Terminal>,
    mut position_q: Query<&CurrentFont>,
    fonts: Res<FontNames>,
) {
    let font_choice = match position_q.get_single_mut() {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };
    let mut terminal = match terminal_q.get_single_mut() {
        Ok(t) => t,
        Err(_) => {
            return;
        }
    };
    terminal.clear();

    terminal.put_string(
        [0, 1].pivot(Pivot::TopLeft),
        "Press [SPACEBAR]/[BACKSPACE] to page through fonts",
    );
    terminal.put_string([0, 2], format!("CurrentFont is {}", font_choice.0));
    terminal.put_string(
        [0, 7].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼",
    );
    terminal.put_string(
        [0, 9].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞",
    );
    terminal.put_string(
        [0, 3],
        format!("Font Name: {}", fonts.names[font_choice.0 as usize].name),
    );

    terminal.put_string([5, 5], "☺".fg(Color::GREEN));
}

fn update_font(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    position_q: Query<&CurrentFont>,
    terminal_q: Query<Entity, With<Terminal>>,
    keeb: Res<Input<KeyCode>>,
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
    let terminal = match terminal_q.get_single() {
        Ok(t) => t,
        Err(_) => {
            panic!("More than one terminal or zero");
        }
    };

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
        .entity(terminal)
        .insert(TerminalFont::Custom(font_asset));
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
