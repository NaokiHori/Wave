mod converter;
mod coordinate;
mod init_field;
mod integrate;

pub const NDIMS: usize = 2;

#[derive(Clone)]
pub struct Config {
    pub lengths: [f64; NDIMS],
    pub nitems: [usize; NDIMS],
    pub param_c2: f64,
    pub param_nu: f64,
}

pub struct Simulator {
    config: Config,
    converter: converter::Converter,
    pos: Vec<f64>,
    vel: Vec<f64>,
    acc: Vec<f64>,
    pos_phys: Vec<f64>,
}

impl Simulator {
    pub fn new(config: &Config) -> Self {
        let nitems_total: usize = config.nitems[0] * config.nitems[1];
        let mut pos = vec![0f64; nitems_total];
        let mut vel = vec![0f64; nitems_total];
        let mut acc = vec![0f64; nitems_total];
        let pos_phys = vec![0f64; nitems_total];
        let mut converter = converter::Converter::new(&config.nitems);
        init_field::init_field(config, &mut converter, &mut pos, &mut vel, &mut acc);
        Self {
            config: config.clone(),
            converter,
            pos,
            vel,
            acc,
            pos_phys,
        }
    }

    pub fn integrate(&mut self, dt_max: f64, time: &mut f64) {
        let mut dt = dt_max;
        integrate::integrate(
            &self.config,
            *time,
            &mut self.pos,
            &mut self.vel,
            &mut self.acc,
            &mut dt,
        );
        *time += dt;
    }

    pub fn pos(&mut self) -> &[f64] {
        let pos_phys: &mut [f64] = &mut self.pos_phys;
        self.converter.freq_to_phys(&self.pos, pos_phys);
        pos_phys
    }
}
