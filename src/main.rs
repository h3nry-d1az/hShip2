use std::collections::HashSet;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::{
    WindowMode,
    WindowId
};
use bevy::winit::WinitWindows;
use winit::window::Icon;

mod player;
mod enemy;
mod constants;
mod components;
mod resources;
mod background;
mod explosion;

use constants::*;
use resources::*;
use components::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.11)))
        .insert_resource(WindowDescriptor {
            title: "hShip ][: the last voyage".to_string(),
            width: 1366.,
            height: 768.,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(explosion::ExplosionPlugin)
        .add_startup_system(setup_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(enemy_laser_hit_player_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
    winit_windows: NonSend<WinitWindows>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    commands.insert_resource(WinSize { w: window.width(), h: window.height() });

    let window = winit_windows.get_window(WindowId::primary()).unwrap();
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    window.set_window_icon(Some(icon));

    let texture_handle = asset_server.load(EXPLOSION_SPRITESHEET_PATH);
    let texture_atlass = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(288., 240.),
        3,
        1
    );
    let explosion = texture_atlasses.add(texture_atlass);

    commands.insert_resource(GameTextures {
        player: asset_server.load(PLAYER_SPRITE_PATH),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE_PATH),
        enemy1: asset_server.load(ENEMY1_SPRITE_PATH),
        enemy2: asset_server.load(ENEMY2_SPRITE_PATH),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE_PATH),
        background: asset_server.load(BACKGROUND_SPRITE_PATH),
        explosion
    });
    commands.insert_resource(EnemyCount(0));
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    for (bl_entity, bl_trans, bl_size) in laser_query.iter() {
        if despawned_entities.contains(&bl_entity) { continue; }
        let bl_scale = Vec2::from(bl_trans.scale.xy());
        for (en_entity, en_trans, en_size) in enemy_query.iter() {
            if despawned_entities.contains(&en_entity) 
            || despawned_entities.contains(&bl_entity) { continue; }
            let en_scale = Vec2::from(en_trans.scale.xy());

            let collision = collide(
                bl_trans.translation,
                bl_size.0 * bl_scale,
                en_trans.translation,
                en_size.0 * en_scale,
            );

            if let Some(_) = collision {
                commands.entity(en_entity).despawn();
                despawned_entities.insert(en_entity);
                enemy_count.0 -= 1;

                commands.entity(bl_entity).despawn();
                despawned_entities.insert(bl_entity);

                commands.spawn().insert(ExplosionToSpawn(en_trans.translation.clone()));
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>
) {
    if let Ok((pl_entity, pl_trans, pl_size)) = player_query.get_single() {
        let pl_scale = Vec2::from(pl_trans.scale.xy());
        for (bl_entity, bl_trans, bl_size) in laser_query.iter() {
            let bl_scale = Vec2::from(bl_trans.scale.xy());

            let collision = collide(
                bl_trans.translation,
                bl_size.0 * bl_scale,
                pl_trans.translation,
                pl_size.0 * pl_scale,
            );

            if let Some(_) = collision {
                commands.entity(pl_entity).despawn();
                player_state.shot(time.seconds_since_startup());

                commands.entity(bl_entity).despawn();
                commands.spawn().insert(ExplosionToSpawn(pl_trans.translation.clone()));
            }
        }
    }
}