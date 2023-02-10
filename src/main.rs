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
        [0, 3].pivot(Pivot::TopLeft),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    );
    terminal.put_string(
        [0, 5].pivot(Pivot::TopLeft),
        "abcdefghijklmnopqrstuvwxyz",
    );
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
        x if x == 1 => sprites.alloy.clone(),
        x if x == 2 => sprites.anikki.clone(),
        x if x == 3 => sprites.anikkisquare.clone(),
        x if x == 4 => sprites.buddy.clone(),
        x if x == 5 => sprites.buddygraphical.clone(),
        x if x == 6 => sprites.cgathick.clone(),
        x if x == 7 => sprites.cgathin.clone(),
        x if x == 8 => sprites.cheepicus8.clone(),
        x if x == 9 => sprites.cheepicus12.clone(),
        x if x == 10 => sprites.cheepicus14.clone(),
        x if x == 11 => sprites.cheepicus15.clone(),
        x if x == 12 => sprites.curses.clone(),
        x if x == 13 => sprites.db.clone(),
        x if x == 14 => sprites.ddw.clone(),
        x if x == 15 => sprites.dullard.clone(),
        x if x == 16 => sprites.geti.clone(),
        x if x == 17 => sprites.haberdash.clone(),
        x if x == 18 => sprites.herrbdog7.clone(),
        x if x == 19 => sprites.herrbdog12.clone(),
        x if x == 20 => sprites.jdpage.clone(),
        x if x == 21 => sprites.kein.clone(),
        x if x == 22 => sprites.kren.clone(),
        x if x == 23 => sprites.ln.clone(),
        x if x == 24 => sprites.lordnightmare.clone(),
        x if x == 25 => sprites.markvii.clone(),
        x if x == 26 => sprites.mkv.clone(),
        x if x == 27 => sprites.mkvsolid.clone(),
        x if x == 28 => sprites.nice.clone(),
        x if x == 29 => sprites.nobbins.clone(),
        x if x == 30 => sprites.nostalgia.clone(),
        x if x == 31 => sprites.pastiche.clone(),
        x if x == 32 => sprites.paul.clone(),
        x if x == 33 => sprites.potash8.clone(),
        x if x == 34 => sprites.potash10.clone(),
        x if x == 35 => sprites.rde.clone(),
        x if x == 36 => sprites.smoothwalls.clone(),
        x if x == 37 => sprites.taffer.clone(),
        x if x == 38 => sprites.talryth.clone(),
        x if x == 39 => sprites.terbert7.clone(),
        x if x == 40 => sprites.terbert10.clone(),
        x if x == 41 => sprites.terminus.clone(),
        x if x == 42 => sprites.tilesetunknown.clone(),
        x if x == 43 => sprites.tocky.clone(),
        x if x == 44 => sprites.unknown.clone(),
        x if x == 45 => sprites.vidumec.clone(),
        x if x == 46 => sprites.yayo8.clone(),
        x if x == 47 => sprites.yayo13.clone(),
        x if x == 48 => sprites.zaratustra5.clone(),
        x if x == 49 => sprites.zaratustra8.clone(),
        x if x == 50 => sprites.zesty.clone(),
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
                        // width: 680.,
                        // height: 680.,
                        title: "Font Previewer".to_string(),
                        // resizable: false,
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
