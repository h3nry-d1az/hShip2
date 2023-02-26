use bevy::prelude::*;
use rand::Rng;
use crate::{
    GameTextures,
    constants::{ENEMY_SPRITE_SCALE, ENEMY_SPRITE_SIZE},
    resources::WinSize,
    components::{Enemy, SpriteSize}
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    wsize: Res<WinSize>
) {
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
                    translation: Vec3::new(0., 0., 10.),
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
}