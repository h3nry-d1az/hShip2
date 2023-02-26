use crate::{
    resources::{
        GameTextures,
    },
    constants::{
        BACKGROUND_BASE_SPEED,
        BACKGROUND_SPRITE_SIZE,
        BACKGROUND_SPRITE_SCALE
    },
    components::Background
};
use bevy::prelude::*;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                background_spawn_system
            )
            .add_system(background_movement_system);
    }
}

fn background_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let mut spawn_background = |offset: f32| {
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.background.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        0. + offset*BACKGROUND_SPRITE_SIZE.1*BACKGROUND_SPRITE_SCALE,
                        0.
                    ),
                    scale: Vec3::new(
                        BACKGROUND_SPRITE_SCALE,
                        BACKGROUND_SPRITE_SCALE,
                        1.
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Background);
    };

    spawn_background(-1.);
    spawn_background(0.);
    spawn_background(1.);
}

fn background_movement_system(
    mut query: Query<&mut Transform, With<Background>>
) {
    for mut transf in query.iter_mut() {
        let transl = &mut transf.translation;
        transl.y -= BACKGROUND_BASE_SPEED;
        if transl.y <= - (BACKGROUND_SPRITE_SCALE*BACKGROUND_SPRITE_SIZE.1) {
            transl.y = BACKGROUND_SPRITE_SIZE.1*BACKGROUND_SPRITE_SCALE;
        }
    }
}