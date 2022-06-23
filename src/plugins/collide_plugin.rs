use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;

use std::collections::HashSet;
use std::iter::FromIterator;

use crate::EnemyCount;
use crate::component::{
    enemy::Enemy,
    sprite_size::SpriteSize,
};

use crate::component::{
    laser::Laser
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
