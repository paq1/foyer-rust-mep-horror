use bevy::prelude::*;

use crate::AppState;
use crate::component::{
    menu::TextMenu
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(write_menu_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(on_exit_menu_sytem)
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(from_menu_to_game_system)
            );
    }
}

fn write_menu_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    // on ajoute le texte du score
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "press [space] to start".to_string(),
                    style: TextStyle {
                        color: Color::RED,
                        font: asset_server.load("COMICATE.TTF"),
                        font_size: 36.
                    }
                }],
                
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                
                ..Default::default()
            },
            // transform: Transform::from_xyz(pos_score.0, pos_score.1, 1.),
            ..Default::default()
        })
        .insert(TextMenu);
}


fn on_exit_menu_sytem(
    mut commands: Commands,
    query: Query<Entity, With<TextMenu>>
) {
    let entity = query.get_single().unwrap();
    commands.entity(entity).despawn();
}

fn from_menu_to_game_system(
    mut app_state: ResMut<State<AppState>>,
    kb: Res<Input<KeyCode>>
) {
    if kb.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }
}
