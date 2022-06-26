mod plugins;
mod resources;
mod component;
mod factory;
mod states;

use bevy::prelude::*;

use plugins::{
    player::PlayerPlugin,
    enemy::EnemyPlugin,
    collide_plugin::CollideFireEnemiesPlugin,
    endgame_plugin::EndgamePlugin,
    menu_plugin::MenuPlugin,
    ingame_plugin::IngamePlugin
};
use resources::{Timer as MonTimer, WinSize, GameTextures, Scoring};
use factory::texture_factory::create_game_textures;
use states::AppState;

// region constantes
const SPRITE_SIZE: (f32, f32) = (64., 64.);
const SPRITE_SCALE: f32 = 1.;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const ENEMY_MAX: u32 = 2; 
const TIME: f32 = 30.; 
// endregion

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {title: "Foyer MEP horror".to_string(),width: 600.0,height: 600.0,..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_state(AppState::MainMenu) // state de départ
        .add_plugin(EndgamePlugin)
        .add_plugin(IngamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(CollideFireEnemiesPlugin)
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

    let game_textures = create_game_textures(&asset_server);
    commands.insert_resource(game_textures);
    commands.insert_resource(Scoring::default());
    commands.insert_resource(MonTimer(TIME));
}
