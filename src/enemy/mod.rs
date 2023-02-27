use std::f32::consts::PI;

use bevy::{prelude::*, core::FixedTimestep, ecs::schedule::ShouldRun};
use rand::Rng;
use crate::{
    GameTextures,
    constants::{
        ENEMY_SPRITE_SCALE,
        ENEMY_SPRITE_SIZE,
        MAX_ENEMIES_ON_STAGE,
        LASER_SPRITE_SCALE, LASER_SPRITE_SIZE, FPS_TARGET, ENEMY_BASE_SPEED
    },
    resources::{WinSize, EnemyCount},
    components::{Enemy, SpriteSize, Velocity, Bullet, FromEnemy}
};

mod formation;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(enemy_spawn_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system)
            )
            .add_system(enemy_movement_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    wsize: Res<WinSize>
) {
    if enemy_count.0 < MAX_ENEMIES_ON_STAGE {
        let mut rng = rand::thread_rng();
        let w_span = wsize.w / 2. - 100.;
        let h_span = wsize.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range((-h_span+200.)..h_span);
        let enemy_type = rng.gen_bool(0.5);

        if enemy_type {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.enemy1.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.),
                        scale: Vec3::new(
                            ENEMY_SPRITE_SCALE,
                            ENEMY_SPRITE_SCALE,
                            1.
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Enemy)
                .insert(SpriteSize::from(ENEMY_SPRITE_SIZE));
        } else {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.enemy2.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.),
                        scale: Vec3::new(
                            ENEMY_SPRITE_SCALE,
                            ENEMY_SPRITE_SCALE,
                            1.
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Enemy)
                .insert(SpriteSize::from(ENEMY_SPRITE_SIZE));
        }

        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if rand::thread_rng().gen_bool(FPS_TARGET as f64) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Enemy>>
) {
    for &trans in query.iter() {
        let (x, y) = (trans.translation.x, trans.translation.y);
        commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.enemy_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y - 25., 1.),
                        rotation: Quat::from_rotation_x(PI),
                        scale: Vec3::new(
                            LASER_SPRITE_SCALE,
                            LASER_SPRITE_SCALE + 0.1,
                            1.
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Velocity {x: 0., y: -1.})
                .insert(Bullet)
                .insert(FromEnemy)
                .insert(SpriteSize::from(LASER_SPRITE_SIZE));
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>
) {
    let now = time.seconds_since_startup() as f32;

    for mut transf in query.iter_mut() {
        let (x, y) = (transf.translation.x, transf.translation.y);
        let max_distance = FPS_TARGET * ENEMY_BASE_SPEED;

        let dir = -1.;
        let (x_pivot, y_pivot) = (0., 0.);
        let (x_radius, y_radius) = (200., 130.);

        let angle = dir * ENEMY_BASE_SPEED * FPS_TARGET * now % 360. / PI;

        let x_dest = x_radius * angle.cos() + x_pivot;
        let y_dest = y_radius * angle.sin() + y_pivot;

        let dx = x - x_dest;
        let dy = y - y_dest;

        let distance = (dx*dx + dy*dy).sqrt();
        let distance_ratio = if distance != 0. { max_distance / distance } else { 0. };

        let x_fin = x - dx * distance_ratio;
        let x_fin = if dx > 0. { x_fin.max(x_dest) } else { x_fin.min(x_dest) };
        let y_fin = y - dy * distance_ratio;
        let y_fin = if dy > 0. { y_fin.max(y_dest) } else { y_fin.min(y_dest) };

        let transl = &mut transf.translation;
        (transl.x, transl.y) = (x_fin, y_fin);
    }
}