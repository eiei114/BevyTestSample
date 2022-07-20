use crate::{ENEMY_SIZE, GameTextures, SPRITE_SCALE, WinSize};
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::components::{Enemy, SpriteSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>,
                      win_size: Res<WinSize>, ) {
    //ランダムに生成
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(SPRITE_SCALE * 1. / 2., SPRITE_SCALE * 1. / 2., 1.),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));
}