use bevy::prelude::Component;

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}