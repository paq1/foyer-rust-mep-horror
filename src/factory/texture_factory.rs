use bevy::prelude::*;
use crate::GameTextures;

const COMPUTER_SPRITE: &str = "pc-codeur.png";
const ENEMY_SPRITE: &str = "fixme-file.png";
const FILE_LASER_SPRITE: &str = "scala-file.png";
const PUSH_FILE_SPRITE: &str = "push-file.png";

pub fn create_game_textures(asset_server: &AssetServer) -> GameTextures {
    GameTextures {
        computer: asset_server.load(COMPUTER_SPRITE),
        file_laser: asset_server.load(FILE_LASER_SPRITE),
        fixme_file: asset_server.load(ENEMY_SPRITE),
        push_file: asset_server.load(PUSH_FILE_SPRITE)
    }
}
