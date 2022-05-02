const PI: f32 = std::f32::consts::PI;
const NOISE_SIZE: usize = 128;

pub struct KPSound {
    noise: [f32; NOISE_SIZE],
    samples: Vec<f32>,
    pos: usize,
    sample_rate: f32,
}

pub fn new_kp_sound() -> KPSound {
    let kp: KPSound = KPSound {
        noise: [0.0; NOISE_SIZE],
        samples: vec![0.0],
        pos: 0,
        sample_rate: 0.0,
    };

    kp
}

impl KPSound {
    pub fn create_noise(&mut self) {
        self.noise = [0.0; NOISE_SIZE];
        let phase: f32 = 2.0 * PI / (NOISE_SIZE as f32);
        for i in 0..NOISE_SIZE {
            self.noise[i] = f32::sin(i as f32 * phase);
        }
    }

    pub fn generate_next_sample(&mut self) -> f32 {
        // https://crypto.stanford.edu/~blynn/sound/karplusstrong.html

        if self.pos < NOISE_SIZE {
            self.samples.push(self.noise[self.pos]);
        } else if self.pos > NOISE_SIZE {
            self.samples.push(
                0.92 * ((self.samples[self.pos - NOISE_SIZE]
                    + self.samples[self.pos - NOISE_SIZE - 1])
                    / 2.0),
            );
        } else {
            self.samples.push(0.0);
        };

        self.pos = self.pos + 1;

        self.samples[self.pos - 1]
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}
