pub const PLAYER_SPRITE_PATH: &str = "images/img_player.png";
pub const PLAYER_SPRITE_SIZE: (f32, f32) = (200., 240.);
pub const PLAYER_SPRITE_SCALE: f32 = 0.4;
pub const PLAYER_RESPAWN_DELAY: f64 = 2.;
pub const PLAYER_SHOOT_COOLDOWN: f64 = 0.5;

pub const PLAYER_LASER_SPRITE_PATH: &str = "images/img_player_laser.png";
pub const ENEMY_LASER_SPRITE_PATH: &str = "images/img_enemy_laser.png";
pub const LASER_SPRITE_SIZE: (f32, f32) = (78., 214.);
pub const LASER_SPRITE_SCALE: f32 = 0.2;

pub const ENEMY1_SPRITE_PATH: &str = "images/img_enemy_1.png";
pub const ENEMY2_SPRITE_PATH: &str = "images/img_enemy_2.png";
pub const ENEMY_SPRITE_SIZE: (f32, f32) = (168., 168.);
pub const ENEMY_SPRITE_SCALE: f32 = 0.4;
pub const MAX_ENEMIES_ON_STAGE: u32 = 8;
pub const MAX_ENEMIES_IN_FORMATION: u32 = 4;
pub const SCORE_PER_ENEMY: u32 = 100;

pub const EXPLOSION_SPRITESHEET_PATH: &str = "images/img_explosion.png";
pub const EXPLOSION_SPRITESHEET_SCALE: f32 = 0.4;
pub const EXPLOSION_LENGTH: usize = 3;

pub const BACKGROUND_SPRITE_PATH: &str = "images/img_background.png";
pub const BACKGROUND_SPRITE_SIZE: (f32, f32) = (862., 352.);
pub const BACKGROUND_SPRITE_SCALE: f32 = 2.;

pub const STORY_SPRITE_PATH: &str = "images/img_story.png";
pub const STORY_SPRITE_SIZE: (f32, f32) = (433., 576.);
pub const STORY_SPRITE_SCALE: f32 = 2.;

pub const GAME_OVER_SPRITE_PATH: &str = "images/img_game_over.png";
pub const GAME_OVER_SPRITE_SCALE: f32 = 1.;

pub const FPS_TARGET: f32 = 1./60.;
pub const PLAYER_BASE_SPEED: f32 = 500.;
pub const ENEMY_BASE_SPEED: f32 = 350.;
pub const BULLET_BASE_SPEED: f32 = 500.;
pub const BACKGROUND_BASE_SPEED: f32 = 2.;
pub const STORY_BASE_SPEED: f32 = 0.85;
