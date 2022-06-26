use bevy::prelude::*;
use rand::prelude::*;
use crate::{GameTextures, WinSize, SPRITE_SIZE, ENEMY_MAX};
use crate::component::{
    enemy::Enemy,
    sprite_size::SpriteSize,
    velocity::Velocity,
    movable::Movable,
    state::InGameComponent
};
use crate::AppState;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(enemy_spawn_system_v2)
            );
    }
}

fn enemy_spawn_system_v2(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    query: Query<Entity, With<Enemy>>
) {
    let enemies = query.iter().collect::<Vec<Entity>>();
    if (enemies.len() as u32) < ENEMY_MAX {
        spawn_enemy(&mut commands, &game_textures, &win_size);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    win_size: &Res<WinSize>
) {
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = win_size.h / 2. + 64.;// rng.gen_range(-h_span..h_span);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                ..Default::default()
            },
            texture: game_textures.fixme_file.clone(),
            ..Default::default()
        })
        .insert(SpriteSize::from(SPRITE_SIZE))
        .insert(Velocity {x: 0., y: -0.5})
        .insert(Movable { auto_despawn: true })
        .insert(Enemy)
        .insert(InGameComponent);
}