use bricks::*;
use ordered_float::OrderedFloat;
use rand::prelude::*;

#[derive(Resource)]
struct Terrain {
    x_start: f32,
    x_stop: f32,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain {
            x_start: -500.,
            x_stop: 500.,
        }
    }
}

impl Terrain {
    fn is_inside_range(&self, x: f32) -> bool {
        return self.x_start <= x && x <= self.x_stop;
    }

    fn at(&self, x: f32) -> Option<f32> {
        if self.is_inside_range(x) {
            if 0. < x && x < 300. {
                return Some(0.);
            }
            let terms = [
                (x * 0.01).cos().powi(2),
                (x * 0.01).sin(),
                (x * 0.03).cos().powi(2),
                (x * 0.01).sin(),
                (-(x * 0.01).abs()).exp(),
                // (x * 0.05).sin(),
                // 0.,
            ];
            Some(terms.iter().sum::<f32>() * 40.0)
        } else {
            return None;
        }
    }

    fn sample_x(&self) -> f32 {
        let mut rng = rand::thread_rng();
        self.x_start + rng.gen::<f32>() * (self.x_stop - self.x_start)
    }
}

#[derive(Resource, Default)]
struct Agent {
    x: f32,
    y: f32,
}

impl Agent {
    fn propagate(&mut self) {
        self.x += 0.1;
    }
}

#[derive(Resource)]
struct Particles {
    xs_and_weights: [(f32, f32); 100],
}

impl Default for Particles {
    fn default() -> Self {
        Particles {
            xs_and_weights: [(0., 0.); 100],
        }
    }
}

impl Particles {
    fn on_terrain(&mut self, terrain: Res<Terrain>) {
        let num_particles = self.xs_and_weights.len();
        for (idx, (x, _)) in self.xs_and_weights.iter_mut().enumerate() {
            *x = terrain.x_start
                + (terrain.x_stop - terrain.x_start) * (idx + 1) as f32 / num_particles as f32;
        }
    }

    fn propagate(&mut self) {
        for (x, _) in self.xs_and_weights.iter_mut() {
            *x += 0.1;
        }
    }

    fn update(&mut self, terrain: &Terrain, measured_y: f32) {
        for (x, weight) in self.xs_and_weights.iter_mut() {
            let measured_y_at_particle = terrain.at(*x);
            if measured_y_at_particle.is_none() {
                *weight = 0.;
                continue;
            }
            // The order of diff here does not matter since we use to sample from normal
            // distribution
            let diff_in_measurement = measured_y - measured_y_at_particle.unwrap();
            // Compute N(x)
            use std::f32::consts::PI;
            let std = 1.;
            *weight =
                1. / (2. * PI).sqrt() * std * (-0.5 * (diff_in_measurement / std).powi(2)).exp();
        }
    }

    fn resample(&mut self) {
        // Sort according to weights
        self.xs_and_weights
            .sort_by_key(|(_, weight)| OrderedFloat(*weight));
        // Place least weighted 10 particles near top weighed 10 particles
        let num_particles = self.xs_and_weights.len();
        let mut rng = rand::thread_rng();
        for idx in 0..2 {
            let noise = rng.gen::<f32>() - 0.5;
            self.xs_and_weights[0].0 = self.xs_and_weights[num_particles - 1 - idx].0 + noise * 40.;
        }
        // Make all weights equal
        for (_, weight) in self.xs_and_weights.iter_mut() {
            *weight = 0.;
        }
    }
}

bricks::game_2d! {
    "particle filter",
    {
        Terrain -> draw_terrain,
        Agent -> update_and_draw_agent,
        Particles -> update_and_draw_particles,
    }
}

fn init(mut commands: Commands, mut terrain: ResMut<Terrain>) {
    commands.spawn(Camera2d::default());
}

fn draw_terrain(mut gizmos: Gizmos, terrain: Res<Terrain>) {
    let points = (0..4000)
        .map(|idx| idx as f32 / 4000.0 * (terrain.x_stop - terrain.x_start) + terrain.x_start)
        .map(|x| Vec2::new(x, terrain.at(x).unwrap_or_default()));
    gizmos.linestrip_2d(points, Color::WHITE);
}

fn update_and_draw_agent(mut gizmos: Gizmos, mut agent: ResMut<Agent>, terrain: Res<Terrain>) {
    if terrain.at(agent.x).is_some() {
        agent.propagate();
    }
    gizmos.circle_2d(
        Isometry2d::from_xy(agent.x, agent.y),
        10.,
        Color::srgb(0., 1., 0.),
    );
    gizmos.line_2d(
        [terrain.x_start, agent.y].into(),
        [terrain.x_stop, agent.y].into(),
        Color::srgba(0., 1., 0., 0.5),
    );
    gizmos.line_2d(
        [agent.x, agent.y].into(),
        [agent.x, terrain.at(agent.x).unwrap_or_default()].into(),
        Color::srgba(0., 1., 0., 0.5),
    );
    gizmos.line_2d(
        [terrain.x_start, terrain.at(agent.x).unwrap_or_default()].into(),
        [terrain.x_stop, terrain.at(agent.x).unwrap_or_default()].into(),
        Color::srgba(0., 1., 1., 0.5),
    );
}

fn update_and_draw_particles(
    mut gizmos: Gizmos,
    mut partices: ResMut<Particles>,
    agent: Res<Agent>,
    terrain: Res<Terrain>,
) {
    if let Some(measured_y) = terrain.at(agent.x) {
        partices.propagate();
        partices.update(&terrain, measured_y);
        partices.resample();
    }

    for (x, weight) in partices.xs_and_weights.iter() {
        gizmos.circle_2d(
            Isometry2d::from_xy(*x, agent.y),
            5.,
            Color::srgb(1., 0., 0.),
        );
        gizmos.line_2d(
            [*x, agent.y].into(),
            [*x, terrain.at(*x).unwrap_or_default()].into(),
            Color::srgba(1., 0., 0., 0.5),
        );
    }
}

fn on_spacebar_press(
    terrain: Res<Terrain>,
    mut agent: ResMut<Agent>,
    mut partices: ResMut<Particles>,
) {
    agent.x = terrain.sample_x();
    agent.y = 200.;

    partices.on_terrain(terrain);
}

fn on_mouse_click(In(point): In<Result<Vec2, ()>>) {}
