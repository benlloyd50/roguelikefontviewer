mod assetload;
use assetload::{AssetLoadPlugin, SpriteAssets, MAX_FONT_POSITION};

use bevy::prelude::*;
use bevy::window::PresentMode;
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
        .add_system(run_game.run_in_state(AppState::GameLoading))
        .add_system(switch_font.run_in_state(AppState::Running))
        .add_system(update_font.run_in_state(AppState::Running))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct CurrentFont(u32);

fn setup_game(mut commands: Commands, sprite: Res<SpriteAssets>) {
    let mut terminal = Terminal::new([80, 50]);
    terminal.put_string(
        [0, 1].pivot(Pivot::TopLeft),
        "Press spacebar/backspace to change fonts",
    );
    terminal.put_string(
        [0, 7].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼",
    );
    terminal.put_string(
        [0, 9].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞",
    );

    terminal.put_string([5, 5], "☺".fg(Color::GREEN));
    let terminal_entity = commands
        .spawn((TerminalBundle::from(terminal), AutoCamera))
        .id();
    commands
        .entity(terminal_entity)
        .insert(TerminalFont::Custom(sprite.acorn.clone()));

    commands.spawn(CurrentFont(0));
}

fn switch_font(keeb: Res<Input<KeyCode>>, mut position_q: Query<&mut CurrentFont>) {
    if keeb.just_pressed(KeyCode::Space) {
        if let Ok(mut position) = position_q.get_single_mut() {
            if position.0 == MAX_FONT_POSITION {
                position.0 = 0;
                return;
            }
            position.0 += 1;
        }
    }
    if keeb.just_pressed(KeyCode::Back) {
        if let Ok(mut position) = position_q.get_single_mut() {
            if position.0 == 0 {
                position.0 = MAX_FONT_POSITION;
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
    terminal_q: Query<(Entity, &Terminal)>,
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
    commands
        .entity(terminal.0)
        .insert(TerminalFont::Custom(sprites.acorn.clone()));

    match font_choice.0 {
        x if x == 0 => {
            commands
                .entity(terminal.0)
                .insert(TerminalFont::Custom(sprites.acorn.clone()));
        }
        x if x == 1 => {
            commands
                .entity(terminal.0)
                .insert(TerminalFont::Custom(sprites.anikki.clone()));
        }
        x if x == 2 => {
            commands
                .entity(terminal.0)
                .insert(TerminalFont::Custom(sprites.cgathin.clone()));
        }
        x if x == 3 => {
            commands
                .entity(terminal.0)
                .insert(TerminalFont::Custom(sprites.cgathick.clone()));
        }
        _ => {
            commands
                .entity(terminal.0)
                .insert(TerminalFont::Custom(sprites.cgathick.clone()));
        }
    };
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
