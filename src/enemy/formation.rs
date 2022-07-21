use bevy::prelude::Component;
use rand::{Rng, thread_rng};
use crate::{BASE_SPEED, FORMATION_MEMBERS_MAX, WinSize};

//敵のポジション
#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32,
}

#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

//フォーメーション作成
impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (&self.current_template, self.current_members >= FORMATION_MEMBERS_MAX) {
            //敵の数が最大である場合
            (Some(tmpl), false) => {
                self.current_members += 1;
                tmpl.clone()
            }
            //新しいフォーメーションの作成
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // スタート位置
                let w_span = win_size.w / 2. + 100.;
                let h_span = win_size.h / 2. + 100.;
                let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
                let y = rng.gen_range(-h_span..h_span) as f32;
                let start = (x, y);

                // ピボットの位置
                let w_span = win_size.w / 4.;
                let h_span = win_size.h / 3. - 50.;
                let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

                // 円
                let radius = (rng.gen_range(80.0..150.), 100.);

                //角度
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // スピード
                let speed = BASE_SPEED;

                // フォーメーションをまとめる
                let formation = Formation {
                    start,
                    radius,
                    pivot,
                    speed,
                    angle,
                };

                self.current_template = Some(formation.clone());

                self.current_members = 1;

                formation
            }
        }
    }
}
