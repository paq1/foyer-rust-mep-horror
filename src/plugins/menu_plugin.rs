use bevy::prelude::*;

use crate::{AppState, POLICE};
use crate::component::{
    state::InMenuComponent
};
use crate::resources::{WinSize};

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
    mut commands: Commands,
    win_size: Res<WinSize>
) {
    // on ajoute le texte du score


    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Foyer MEP horror".to_string(),
                    style: TextStyle {
                        color: Color::RED,
                        font: asset_server.load(POLICE),
                        font_size: 36.
                    }
                }],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0., win_size.h / 2., 1.),
            ..Default::default()
        })
        .insert(InMenuComponent);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Press [X] pour fix un bug lorsqu'un\nfichier \"fixme\" apparait".to_string(),
                    style: TextStyle {
                        color: Color::ORANGE,
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
            // transform: Transform::from_xyz(pos_score.0, pos_score.1, 1.),
            ..Default::default()
        })
        .insert(InMenuComponent);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Press [Espace] pour jouer".to_string(),
                    style: TextStyle {
                        color: Color::YELLOW,
                        font: asset_server.load(POLICE),
                        font_size: 36.
                    }
                }],
                
                alignment: TextAlignment {
                    //vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
                
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -win_size.h / 2. + 36., 1.),
            ..Default::default()
        })
        .insert(InMenuComponent);
}


fn on_exit_menu_sytem(
    mut commands: Commands,
    query: Query<Entity, With<InMenuComponent>>
) {
    query.iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn from_menu_to_game_system(
    mut app_state: ResMut<State<AppState>>,
    kb: Res<Input<KeyCode>>
) {
    if kb.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn text_menu() -> String {
    vec![
        "Foyer MEP horror",
        "fixer le plus de bugs avant la MEP",
        "appuyer [espace] pour jouer"
    ].join("\n\n\n")
}
