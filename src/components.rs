use bevy::prelude::Component;

//region --- Common Components
#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable{
    pub auto_despawn:bool,
}
//endregion

//region --- Player Components
#[derive(Component)]
pub struct Player;
//endregion