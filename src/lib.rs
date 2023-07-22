use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use std::marker::PhantomData;
use std::marker::Send;
use std::marker::Sync;

pub struct TurtleWalk(pub Vec<Vec3>);

impl From<TurtleWalk> for Mesh {
    fn from(turtle_walk: TurtleWalk) -> Mesh {
        let vertices: Vec<_> = turtle_walk
            .0
            .iter()
            .map(|point| ([point.x, point.y, point.z], [0., 0., 1.], [0., 0.]))
            .collect();
        let indices = Indices::U32((0..turtle_walk.0.len()).map(|e| e as u32).collect());
        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, p, _)| *p).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

#[derive(Component)]
pub struct Vertex;

pub struct AddVertexPlugin;

impl Plugin for AddVertexPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_vertex);
    }
}

fn add_vertex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor) = window.cursor_position() {
            let semi_viewport_axes = Vec2::new(window.width() / 2., window.height() / 2.);
            let click = cursor - semi_viewport_axes;
            let new_vertex_position = Vec3::new(click.x, click.y, 0.);
            commands.spawn((
                Vertex,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(new_vertex_position),
                    ..default()
                },
            ));
        }
    }
}

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
