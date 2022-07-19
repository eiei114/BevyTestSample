mod player;
mod components;
mod enemy;

use bevy::prelude::*;
use crate::components::{Movable, Velocity};
use crate::player::PlayerPlugin;

const PLAYER_SPRITE: &str = "Player.png";
const PLAYER_SIZE: (f32, f32) = (1.5, 0.78125);
const PLAYER_LASER_SPRITE: &str = "laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "Enemy.png";
const ENEMY_SIZE: (f32, f32) = (1.5, 0.78125);
const ENEMY_LASER_SPRITE: &str = "Flame_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .run();
}

fn setup_system(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut windows: ResMut<Windows>,
) {
    //カメラ
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //ウィンドウサイズ
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    //ウィンドウの位置
    window.set_position(IVec2::new(1900, 0));

    //Windowsサイズのリソースを追加
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    //ゲームテクスチャのリソースを追加
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),

    };
    commands.insert_resource(game_textures)
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            //画面外にlaserが出た時にそれを削除する
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}


