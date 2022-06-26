use bevy::prelude::*;

use crate::{AppState, POLICE};
use crate::component::{
    state::InEndgameComponent
};
use crate::resources::{Scoring};

pub struct EndgamePlugin;

impl Plugin for EndgamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::EndGame)
                    .with_system(setup_endgame_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::EndGame)
                    .with_system(from_endgame_to_menu_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::EndGame)
                    .with_system(on_exit_endgame_sytem)
                    .with_system(restart_scoring_system)
            );
    }
}

fn from_endgame_to_menu_system(
    mut app_state: ResMut<State<AppState>>,
    kb: Res<Input<KeyCode>>
) {
    if kb.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn restart_scoring_system(
    mut scoring: ResMut<Scoring>
) {
    scoring.bug_fix = 0;
}

fn on_exit_endgame_sytem(
    mut commands: Commands,
    query: Query<Entity, With<InEndgameComponent>>
) {
    query.iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn setup_endgame_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    scoring: Res<Scoring>
) {

    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Score : {}", scoring.bug_fix),
                    style: TextStyle {
                        color: Color::GREEN,
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
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        })
        .insert(InEndgameComponent);
}
