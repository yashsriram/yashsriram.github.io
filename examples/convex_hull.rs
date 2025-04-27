use bricks::*;
use rand::*;

#[derive(Resource, Default)]
struct Inp {
    points: Vec<Vec2>,
}

#[derive(Resource, Default)]
struct Outp {
    line: Vec<Vec2>,
}

bricks::game_2d!(
    "convex hull",
    {
        Inp -> draw_inp,
        Outp -> draw_outp,
    }
);

fn init(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn on_mouse_click(In(point): In<Result<Vec2, ()>>, mut inp: ResMut<Inp>, mut outp: ResMut<Outp>) {
    inp.points.push(point.unwrap_or_default());

    outp.line = convex::hull::algo(inp.points.clone());
}

fn on_spacebar_press(mut outp: ResMut<Outp>, window: Single<&Window>, mut inp: ResMut<Inp>) {
    inp.points.clear();

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

    outp.line = convex::hull::algo(inp.points.clone());
}

fn draw_inp(mut gizmos: Gizmos, inp: Res<Inp>) {
    for point in &inp.points {
        gizmos.circle_2d(Isometry2d::from_translation(*point), 3.0, Color::WHITE);
    }
}

fn draw_outp(mut gizmos: Gizmos, outp: Res<Outp>) {
    gizmos.linestrip_2d(outp.line.clone(), Color::linear_rgb(1.0, 0.0, 0.0));
}
