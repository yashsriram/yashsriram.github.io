use bricks::*;

#[derive(Resource)]
struct MLE {
    mean: f32,
    std_dev: f32,
    x_start: f32,
    x_stop: f32,
}

impl MLE {
    const X_SCALE: f32 = 1000.;
    const Y_SCALE: f32 = 50.;

    fn at(&self, x: f32) -> Option<f32> {
        if self.std_dev < 1e-6f32 {
            return None;
        }
        // Compute N(x)
        use std::f32::consts::PI;
        let x = x / Self::X_SCALE;
        let n_x = 1. / ((2. * PI).sqrt() * self.std_dev)
            * (-0.5 * ((x - self.mean) / self.std_dev).powi(2)).exp();
        let n_x = n_x * Self::Y_SCALE;
        return Some(n_x);
    }

    fn update(&mut self, msmts: &Measurements) {
        let updated_mean = msmts.0.iter().map(|m| m.x).sum::<f32>() / (msmts.0.len() as f32);
        let updated_variance = msmts
            .0
            .iter()
            .map(|m| (m.x - updated_mean).powi(2))
            .sum::<f32>()
            / (msmts.0.len() as f32);
        let updated_std_dev = updated_variance.sqrt();
        self.mean = updated_mean / Self::X_SCALE;
        self.std_dev = updated_std_dev / Self::X_SCALE;
    }
}

impl Default for MLE {
    fn default() -> Self {
        MLE {
            mean: 0.,
            std_dev: 0.,
            x_start: -500.,
            x_stop: 500.,
        }
    }
}

#[derive(Resource, Default)]
struct Measurements(Vec<Vec2>);

bricks::game_2d! {
    "maximum likelihood estimate",
    {
        MLE -> draw_estimate,
        Measurements -> draw_measurement,
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn draw_estimate(mut gizmos: Gizmos, mle: Res<MLE>) {
    // X axis
    gizmos.line_2d(
        [mle.x_start, 0.].into(),
        [mle.x_stop, 0.].into(),
        Color::WHITE,
    );
    // Curve
    let points = (0..4000)
        .map(|idx| idx as f32 / 4000.0 * (mle.x_stop - mle.x_start) + mle.x_start)
        .map(|x| Vec2::new(x, mle.at(x).unwrap_or_default()));
    gizmos.linestrip_2d(points, Color::srgb(0., 1., 0.));
    // Mean
    gizmos.circle_2d(
        Isometry2d::from_xy(mle.mean * MLE::X_SCALE, 0.),
        10.,
        Color::srgb(1., 0., 0.),
    );
}

fn draw_measurement(mut gizmos: Gizmos, measurements: Res<Measurements>) {
    for msmt in measurements.0.iter() {
        gizmos.circle_2d(
            Isometry2d::from_xy(msmt.x, 0.),
            5.,
            Color::srgba(1., 1., 0., 0.1),
        );
        gizmos.line_2d(
            [msmt.x, 50.].into(),
            [msmt.x, -50.].into(),
            Color::srgba(1., 1., 0., 0.1),
        );
    }
}

fn on_spacebar_press(mut msmts: ResMut<Measurements>, mut mle: ResMut<MLE>) {
    msmts.0.clear();
    *mle = MLE::default();
}

fn on_mouse_click(
    In(point): In<Result<Vec2, ()>>,
    mut msmts: ResMut<Measurements>,
    mut mle: ResMut<MLE>,
) {
    let Ok(point) = point else {
        return;
    };
    msmts.0.push(point);
    mle.update(&msmts);
}
