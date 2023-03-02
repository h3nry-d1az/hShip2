use bevy::{prelude::{Component, Vec2, Vec3, Timer}};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(v: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(v.0, v.1))
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct Background;
#[derive(Component)]
pub struct Story;

#[derive(Component)]
pub struct LivesText;
#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Explosion;
#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);
impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1./3., true))
    }
}