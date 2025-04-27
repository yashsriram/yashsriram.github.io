use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CuboidWithHoldSpace {
    pub size: Vec3,
    pub hole_radius: f32,
}
