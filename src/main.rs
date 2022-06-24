use bevy::prelude::*;

mod plugins;
use plugins::{
    player::PlayerPlugin,
    enemy::EnemyPlugin,
    collide_plugin::CollideFireEnemiesPlugin
};

mod component;
use component::{
    velocity::Velocity,
    movable::Movable,
    player::ScoreBugFix
};

// region constants
const COMPUTER_SPRITE: &str = "pc-codeur.png";
const COMPUTER_SIZE: (f32, f32) = (64., 64.);
const ENEMY_SPRITE: &str = "fixme-file.png";
const FILE_LASER_SPRITE: &str = "scala-file.png";
const FILE_LASER_SIZE: (f32, f32) = (64., 64.);
const SPRITE_SCALE: f32 = 1.;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const ENEMY_MAX: u32 = 2; 
// endregion

// region resources
pub struct WinSize {
    pub w: f32,
    pub h: f32
}
struct GameTextures {
    computer: Handle<Image>,
    file_laser: Handle<Image>,
    fixme_file: Handle<Image>
}
struct EnemyCount(u32);

pub struct Scoring {
    pub bug_fix: u32
}

impl Default for Scoring {
    fn default() -> Self {
        Scoring {bug_fix: 0}
    }
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
        .add_plugin(EnemyPlugin)
        .add_plugin(CollideFireEnemiesPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
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
    let pos_score = get_position_score(&win_size);
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        computer: asset_server.load(COMPUTER_SPRITE),
        file_laser: asset_server.load(FILE_LASER_SPRITE),
        fixme_file: asset_server.load(ENEMY_SPRITE)
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
    commands.insert_resource(Scoring::default());


    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "bug fix : 0".to_string(),
                    style: TextStyle {
                        color: Color::ORANGE,
                        font: asset_server.load("COMICATE.TTF"),
                        font_size: 18.
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
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN {
                commands.entity(entity).despawn();
            }
        }
    }
}

