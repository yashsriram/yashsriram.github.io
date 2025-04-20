use bricks::*;

#[derive(Resource, Default)]
struct Inp {
    points: Vec<Vec2>,
}

#[derive(Resource, Default)]
struct Outp {
    line: Vec<Vec2>,
}

bricks::vis_2d!(
    "convex hull",
    {
        Inp(gizmos, inp) => {
            for point in &inp.points {
                gizmos.circle_2d(Isometry2d::from_translation(*point), 3.0, Color::WHITE);
            }
        },
        Outp(gizmos, outp) => {
            gizmos.linestrip_2d(outp.line.clone(), Color::linear_rgb(1.0, 0.0, 0.0));
        },
    }
);

fn init(window: Single<&Window>, mut inp: ResMut<Inp>) {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let sample = Vec2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5)
            * Vec2::new(
                window.resolution.physical_width() as f32 - 200.0,
                window.resolution.physical_height() as f32 - 200.0,
            )
            * (1.0 / window.resolution.scale_factor());
        inp.points.push(sample);
    }
}

fn on_mouse_click(In(point): In<Result<Vec2, ()>>, mut inp: ResMut<Inp>) {
    let Ok(point) = point else {
        return;
    };
    inp.points.push(point);
}

fn on_spacebar_press(inp: Res<Inp>, mut output: ResMut<Outp>) {
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
