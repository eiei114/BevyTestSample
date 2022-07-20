mod player;
mod components;
mod enemy;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::components::{Enemy, ExplosionToSpawn, FromPlayer, Laser, Movable, SpriteSize, Velocity};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

const PLAYER_SPRITE: &str = "Player.png";
const PLAYER_SIZE: (f32, f32) = (1.5, 0.78125);
const PLAYER_LASER_SPRITE: &str = "laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "Enemy.png";
const ENEMY_SIZE: (f32, f32) = (1.5, 0.78125);
const ENEMY_LASER_SPRITE: &str = "Flame_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);

const EXPLOSION_SHEET: &str = "Explosion.png";

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
    explosion: Handle<TextureAtlas>,
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
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .run();
}

fn setup_system(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut texture_atlases:ResMut<Assets<TextureAtlas>>,
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

    //爆発テクスチャ生成
    let texture_handle=asset_server.load(EXPLOSION_SHEET);
    let texture_atlas=TextureAtlas::from_grid(texture_handle,Vec2::new(64.,64.),4,4);
    let explosion=texture_atlases.add(texture_atlas);

    //ゲームテクスチャのリソースを追加
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE), enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
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


fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    //レーザーを繰り返して出す
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        //繰り返し敵を出す
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            //当たり判定の設定
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            //当たり判定による行動
            if let Some(_) = collision {
                //敵を削除する
                commands.entity(enemy_entity).despawn();

                //レーザーの削除
                commands.entity(laser_entity).despawn();

                //爆発エフェクトをスポーンさせる
                commands.spawn().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}



