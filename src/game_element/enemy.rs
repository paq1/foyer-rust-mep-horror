use bevy::prelude::*;
use rand::prelude::*;
use crate::{GameTextures, WinSize, COMPUTER_SIZE, SPRITE_SCALE, TIME_STEP, BASE_SPEED};
use crate::component::{
    enemy::Enemy,
    sprite_size::SpriteSize,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
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
}