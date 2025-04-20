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
    "convex spiral",
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

fn on_spacebar_press(inp: Res<Inp>, mut output: ResMut<Outp>) {
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

fn on_mouse_click(In(point): In<Result<Vec2, ()>>, mut inp: ResMut<Inp>) {
    let Ok(point) = point else {
        return;
    };
    inp.points.push(point);
}
