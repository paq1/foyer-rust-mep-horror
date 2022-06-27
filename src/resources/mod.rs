use bevy::prelude::{Handle, Image};

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

pub struct GameTextures {
    pub computer: Handle<Image>,
    pub file_laser: Handle<Image>,
    pub fixme_file: Handle<Image>,
    pub push_file: Handle<Image>,
    pub bg: Handle<Image>
}

pub struct Scoring {
    pub bug_fix: u32
}


impl Default for Scoring {
    fn default() -> Self {
        Scoring {bug_fix: 0}
    }
}

pub struct Timer(pub f32);
