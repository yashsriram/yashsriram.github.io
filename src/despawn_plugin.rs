use bevy::prelude::*;
use std::marker::PhantomData;
use std::marker::Send;
use std::marker::Sync;

pub struct DespawnOnKeyRPlugin<S> {
    selector: PhantomData<S>,
}

impl<S> Default for DespawnOnKeyRPlugin<S> {
    fn default() -> Self {
        Self {
            selector: PhantomData,
        }
    }
}

impl<S: 'static + Send + Sync + Component> Plugin for DespawnOnKeyRPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_on_key_r::<S>);
    }
}

fn despawn_on_key_r<S: Component>(
    mut commands: Commands,
    color_materials: Query<Entity, With<S>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for entity in &color_materials {
            commands.entity(entity).despawn();
        }
        *clear_color = ClearColor(Color::BLACK);
    }
}
