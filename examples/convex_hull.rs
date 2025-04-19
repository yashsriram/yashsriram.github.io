use bevy::prelude::*;
use bricks::*;

#[derive(Resource, Default)]
struct Inp {
    points: Vec<Vec2>,
}

#[derive(Resource, Default)]
struct Outp {
    line: Vec<Vec2>,
}

bricks::visualize!({
    Inp(gizmos, inp) => {
        for point in &inp.points {
            gizmos.rect_2d(Isometry2d::from_translation(*point), 3.0 * Vec2::ONE, Color::WHITE);
        }
    },
    Outp(gizmos, outp) => {
        gizmos.linestrip_2d(outp.line.clone(), Color::linear_rgb(1.0, 0.0, 0.0));
    },
});

fn algo(inp: Res<Inp>, mut output: ResMut<Outp>) {
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
}

fn add_point_at_mouse_click(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut inp: ResMut<Inp>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = *camera_query;
        let Some(cursor_position) = windows.cursor_position() else {
            return;
        };
        let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };
        inp.points.push(point);
    }
}
