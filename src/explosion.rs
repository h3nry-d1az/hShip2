use bevy::prelude::*;
use crate::{
    resources::GameTextures,
    components::{
        ExplosionToSpawn,
        Explosion,
        ExplosionTimer
    },
    constants::{
        EXPLOSION_SPRITESHEET_SCALE,
        EXPLOSION_LENGTH
    }
};

pub struct ExplosionPlugin;
impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(explosion_to_spawn_system)
           .add_system(explosion_animation_system);
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    scale: Vec3::new(
                        EXPLOSION_SPRITESHEET_SCALE,
                        EXPLOSION_SPRITESHEET_SCALE,
                        1.
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(explosion).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LENGTH {
                commands.entity(entity).despawn();
            }
        }
    }
}