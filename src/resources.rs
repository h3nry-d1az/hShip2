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
    pub story: Handle<Image>,
    pub game_over: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
    pub cascadia_code: Handle<Font>
}

pub struct EnemyCount(pub u32);
pub struct GameReady(pub bool);
pub struct SpawnedText(pub bool);

pub struct PlayerState {
    pub alive: bool,
    pub lives: u32,
    pub score: u32,
    pub last_shot: f64,
    pub last_shoot: f64
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: false,
            lives: 4,
            score: 0,
            last_shot: -1.,
            last_shoot: -1.
        }
    }
}
impl PlayerState {
    pub fn shot(
        &mut self,
        time: f64
    ) {
        self.alive = false;
        self.lives -= 1;
        self.last_shot = time;
    }

    pub fn shoot(
        &mut self,
        time: f64
    ) {
        self.last_shoot = time;
    }

    pub fn spawned(&mut self) {
        self.alive = true;
        self.last_shot = -1.;
        self.last_shoot = -1.;
    }
}