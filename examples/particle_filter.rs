use simple_vis::*;

#[derive(Resource, Default)]
struct Terrain {
    a: f32,
    b: f32,
}

impl Terrain {
    fn at(&self, x: f32) -> f32 {
        ((x / 100.).cos().powi(2) + (x / 90.).sin() - (x / 80.).sin().powi(2) + (x / 70.).sin())
            * 100.0
    }
}

simple_vis::simple_vis! {
    "particle filter",
    {
        Terrain -> draw_terrain,
    }
}

fn init(mut commands: Commands, mut terrain: ResMut<Terrain>) {
    commands.spawn(Camera2d::default());
    terrain.a = -400.0;
    terrain.b = 400.0;
}

fn draw_terrain(mut gizmos: Gizmos, terrain: Res<Terrain>) {
    let points = (0..4000)
        .map(|idx| idx as f32 / 4000.0 * (terrain.b - terrain.a) + terrain.a)
        .map(|x| Vec2::new(x, terrain.at(x)));
    gizmos.linestrip_2d(points, Color::WHITE);
}

fn on_spacebar_press() {}

fn on_mouse_click(In(point): In<Result<Vec2, ()>>) {}
