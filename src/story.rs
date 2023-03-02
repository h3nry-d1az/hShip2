use crate::{
    components::Story,
    constants::{STORY_BASE_SPEED, STORY_SPRITE_SCALE, STORY_SPRITE_SIZE},
    resources::{WinSize, GameReady}
};
use bevy::prelude::*;

pub struct StoryPlugin;
impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(story_movement_system);
    }
}

fn story_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Story>>,
    mut game_ready: ResMut<GameReady>,
    wsize: Res<WinSize>,
    kb: Res<Input<KeyCode>>
) {
    for (e, mut transf) in query.iter_mut() {
        let transl = &mut transf.translation;
        if kb.pressed(KeyCode::Space) {
            transl.y += 5.*STORY_BASE_SPEED;
        } else {
            transl.y += STORY_BASE_SPEED;
        }
        if transl.y >= ((wsize.h)/2. + (STORY_SPRITE_SIZE.1 * STORY_SPRITE_SCALE)/2.) {
            commands.entity(e).despawn();
            game_ready.0 = true;
        }
    }
}