use crate::random::Random;
use crate::simulator::Config;
use crate::simulator::NDIMS;

struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

pub struct Distribution {
    pub mean: f64,
    pub deviation: f64,
}

pub struct Point {
    pub position: f64,
    interval: f64,
    distribution: Distribution,
    velocity: f64,
    quadratic: Quadratic,
}

impl Point {
    pub fn new(
        rng: &mut Random,
        distribution: Distribution,
        interval: f64,
        position: f64,
        velocity: f64,
    ) -> Self {
        let mut quadratic = Quadratic {
            a: 0f64,
            b: 0f64,
            c: 0f64,
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

    pub fn update(&mut self, rng: &mut Random, dt: f64) {
        update_quadratic(
            rng,
            &self.distribution,
            self.interval,
            get_position(&self.quadratic, dt),
            get_velocity(&self.quadratic, dt),
            &mut self.quadratic,
        );
        self.position = get_position(&self.quadratic, 0f64);
        self.velocity = get_velocity(&self.quadratic, 0f64);
    }
}

fn get_position(q: &Quadratic, time: f64) -> f64 {
    q.a * time.powi(2) + q.b * time + q.c
}

fn get_velocity(q: &Quadratic, time: f64) -> f64 {
    2f64 * q.a * time + q.b
}

fn update_quadratic(
    rng: &mut Random,
    distribution: &Distribution,
    t_aft: f64,
    x_bef: f64,
    u_bef: f64,
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
        let mean: f64 = distribution.mean;
        let dev: f64 = distribution.deviation;
        mean + rng.gen_range(0f64 - dev, 0f64 + dev)
    };
    q.a = (x_aft - x_bef - u_bef * t_aft) / t_aft.powi(2);
    q.b = u_bef;
    q.c = x_bef;
}

pub struct Source {
    pub points: [Point; NDIMS],
    pub amp: f64,
    omega: f64,
    time: f64,
}

impl Source {
    pub fn new(config: &Config, rng: &mut Random) -> Self {
        let position = [0.5f64 * config.lengths[0], 0.5f64 * config.lengths[1]];
        let velocity = [0f64, 0f64];
        let points = [
            Point::new(
                rng,
                Distribution {
                    mean: position[0],
                    deviation: 16f64 * config.lengths[0],
                },
                25f64,
                position[0],
                velocity[0],
            ),
            Point::new(
                rng,
                Distribution {
                    mean: position[1],
                    deviation: 16f64 * config.lengths[1],
                },
                25f64,
                position[1],
                velocity[1],
            ),
        ];
        let amp = 1f64;
        let omega = 1.5f64 * std::f64::consts::PI;
        let time = 0f64;
        Self {
            points,
            amp,
            omega,
            time,
        }
    }

    pub fn get_position(&self) -> [f64; NDIMS] {
        [self.points[0].position, self.points[1].position]
    }

    pub fn update(&mut self, rng: &mut Random, dt: f64) {
        self.points[0].update(rng, dt);
        self.points[1].update(rng, dt);
        self.time += dt;
        self.amp = (self.time * self.omega).sin();
    }
}
