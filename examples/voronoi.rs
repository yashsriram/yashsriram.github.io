use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    // Input
    let mut rng = rand::thread_rng();
    let a = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    let b = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    let c = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
    // Algo
    struct PointVecForm {
        p: Vec2,
        v: Vec2,
    }
    impl PointVecForm {
        fn bisect(p1: Vec2, p2: Vec2) -> Self {
            PointVecForm {
                p: (p1 + p2) / 2.0,
                v: (p1 - p2).perp(),
            }
        }

        fn xn_with(&self, other: &Self) -> Option<Vec2> {
            // L1: self.point + t_1 * self.vec
            // L2: other.point + t_2 * other.vec
            // self.vec.x * t_1 + (-1.0 * other.vec.x) * t_2 + (self.point.x - other.point.x) = 0
            // self.vec.y * t_1 + (-1.0 * other.vec.y) * t_2 + (self.point.y - other.point.y) = 0
            let existence_matrix =
                Mat2::from_cols_array(&[self.v.x, self.v.y, -1.0 * other.v.x, -1.0 * other.v.y]);
            let x_matrix = Mat2::from_cols_array(&[
                self.p.x - other.p.x,
                self.p.y - other.p.y,
                -1.0 * other.v.x,
                -1.0 * other.v.y,
            ]);
            if existence_matrix.determinant() == 0.0 {
                return None;
            } else {
                let t_1 = -1.0 * x_matrix.determinant() / existence_matrix.determinant();
                let intersection = self.p + self.v * t_1;
                Some(intersection)
            }
        }
    }

    // Perpendicular bisector of a and b
    let ab_perp = PointVecForm::bisect(a, b);
    // Perpendicular bisector of b and c
    let bc_perp = PointVecForm::bisect(b, c);
    let xn = ab_perp.xn_with(&bc_perp).unwrap();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Update, move |mut giz: Gizmos, windows: Query<&Window>| {
            giz.rect_2d(a, 0.0, Vec2::ONE * 5.0, Color::WHITE);
            giz.rect_2d(b, 0.0, Vec2::ONE * 5.0, Color::WHITE);
            giz.rect_2d(c, 0.0, Vec2::ONE * 5.0, Color::WHITE);
            giz.linestrip_2d([a, b, c, a], Color::WHITE);
            giz.rect_2d(xn, 0.0, Vec2::ONE * 10.0, Color::CYAN);
            giz.line_2d(ab_perp.p, xn, Color::CYAN);
            giz.line_2d(bc_perp.p, xn, Color::CYAN);
            giz.line_2d(c, xn, Color::CYAN);
            giz.circle_2d(xn, (xn - a).length(), Color::RED);
        })
        .run();
}
