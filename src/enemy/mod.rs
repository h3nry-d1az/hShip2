use std::f32::consts::PI;

use bevy::{prelude::*, core::FixedTimestep, ecs::schedule::ShouldRun};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::{
    GameTextures,
    constants::{
        ENEMY_SPRITE_SCALE,
        ENEMY_SPRITE_SIZE,
        MAX_ENEMIES_ON_STAGE,
        LASER_SPRITE_SCALE,
        LASER_SPRITE_SIZE,
        FPS_TARGET
    },
    resources::{WinSize, EnemyCount, GameReady},
    components::{Enemy, SpriteSize, Velocity, Bullet, FromEnemy}
};

use self::formation::{FormationMaker, Formation};

mod formation;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FormationMaker::default())
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
    game_ready: Res<GameReady>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    wsize: Res<WinSize>
) {
    if enemy_count.0 < MAX_ENEMIES_ON_STAGE
    && game_ready.0 {
        let mut rng = rand::thread_rng();
        let enemy_type = rng.gen_bool(0.5);

        let formation = formation_maker.make(&wsize);
        let (x, y) = formation.start;

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
                .insert(formation)
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
                .insert(formation)
                .insert(SpriteSize::from(ENEMY_SPRITE_SIZE));
        }

        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if rand::thread_rng().gen_bool((FPS_TARGET * 2.) as f64) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    audio: Res<Audio>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Enemy>>
) {
    let transform_vec = query
        .iter()
        .collect::<Vec<&Transform>>();
    let transform = transform_vec.choose(&mut rand::thread_rng());

    match transform {
        Some(trans) => {
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
            audio.play(game_textures.sfx_shoot.clone());
        },
        None => {}
    }

}

fn enemy_movement_system(
    mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>
) {
    for (mut transf, mut formation) in query.iter_mut() {
        let (x, y) = (transf.translation.x, transf.translation.y);
        let max_distance = FPS_TARGET * formation.speed;

        let dir = if formation.start.0 < 0. { 1. } else { -1. };
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        let angle = formation.angle + dir * formation.speed * FPS_TARGET / (x_radius.min(y_radius) * PI / 2.);

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

        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let transl = &mut transf.translation;
        (transl.x, transl.y) = (x_fin, y_fin);
    }
}