use crate::kp_sound::{new_kp_sound, KPSound};

pub struct MetronomeCore {
    click_states: [KPSound; 4],
    nb_sample_per_click: usize,
    bpm: usize,
    time_per_bar: usize,
    time_subdivision: usize,
    score_length: usize,
    sample_rate: f32,
    score: Vec<usize>,
    position_in_score: usize,
    current_sample_index: usize,
}

pub fn new_metronome_core() -> MetronomeCore {
    let kp1: KPSound = new_kp_sound();
    let kp2: KPSound = new_kp_sound();
    let kp3: KPSound = new_kp_sound();
    let kp4: KPSound = new_kp_sound();
    let mut metronome: MetronomeCore = MetronomeCore {
        click_states: [kp1, kp2, kp3, kp4],
        nb_sample_per_click: 1,
        bpm: 60,
        time_per_bar: 4,
        time_subdivision: 3,
        score_length: 1,
        sample_rate: 0.0,
        score: vec![0],
        position_in_score: 0,
        current_sample_index: 0,
    };

    metronome.init_metronome();
    metronome.init_score();

    metronome
}

impl MetronomeCore {
    fn init_metronome(&mut self) {
        for i in 0..4 {
            self.click_states[i].set_noise_length(i * 50);
        }
    }

    fn init_score(&mut self) {
        self.score_length = self.time_per_bar * self.time_subdivision;
        self.score = vec![0; self.score_length];

        for i in 0..self.score_length {
            if i % self.time_subdivision == 0 {
                // if main click
                self.score[i] = 1;
            } else {
                self.score[i] = 2;
            }
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.nb_sample_per_click = sample_rate as usize * 60 / (self.bpm * self.time_subdivision);
        for i in 0..4 {
            self.click_states[i].set_sample_rate(sample_rate);
        }
    }

    pub fn set_bpm(&mut self, bpm: usize) {
        self.bpm = bpm;
        self.nb_sample_per_click =
            self.sample_rate as usize * 60 / (self.bpm * self.time_subdivision);
    }

    pub fn get_next_sample(&mut self) -> f32 {
        let output: f32 = self.click_states[self.score[self.position_in_score]].get_next_sample();
        self.current_sample_index += 1;
        // shouldSync = false;
        if self.current_sample_index > self.nb_sample_per_click {
            // shouldSync = true;
            self.click_states[self.score[self.position_in_score]].set_current_sample(0);
            self.position_in_score += 1;
            self.current_sample_index = 0;
            if self.position_in_score >= self.score_length {
                self.position_in_score = 0;
            }
        }

        output
    }
}
