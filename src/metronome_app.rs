use std::{default, rc::Rc, sync::mpsc::Receiver};

use crate::{
    audio_setup::AudioSetup,
    gui::GUI,
    metronome_core::{new_metronome_core, MetronomeCore},
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct MetronomeApp {
    pub core: MetronomeCore,
    pub audio_setup: AudioSetup,
    // pub gui: GUI,
}

impl MetronomeApp {
    pub fn new_metronome(bpm: usize, time_signature: usize, time_subdiv: usize) -> Self {
        let mut core = new_metronome_core();

        let mut audio_setup = AudioSetup::new_audio_setup();

        let sr = audio_setup.sample_rate;

        core.set_sample_rate(sr);
        core.set_time_per_bar(time_signature);
        core.set_time_subdivision(time_subdiv);
        core.set_bpm(bpm);
        core.init_score();

        audio_setup
            .new_stream(/*bpm, time_signature, time_subdiv, */ &mut core)
            .play()
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(5000));

        // let mut gui = GUI::create_gui();

        Self {
            core: core,
            audio_setup: audio_setup,
            // gui: gui,
        }
    }

    pub fn deal_rx(&mut self) {
        // while (true) {
        // }
    }

    pub fn app_launch(&mut self) {
        //self.gui.app_main_loop();
    }

    pub fn add_bpm(&mut self, y: usize) {
        self.core.set_bpm(self.core.bpm + y);
    }

    pub fn init_score(&mut self) {
        self.core.init_score();
    }

    pub fn set_bpm(&mut self, bpm: usize) {
        self.core.set_bpm(bpm);
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.core.set_sample_rate(sample_rate);
    }

    pub fn set_time_per_bar(&mut self, time_signature: usize) {
        self.core.set_time_per_bar(time_signature);
    }

    pub fn set_time_subdivision(&mut self, time_subdiv: usize) {
        self.core.set_time_subdivision(time_subdiv);
    }

    pub fn get_next_sample(&mut self) -> f32 {
        self.core.get_next_sample()
    }
}
