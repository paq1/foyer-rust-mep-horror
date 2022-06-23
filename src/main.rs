use std::collections::HashSet;
use std::iter::FromIterator;
use bevy::prelude::*;

mod game_element;
use game_element::{
    player::PlayerPlugin,
    enemy::EnemyPlugin
};

mod component;
use component::{
    velocity::Velocity,
    movable::Movable,
    enemy::Enemy,
    sprite_size::SpriteSize,
    laser::Laser
};

use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;

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
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_file_hit_enemy_system)
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
        computer: asset_server.load(COMPUTER_SPRITE),
        file_laser: asset_server.load(FILE_LASER_SPRITE),
        fixme_file: asset_server.load(ENEMY_SPRITE)
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
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

fn player_file_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), With<Laser>>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) {
    laser_query
        .iter()
        .fold(
            HashSet::new(), 
            |acc, current| 
            handle_collide_laser_enemies(&mut commands, &mut enemy_count, &acc, current, &enemy_query)
        );
}

fn handle_collide_laser_enemies(
    commands: &mut Commands,
    mut enemy_count: &mut ResMut<EnemyCount>,
    despawned_entities: &HashSet<Entity>, 
    laser_info: (Entity, &Transform, &SpriteSize),
    enemy_query: &Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) -> HashSet<Entity> {
    enemy_query
        .iter()
        .fold(
            despawned_entities.clone(), 
            |acc: HashSet<Entity>, current| 
            handle_collide_laser_enemy(commands, &mut enemy_count, acc, laser_info, current)
        )
}

fn handle_collide_laser_enemy(
    commands: &mut Commands,
    enemy_count: &mut ResMut<EnemyCount>,
    despawned_entities: HashSet<Entity>, 
    laser_info: (Entity, &Transform, &SpriteSize),
    enemy_info: (Entity, &Transform, &SpriteSize)
) -> HashSet<Entity> {
    let (laser_entity, laser_tf, laser_size) = laser_info;
    let (enemy_entity, enemy_tf, enemy_size) = enemy_info;

    let laser_scale = Vec2::from(laser_tf.scale.xy());
    let enemy_scale = Vec2::from(enemy_tf.scale.xy());

    let collision = collide(
        laser_tf.translation,
        laser_size.0 * laser_scale,
        enemy_tf.translation,
        enemy_size.0 * enemy_scale
    );

    if let Some(_) = collision {
        // ces checks permettent de ne pas delete deux fois une meme entité dans le cas d'une collision double
        // (évite un warning de bevy)
        //remove enemy
        let despawn_plus_enemy: HashSet<Entity> = if !despawned_entities.contains(&enemy_entity) {
            commands.entity(enemy_entity).despawn();
            enemy_count.0 -= 1;
            despawned_entities
                .union(&HashSet::from_iter(vec![enemy_entity]))
                .map(|a| a.clone())
                .collect()
        } else {
            despawned_entities
        };
        
        // remove laser
        if !despawn_plus_enemy.contains(&laser_entity) {
            commands.entity(laser_entity).despawn();
            
            despawn_plus_enemy
                .union(&HashSet::from_iter(vec![laser_entity]))
                .map(|a| a.clone())
                .collect::<HashSet<Entity>>()

        } else {
            despawn_plus_enemy
        }        
    } else {
        despawned_entities
    }
}
