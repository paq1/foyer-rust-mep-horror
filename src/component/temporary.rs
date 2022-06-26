use bevy::prelude::Component;

#[derive(Component)]
pub struct Temporary {
    pub duration: f32,
    pub current_time: f32,
}