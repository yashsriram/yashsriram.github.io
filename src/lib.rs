use bevy::prelude::*;
pub mod turtle_walk;
pub use turtle_walk::*;
pub mod despawn_plugin;
pub use despawn_plugin::*;
pub mod add_vertex_plugins;
pub use add_vertex_plugins::*;

#[derive(Component)]
pub struct Output;
