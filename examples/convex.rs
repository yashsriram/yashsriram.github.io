use bevy::prelude::*;

#[derive(Resource, Default)]
struct Inp {
    points: Vec<Vec2>,
}

#[derive(Resource, Default)]
struct Outp {
    line: Vec<Vec2>,
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
        .init_resource::<Inp>()
        .init_resource::<Outp>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw, add_point_on_left_click, algo))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw(mut gizmos: Gizmos, inp: Res<Inp>, output: Res<Outp>) {
    for point in &inp.points {
        gizmos.rect_2d(*point, 0.0, 3.0 * Vec2::ONE, Color::WHITE);
    }
    gizmos.linestrip_2d(output.line.clone(), Color::CYAN);
}

fn add_point_on_left_click(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
    mut inp: ResMut<Inp>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };
        inp.points.push(point);
    }
}

fn algo(inp: Res<Inp>, mut output: ResMut<Outp>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::H) && inp.points.len() >= 3 {
        let start = inp
            .points
            .iter()
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .map(|e| *e)
            .unwrap_or(Vec2::ZERO);
        let finish = inp
            .points
            .iter()
            .filter(|v| **v != start)
            .reduce(|all_on_left, v| {
                let cross = (*all_on_left - start)
                    .extend(0.0)
                    .cross((*v - start).extend(0.0));
                if cross.z < 0. {
                    v
                } else {
                    all_on_left
                }
            })
            .map(|e| *e)
            .unwrap_or(Vec2::ZERO);
        let mut hull: Vec<_> = vec![start];
        let mut rem: Vec<_> = inp.points.clone();
        loop {
            let last = *hull.last().unwrap();
            rem = rem.into_iter().filter(|v| *v != last).collect();
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
            hull.push(next);
            if next == finish {
                hull.push(start);
                break;
            }
        }
        output.line = hull;
    } else if keyboard.just_pressed(KeyCode::S) && inp.points.len() >= 3 {
        let start = inp
            .points
            .iter()
            .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
            .map(|e| *e)
            .unwrap_or(Vec2::ZERO);
        let mut spiral: Vec<_> = vec![start];
        let mut rem: Vec<_> = inp.points.clone();
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
        output.line = spiral;
    }
}
