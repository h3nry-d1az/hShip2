use crate::{
    resources::{
        GameTextures,
        WinSize,
    },
    constants::{
        PLAYER_SPRITE_SIZE,
        PLAYER_SPRITE_SCALE,
        FPS_TARGET,
        PLAYER_BASE_SPEED,
        BULLET_BASE_SPEED,
        LASER_SPRITE_SCALE,
    },
    components::{
        Player,
        Bullet,
        Velocity,
    }
};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                player_spawn_system
            )
            .add_system(player_movement_system)
            .add_system(bullet_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    wsize: Res<WinSize>
) {
    let bottom = -wsize.h / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(
                    0.,
                    bottom + PLAYER_SPRITE_SIZE.1 / 2. * PLAYER_SPRITE_SCALE + 5.,
                    10.
                ),
                scale: Vec3::new(
                    PLAYER_SPRITE_SCALE,
                    PLAYER_SPRITE_SCALE,
                    1.
                ),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity {x: 0., y: 0.});
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut vel) = query.get_single_mut() {
        vel.x = if kb.pressed(KeyCode::Left)
                || kb.pressed(KeyCode::A) {
            -1.
        } else if kb.pressed(KeyCode::Right)
               || kb.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        vel.y = if kb.pressed(KeyCode::Down)
                || kb.pressed(KeyCode::S) {
            -1.
        } else if kb.pressed(KeyCode::Up)
               || kb.pressed(KeyCode::W) {
            1.
        } else {
            0.
        };
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(trans) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (trans.translation.x, trans.translation.y);
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y + 20., 1.),
                        scale: Vec3::new(
                            LASER_SPRITE_SCALE,
                            LASER_SPRITE_SCALE + 0.1,
                            1.
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Velocity {x: 0., y: 1.})
                .insert(Bullet);
        }
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
    wsize: Res<WinSize>
) {
    for (vel, mut transf) in query.iter_mut() {
        let transl = &mut transf.translation;
        transl.x = (transl.x + (vel.x * FPS_TARGET * PLAYER_BASE_SPEED)).min(640.).max(-640.);
        transl.y = (transl.y + (vel.y * FPS_TARGET * PLAYER_BASE_SPEED)).min(-175.).max(-wsize.h/2. + PLAYER_SPRITE_SIZE.1 / 2. * PLAYER_SPRITE_SCALE + 5.);
    }
}

fn bullet_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Bullet>>,
    wsize: Res<WinSize>
) {
    for (entity, vel, mut transf) in query.iter_mut() {
        let transl = &mut transf.translation;
        transl.x += vel.x * FPS_TARGET * BULLET_BASE_SPEED;
        transl.y += vel.y * FPS_TARGET * BULLET_BASE_SPEED;

        const M: f32 = 10.;
        if transl.y > wsize.h / 2. + M
        || transl.y < -wsize.h / 2. - M
        || transl.x > wsize.w / 2. + M
        || transl.x < -wsize.w / 2. - M
        {
            commands.entity(entity).despawn();
        }
    }
}