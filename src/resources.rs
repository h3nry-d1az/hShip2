use bevy::prelude::*;

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy1: Handle<Image>,
    pub enemy2: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub background: Handle<Image>,
    pub explosion: Handle<TextureAtlas>
}

pub struct EnemyCount(pub u32);

pub struct PlayerState {
    pub alive: bool,
    pub last_shot: f64
}
impl Default for PlayerState {
    fn default() -> Self {
        Self { alive: false, last_shot: -1. }
    }
}
impl PlayerState {
    pub fn shot(
        &mut self,
        time: f64
    ) {
        self.alive = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self) {
        self.alive = true;
        self.last_shot = -1.;
    }
}