use bevy::prelude::*;

use crate::{
    resources::{
        SpawnedText,
        GameReady,
        GameTextures,
        PlayerState
    },
    constants::GAME_FONT_SIZE,
    components::{LivesText, ScoreText}
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_text_system)
            .add_system(update_lives_text_system)
            .add_system(update_score_text_system);
    }
}

fn spawn_text_system(
    mut commands: Commands,
    mut text_spawned: ResMut<SpawnedText>,
    game_ready: Res<GameReady>,
    game_textures: Res<GameTextures>,
) {
    if game_ready.0
    && !text_spawned.0 {
        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "lives: ".to_string(),
                    TextStyle {
                        font: game_textures.cascadia_code.clone(),
                        font_size: GAME_FONT_SIZE,
                        color: Color::RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(10.),
                        left: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(LivesText);

        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "score: ".to_string(),
                    TextStyle {
                        font: game_textures.cascadia_code.clone(),
                        font_size: GAME_FONT_SIZE,
                        color: Color::YELLOW_GREEN,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(70.),
                        left: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(ScoreText);

            text_spawned.0 = true;
    }
}

fn update_lives_text_system(
    mut query: Query<&mut Text, With<LivesText>>,
    player_state: Res<PlayerState>,
    text_spawned: Res<SpawnedText>
) {
    if text_spawned.0 {
        let mut lives_text = "lives: ".to_string();
        for _ in 0..player_state.lives {
            lives_text += "♥ ";
        }

        if let Ok(mut lt) = query.get_single_mut() {
            lt.sections[0].value = lives_text;
        }
    }
}

fn update_score_text_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    player_state: Res<PlayerState>,
    text_spawned: Res<SpawnedText>
) {
    if text_spawned.0 {
        if let Ok(mut st) = query.get_single_mut() {
            st.sections[0].value = format!("score: {}", player_state.score);
        }
    }
}