mod plugins;
mod resources;
mod component;
mod factory;
mod states;

use bevy::prelude::*;
use plugins::{
    player::PlayerPlugin,
    enemy::EnemyPlugin,
    collide_plugin::CollideFireEnemiesPlugin
};
use component::{
    velocity::Velocity,
    movable::Movable,
    player::ScoreBugFix,
    menu::TextMenu,
    hud::TextTimer
};
use resources::{Timer as MonTimer, WinSize, GameTextures, Scoring};
use factory::texture_factory::*;
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
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollideFireEnemiesPlugin)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(write_menu_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(write_scoring_system)
                .with_system(write_timer_system)
                .with_system(restart_timer_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(on_exit_menu_sytem)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(from_menu_to_game_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movable_system)
                .with_system(update_timer_system)
                .with_system(update_timer_text_system)
        )
        .run();
}

fn from_menu_to_game_system(
    mut app_state: ResMut<State<AppState>>,
    kb: Res<Input<KeyCode>>
) {
    if kb.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn on_exit_menu_sytem(
    mut commands: Commands,
    query: Query<Entity, With<TextMenu>>
) {
    let entity = query.get_single().unwrap();
    commands.entity(entity).despawn();
}   

fn write_scoring_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands, 
    win_size: Res<WinSize>,
) {
    let pos_score = get_position_score(&win_size);

    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "bug fix : 0".to_string(),
                    style: TextStyle {
                        color: Color::GREEN,
                        font: asset_server.load("COMICATE.TTF"),
                        font_size: 36.
                    }
                }],
                /*
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                }
                */
                ..Default::default()
            },
            transform: Transform::from_xyz(pos_score.0, pos_score.1, 1.),
            ..Default::default()
        })
        .insert(ScoreBugFix);

}

fn write_menu_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "press [space] to start".to_string(),
                    style: TextStyle {
                        color: Color::RED,
                        font: asset_server.load("COMICATE.TTF"),
                        font_size: 36.
                    }
                }],
                
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                
                ..Default::default()
            },
            // transform: Transform::from_xyz(pos_score.0, pos_score.1, 1.),
            ..Default::default()
        })
        .insert(TextMenu);
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

fn get_position_score(win_size: &WinSize) -> (f32, f32) {
    (-win_size.w / 2., win_size.h / 2.)
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 300.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn restart_timer_system(mut timer: ResMut<MonTimer>) {
    timer.0 = TIME;
}

fn update_timer_system(
    mut timer: ResMut<MonTimer>,
    time: Res<Time>
) {
    timer.0 -= time.delta_seconds();
}

fn update_timer_text_system(
    timer: ResMut<MonTimer>,
    mut query_timer_text: Query<&mut Text, With<TextTimer>>
) {
    let time_val = match timer.0 {
        a if a < 0. => 0.,
        a => a  
    };

    let mut text = query_timer_text.get_single_mut().unwrap();
    text.sections[0].value = format!("MEP in {:.0} seconds", time_val).to_string();
}

fn write_timer_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>
) {

    let x = 0.;//win_size.w - 256.; 
    let y = win_size.h / 2.; 

    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "MEP in 0 seconds".to_string(),
                    style: TextStyle {
                        color: Color::ORANGE,
                        font: asset_server.load("COMICATE.TTF"),
                        font_size: 36.
                    }
                }],
                /*
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                */
                
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 1.),
            ..Default::default()
        })
        .insert(TextTimer);
}
