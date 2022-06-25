use bevy::prelude::*;
use rand::prelude::*;
use crate::{GameTextures, WinSize, EnemyCount, COMPUTER_SIZE, ENEMY_MAX};
use crate::component::{
    enemy::Enemy,
    sprite_size::SpriteSize,
};
use crate::AppState;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(enemy_spawn_system)
            );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    if enemy_count.0 < ENEMY_MAX {
        spawn_enemy(&mut commands,&mut enemy_count, &game_textures, &win_size);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    enemy_count: &mut ResMut<EnemyCount>,
    game_textures: &Res<GameTextures>,
    win_size: &Res<WinSize>
) {
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = win_size.h / 2. - 64.;// rng.gen_range(-h_span..h_span);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                ..Default::default()
            },
            texture: game_textures.fixme_file.clone(),
            ..Default::default()
        })
        .insert(SpriteSize::from(COMPUTER_SIZE))
        .insert(Enemy);

    enemy_count.0 += 1;
}