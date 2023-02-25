use bevy::prelude::*;

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>
}