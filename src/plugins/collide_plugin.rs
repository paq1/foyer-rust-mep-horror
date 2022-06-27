use std::collections::HashSet;
use std::iter::FromIterator;

use rand::prelude::*;

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;

use crate::{Scoring, SPRITE_SIZE, POLICE};
use crate::component::{
    enemy::Enemy,
    sprite_size::SpriteSize,
    laser::Laser,
    player::ScoreBugFix,
    velocity::Velocity,
    movable::Movable,
    push_file::PushFile,
    state::InGameComponent,
    temporary::Temporary
};
use crate::resources::{
    GameTextures,
    WinSize
};

pub struct CollideFireEnemiesPlugin;

impl Plugin for CollideFireEnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_file_hit_enemy_system);
    }
}

fn player_file_hit_enemy_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut scoring: ResMut<Scoring>,
    asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), With<Laser>>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut score_bug_fix_query: Query<(Entity, &mut Text), With<ScoreBugFix>>
) {
    laser_query
        .iter()
        .fold(
            HashSet::new(), 
            |acc, current| 
            handle_collide_laser_enemies(&mut commands, &win_size, &mut scoring, &asset_server, &acc, current, &game_textures, &enemy_query, &mut score_bug_fix_query)
        );
}

fn handle_collide_laser_enemies(
    commands: &mut Commands,
    win_size: &Res<WinSize>,
    mut scoring: &mut ResMut<Scoring>,
    asset_server: &Res<AssetServer>,
    despawned_entities: &HashSet<Entity>, 
    laser_info: (Entity, &Transform, &SpriteSize),
    game_textures: &Res<GameTextures>,
    enemy_query: &Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    score_bug_fix_query: &mut Query<(Entity, &mut Text), With<ScoreBugFix>>
) -> HashSet<Entity> {
    enemy_query
        .iter()
        .fold(
            despawned_entities.clone(), 
            |acc: HashSet<Entity>, current| 
            handle_collide_laser_enemy(commands, win_size, &mut scoring, asset_server, acc, laser_info, current, game_textures, score_bug_fix_query)
        )
}

fn handle_collide_laser_enemy(
    commands: &mut Commands,
    win_size: &Res<WinSize>,
    scoring: &mut ResMut<Scoring>,
    asset_server: &Res<AssetServer>,
    despawned_entities: HashSet<Entity>, 
    laser_info: (Entity, &Transform, &SpriteSize),
    enemy_info: (Entity, &Transform, &SpriteSize),
    game_textures: &Res<GameTextures>,
    score_bug_fix_query: &mut Query<(Entity, &mut Text), With<ScoreBugFix>>
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
            scoring.bug_fix += 1;
            spawn_push_file(commands, enemy_tf, game_textures);
            spawn_push_text(commands, asset_server, win_size);
            
            for (_, mut text) in score_bug_fix_query.iter_mut() {
                text.sections[0].value = format!("bug fix : {}", scoring.bug_fix).to_string();
            }
            
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

fn spawn_push_file(
    commands: &mut Commands,
    enemy_position: &Transform,
    game_textures: &Res<GameTextures>
) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(enemy_position.translation.x, enemy_position.translation.y, 10.),
                ..Default::default()
            },
            texture: game_textures.push_file.clone(),
            ..Default::default()
        })
        .insert(SpriteSize::from(SPRITE_SIZE))
        .insert(Velocity {x: 0., y: 0.8})
        .insert(Movable { auto_despawn: true })
        .insert(PushFile)
        .insert(InGameComponent);
}

fn spawn_push_text(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    win_size: &Res<WinSize>
) {
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 64.;
    let h_span = win_size.h / 2. - 64.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);// rng.gen_range(-h_span..h_span);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "PUSH !!".to_string(),
                    style: TextStyle {
                        color: Color::rgb(0., 0.3, 0.),
                        font: asset_server.load(POLICE),
                        font_size: 36.
                    }
                }],
                
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 2.),
            ..Default::default()
        })
        .insert(Temporary { duration: 1., current_time: 0.})
        .insert(InGameComponent);
}
