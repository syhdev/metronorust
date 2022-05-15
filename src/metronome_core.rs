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
        time_subdivision: 1,
        score_length: 1,
        sample_rate: 0.0,
        score: vec![0],
        position_in_score: 0,
        current_sample_index: 0,
    };

    metronome.init_metronome();

    metronome
}

impl MetronomeCore {
    fn init_metronome(&mut self) {
        for i in 0..4 {
            self.click_states[i].set_noise_length(i * 30);
        }
    }

    pub fn setup(&mut self, time_per_bar: usize, subdiv: usize, bpm: usize, score: Vec<usize>) {
        self.set_time_per_bar(time_per_bar);
        self.set_time_subdivision(subdiv);
        self.set_bpm(bpm);
        self.init_score(score);
    }

    fn init_score(&mut self, score: Vec<usize>) {
        self.score_length = self.time_per_bar * self.time_subdivision;

        self.score = score;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.nb_sample_per_click = sample_rate as usize * 60 / (self.bpm * self.time_subdivision);
        for i in 0..4 {
            self.click_states[i].set_sample_rate(sample_rate);
        }
    }

    fn set_bpm(&mut self, bpm: usize) {
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

    /// Set the metronome core's time per bar.
    fn set_time_per_bar(&mut self, time_per_bar: usize) {
        self.time_per_bar = time_per_bar;
        self.score_length = self.time_per_bar * self.time_subdivision;
        self.position_in_score = 0;
        self.current_sample_index = 0;
    }

    /// Set the metronome core's time subdivision.
    fn set_time_subdivision(&mut self, time_subdivision: usize) {
        self.time_subdivision = time_subdivision;
        self.score_length = self.time_per_bar * self.time_subdivision;
        self.position_in_score = 0;
        self.current_sample_index = 0;
    }
}
