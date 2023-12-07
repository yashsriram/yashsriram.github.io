use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Default)]
struct State {
    points: Vec<Vec2>,
    start: Vec2,
    spiral: Vec<Vec2>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500., 400.).into(),
                canvas: Some("#interactive".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                draw_state,
                add_point_on_left_click,
                add_few_on_f_press,
                algo_on_space_press,
                clear_on_r_press,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut state: ResMut<State>) {
    commands.spawn(Camera2dBundle::default());
    let mut rng = rand::thread_rng();
    let points: [Vec2; 20] = core::array::from_fn(|_| {
        Vec2::new(
            300.0 * (rng.gen::<f32>() - 0.5),
            300.0 * (rng.gen::<f32>() - 0.5),
        )
    });
    state.points.extend(points);
}

fn draw_state(mut gizmos: Gizmos, state: Res<State>) {
    for point in &state.points {
        gizmos.rect_2d(*point, 0.0, 3.0 * Vec2::ONE, Color::WHITE);
    }
    gizmos.rect_2d(state.start, 1.0, 10.0 * Vec2::ONE, Color::GREEN);
    gizmos.linestrip_2d(state.spiral.clone(), Color::YELLOW);
}

fn add_point_on_left_click(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };
        state.points.push(point);
    }
}

fn add_few_on_f_press(mut state: ResMut<State>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F) {
        let mut rng = rand::thread_rng();
        let points: [Vec2; 20] = core::array::from_fn(|_| {
            Vec2::new(
                300.0 * (rng.gen::<f32>() - 0.5),
                300.0 * (rng.gen::<f32>() - 0.5),
            )
        });
        state.points.extend(points);
    }
}

fn algo_on_space_press(mut state: ResMut<State>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let start = state
            .points
            .iter()
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .map(|e| *e)
            .unwrap_or(Vec2::ZERO);
        let mut spiral: Vec<_> = vec![start];
        let mut rem: Vec<_> = state.points.clone();
        loop {
            let last = *spiral.last().unwrap();
            rem = rem.into_iter().filter(|v| *v != last).collect();
            if rem.len() == 0 {
                break;
            }
            let next = rem
                .clone()
                .into_iter()
                .reduce(|all_on_right, v| {
                    let cross = ((all_on_right - last).extend(0.0)).cross((v - last).extend(0.0));
                    if cross.z > 0. {
                        v
                    } else {
                        all_on_right
                    }
                })
                .map(|e| e)
                .unwrap_or(Vec2::ZERO);
            spiral.push(next);
        }
        state.start = start;
        state.spiral = spiral;
    }
}

fn clear_on_r_press(mut state: ResMut<State>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::R) {
        state.points.clear();
    }
}
