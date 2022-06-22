use bevy::prelude::*;

mod game_element;
use game_element::player::PlayerPlugin;

mod component;

// region constants
const COMPUTER_SPRITE: &str = "pc-codeur.png";
const COMPUTER_SIZE: (f32, f32) = (64., 64.);
const SPRITE_SCALE: f32 = 1.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
// endregion

// region resources
pub struct WinSize {
    pub w: f32,
    pub h: f32
}
struct GameTextures {
    computer: Handle<Image>
}
// endregion

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Foyer MEP horror".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // on recupere la taille de la fenetre
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        computer: asset_server.load(COMPUTER_SPRITE)
    };
    commands.insert_resource(game_textures);
}

