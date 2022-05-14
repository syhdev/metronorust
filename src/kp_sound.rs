const PI: f32 = std::f32::consts::PI;

#[derive(Clone)]

pub struct KPSound {
    noise: Vec<f32>,
    kp_samples: Vec<f32>,
    //pos: usize,
    noise_length: usize,
    sample_rate: f32,
    current_sample: usize,
}

pub fn new_kp_sound() -> KPSound {
    let kp: KPSound = KPSound {
        noise: vec![0.0],
        kp_samples: vec![0.0],
        //pos: 0,
        noise_length: 64,
        sample_rate: 0.0,
        current_sample: 0,
    };

    kp
}

impl KPSound {
    fn create_noise(&mut self) {
        self.noise = vec![0.0; self.noise_length];
        // let phase: f32 = 2.0 * PI / (self.noise_length as f32);
        // for i in 0..self.noise_length {
        //     self.noise[i] = f32::sin(i as f32 * phase);
        // }
        for i in 0..self.noise_length {
            self.noise[i] = f32::sin(i as f32 * 2.0 * PI / (self.noise_length as f32));
        }
    }

    fn create_kp_table(&mut self) {
        let kp_table_length = self.sample_rate as usize;
        self.kp_samples = vec![0.0; kp_table_length];
        for i in 0..kp_table_length {
            if i < self.noise_length {
                self.kp_samples[i] = self.noise[i];
            } else {
                self.kp_samples[i] = self.kp_samples[i - self.noise_length] * 0.90;
            }
        }
    }

    pub fn init_click_sound(&mut self) {
        self.create_noise();
        self.create_kp_table();
    }

    // pub fn generate_next_sample(&mut self) -> f32 {
    //     // https://crypto.stanford.edu/~blynn/sound/karplusstrong.html

    //     if self.pos < NOISE_SIZE {
    //         self.samples.push(self.noise[self.pos]);
    //     } else if self.pos > NOISE_SIZE {
    //         self.samples.push(
    //             0.92 * ((self.samples[self.pos - NOISE_SIZE]
    //                 + self.samples[self.pos - NOISE_SIZE - 1])
    //                 / 2.0),
    //         );
    //     } else {
    //         self.samples.push(0.0);
    //     };

    //     self.pos = self.pos + 1;

    //     self.samples[self.pos - 1]
    // }

    pub fn set_current_sample(&mut self, current_sample: usize) {
        self.current_sample = current_sample;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.create_kp_table();
    }

    pub fn set_noise_length(&mut self, noise_length: usize) {
        self.noise_length = noise_length;
        self.init_click_sound();
    }

    pub fn get_next_sample(&mut self) -> f32 {
        let mut sample: f32 = 0.0;

        if self.current_sample < self.sample_rate as usize {
            sample = self.kp_samples[self.current_sample];
        }

        self.current_sample += 1;

        sample
    }
}
