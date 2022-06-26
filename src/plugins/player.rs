use bevy::prelude::*;

use crate::{SPRITE_SIZE, SPRITE_SCALE};
use crate::resources::{GameTextures, WinSize};
use crate::AppState;
use crate::component::{
    player::Player, 
    velocity::Velocity,
    movable::Movable,
    laser::Laser,
    sprite_size::SpriteSize,
    state::InGameComponent
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // todo : avoir le meme comportement dans SystemSet::on_enter(AppState::InGame)
            // .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(player_spawn_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_keyboard_event_system)
                    .with_system(player_fire_system)
            );
    }
}

fn player_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    // add computer sprite
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.computer.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + SPRITE_SIZE.1 / 2. + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(SpriteSize::from(SPRITE_SIZE))
        .insert(Movable {auto_despawn: false})
        .insert(Velocity {x: 0., y: 0.})
        .insert(InGameComponent);
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::X) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.file_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SpriteSize::from(SPRITE_SIZE))
                .insert(Movable {auto_despawn: true})
                .insert(Laser)
                .insert(Velocity {x: 0., y: 1.})
                .insert(InGameComponent);
        }
    }
}
