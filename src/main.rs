mod player;
mod components;

use bevy::prelude::*;
use crate::player::PlayerPlugin;

const PLAYER_SPRITE: &str = "Player.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
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
        player: asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures)
}


