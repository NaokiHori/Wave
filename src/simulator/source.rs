use crate::random::Random;
use crate::simulator::Config;
use crate::simulator::NDIMS;

struct Quadratic {
    a: f32,
    b: f32,
    c: f32,
}

pub struct Distribution {
    pub mean: f32,
    pub deviation: f32,
}

pub struct Point {
    pub position: f32,
    interval: f32,
    distribution: Distribution,
    velocity: f32,
    quadratic: Quadratic,
}

impl Point {
    pub fn new(
        rng: &mut Random,
        distribution: Distribution,
        interval: f32,
        position: f32,
        velocity: f32,
    ) -> Self {
        let mut quadratic = Quadratic {
            a: 0f32,
            b: 0f32,
            c: 0f32,
        };
        update_quadratic(
            rng,
            &distribution,
            interval,
            position,
            velocity,
            &mut quadratic,
        );
        Self {
            position,
            interval,
            distribution,
            velocity,
            quadratic,
        }
    }

    pub fn update(&mut self, rng: &mut Random, dt: f32) {
        update_quadratic(
            rng,
            &self.distribution,
            self.interval,
            get_position(&self.quadratic, dt),
            get_velocity(&self.quadratic, dt),
            &mut self.quadratic,
        );
        self.position = get_position(&self.quadratic, 0f32);
        self.velocity = get_velocity(&self.quadratic, 0f32);
    }
}

fn get_position(q: &Quadratic, time: f32) -> f32 {
    q.a * time.powi(2) + q.b * time + q.c
}

fn get_velocity(q: &Quadratic, time: f32) -> f32 {
    2f32 * q.a * time + q.b
}

fn update_quadratic(
    rng: &mut Random,
    distribution: &Distribution,
    t_aft: f32,
    x_bef: f32,
    u_bef: f32,
    q: &mut Quadratic,
) {
    // determine
    //   x = a t^2 + b t + c,
    //   u = 2 a t + b,
    // using
    //   - current position x(t = t_a)
    //   - next    position x(t = t_b)
    //   - current velocity u(t = t_a)
    // |   t_bef^2   t_bef   1 | | a | = | x_bef |
    // |   t_aft^2   t_aft   1 | | b | = | x_aft |
    // | 2 t_bef         1   0 | | c | = | u_bef |
    // now we know t_bef = 0, giving
    // |       0       0   1 | | a | = | x_bef |
    // | t_aft^2   t_aft   1 | | b | = | x_aft |
    // |       0       1   0 | | c | = | u_bef |
    let x_aft = {
        let mean: f32 = distribution.mean;
        let dev: f32 = distribution.deviation;
        mean + rng.gen_range(0f32 - dev, 0f32 + dev)
    };
    q.a = (x_aft - x_bef - u_bef * t_aft) / t_aft.powi(2);
    q.b = u_bef;
    q.c = x_bef;
}

pub struct Source {
    pub points: [Point; NDIMS],
    pub amp: f32,
    omega: f32,
    time: f32,
}

impl Source {
    pub fn new(config: &Config, rng: &mut Random) -> Self {
        let position = [0.5f32 * config.lengths[0], 0.5f32 * config.lengths[1]];
        let velocity = [0f32, 0f32];
        let points = [
            Point::new(
                rng,
                Distribution {
                    mean: position[0],
                    deviation: 16f32 * config.lengths[0],
                },
                25f32,
                position[0],
                velocity[0],
            ),
            Point::new(
                rng,
                Distribution {
                    mean: position[1],
                    deviation: 16f32 * config.lengths[1],
                },
                25f32,
                position[1],
                velocity[1],
            ),
        ];
        let amp = 1f32;
        let omega = 1.5f32 * std::f32::consts::PI;
        let time = 0f32;
        Self {
            points,
            amp,
            omega,
            time,
        }
    }

    pub fn get_position(&self) -> [f32; NDIMS] {
        [self.points[0].position, self.points[1].position]
    }

    pub fn update(&mut self, rng: &mut Random, dt: f32) {
        self.points[0].update(rng, dt);
        self.points[1].update(rng, dt);
        self.time += dt;
        self.amp = (self.time * self.omega).sin();
    }
}
