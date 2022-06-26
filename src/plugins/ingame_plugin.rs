use bevy::prelude::*;

use crate::{AppState, TIME, TIME_STEP, BASE_SPEED};
use crate::component::{
    hud::TextTimer,
    state::InGameComponent,
    velocity::Velocity,
    movable::Movable,
    player::ScoreBugFix
};
use crate::resources::{WinSize, Timer as MonTimer};

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(write_scoring_system)
                    .with_system(write_timer_system)
                    .with_system(restart_timer_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                    .with_system(on_exit_ingame_sytem)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(from_game_to_endgame_system)
                    .with_system(movable_system)
                    .with_system(update_timer_system)
                    .with_system(update_timer_text_system)
            );
    }
}

fn write_scoring_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands, 
    win_size: Res<WinSize>,
) {
    let pos_score = (-win_size.w / 2., win_size.h / 2.);

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
        .insert(ScoreBugFix)
        .insert(InGameComponent);
}

fn write_timer_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>
) {
    let x = 0.;
    let y = win_size.h / 2.; 

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
        .insert(TextTimer)
        .insert(InGameComponent);
}

fn restart_timer_system(mut timer: ResMut<MonTimer>) {
    timer.0 = TIME;
}

fn on_exit_ingame_sytem(
    mut commands: Commands,
    query: Query<Entity, With<InGameComponent>>
) {
    query.iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn from_game_to_endgame_system(
    mut app_state: ResMut<State<AppState>>,
    timer: Res<MonTimer>
) {
    if timer.0 < 0. {
        app_state.set(AppState::EndGame).unwrap();
    }
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
