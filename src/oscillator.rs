pub const TWO_PI: f32 = std::f32::consts::PI * 2.0;
pub const WAVETABLE_LENGTH: u16 = 16384;

pub enum OscillatorMode {
    OscillatorModeSine,
    OscillatorModeSquare,
    OscillatorModeSaw,
    OscillatorModeTriangle,
}

pub struct Oscillator {
    pub frequency: f32,
    pub sample_rate: f32,
    pub pos: f32,
    pub incr: f32,
    pub posint: u32,
    pub frq_ti: f32,
    pub frq_two_pi: f32,
    pub modulation: f32,
    pub oscillator_mode: OscillatorMode,
}

pub trait SetOscillator {
    fn set_mode(&mut self, mode: OscillatorMode);

    fn set_sample_rate(&mut self, sr: f32);

    fn set_frequency(&mut self, freq: f32);

    fn reset(&mut self);

    fn generate_next_sample(&mut self) -> f32;

    fn update_increment(&mut self);
}

pub fn new_sound_oscillator() -> Oscillator {
    let osc: Oscillator = Oscillator {
        frequency: 0.0,
        sample_rate: 0.0,
        pos: 0.0,
        incr: 0.0,
        posint: 0,
        frq_ti: 0.0,
        frq_two_pi: 0.0,
        modulation: 0.0,
        oscillator_mode: OscillatorMode::OscillatorModeSine,
    };

    println!("Oscillator created");

    osc
}

impl Oscillator {
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.frq_ti = WAVETABLE_LENGTH as f32 / self.sample_rate;
        self.frq_two_pi = TWO_PI / self.sample_rate;
        self.update_increment();
        println!("Osc sample rate: {}", self.sample_rate);
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
        self.update_increment();
        println!("Osc frequency: {}", self.frequency);
    }

    fn reset(&mut self) {
        self.pos = 0.0;
    }

    pub fn generate_next_sample(&mut self) -> f32 {
        let mut next_sample = 0.0;

        match self.oscillator_mode {
            OscillatorMode::OscillatorModeSine => {
                next_sample = (self.pos + self.modulation).sin();

                self.pos = self.pos + self.incr;
                if self.pos >= TWO_PI {
                    self.pos = self.pos - TWO_PI;
                }
            }
            OscillatorMode::OscillatorModeSquare => {
                if self.pos + self.modulation <= std::f32::consts::PI {
                    next_sample = 1.0;
                } else {
                    next_sample = -1.0;
                }

                self.pos = self.pos + self.incr;
                if self.pos >= TWO_PI {
                    self.pos = self.pos - TWO_PI;
                }
            }
            OscillatorMode::OscillatorModeSaw => {
                next_sample = 1.0 - (2.0 * (self.pos + self.modulation) / TWO_PI);

                self.pos = self.pos + self.incr;
                if self.pos >= TWO_PI {
                    self.pos = self.pos - TWO_PI;
                }
            }
            OscillatorMode::OscillatorModeTriangle => {
                next_sample = -1.0 + (2.0 * (self.pos + self.modulation) / TWO_PI);
                next_sample = 2.0 * (next_sample.abs() - 0.5);

                self.pos = self.pos + self.incr;
                if self.pos >= TWO_PI {
                    self.pos = self.pos - TWO_PI;
                }
            }
        }
        next_sample
    }

    fn update_increment(&mut self) {
        self.incr = self.frequency * self.frq_two_pi;
    }
}
