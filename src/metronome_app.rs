use crate::metronome_core::{new_metronome_core, MetronomeCore};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct MetronomeApp {
    pub metronome_core: Arc<Mutex<MetronomeCore>>,
    stream: cpal::Stream,
}

impl MetronomeApp {
    pub fn create_metronome_app(
        bpm: usize,
        time_per_bar: usize,
        subdiv: usize,
        score: Vec<usize>,
    ) -> Self {
        let mut core: MetronomeCore = new_metronome_core();

        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Default output device is not available");

        println!("Output device : {:?}", device.name());

        let config = device.default_output_config().unwrap();
        let format = config.sample_format();

        let stream_config: cpal::StreamConfig = config.into();

        println!("Default output config : {:?}", stream_config);

        let sample_rate = stream_config.sample_rate.0 as f32;

        core.set_sample_rate(sample_rate);
        core.setup(time_per_bar, subdiv, bpm, score);
        // core.set_sample_rate(sample_rate);
        // core.set_time_per_bar(time_per_bar);
        // core.set_time_subdivision(subdiv);
        // core.set_bpm(bpm);
        // core.init_score(score);

        let metronome_core_ref: Arc<Mutex<MetronomeCore>> = Arc::new(Mutex::new(core));

        let stream = match format {
            cpal::SampleFormat::F32 => {
                Self::run_f32(metronome_core_ref.clone(), &device, &stream_config)
            }
            cpal::SampleFormat::I16 => {
                Self::run_i16(metronome_core_ref.clone(), &device, &stream_config)
            }
            cpal::SampleFormat::U16 => {
                Self::run_u16(metronome_core_ref.clone(), &device, &stream_config)
            }
        };

        Self {
            metronome_core: metronome_core_ref,
            stream,
        }
    }

    fn run_f32(
        metronome: Arc<Mutex<MetronomeCore>>,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> cpal::Stream {
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);
        let nchannels = config.channels as usize;

        let stream: cpal::Stream = device
            .build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    //let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
                    let mut core_lock: MutexGuard<MetronomeCore> = metronome.lock().unwrap();

                    for frame in data.chunks_mut(nchannels) {
                        let s = core_lock.get_next_sample() as f32;
                        for sample in frame.iter_mut() {
                            *sample = s;
                        }
                    }
                },
                err_fn,
            )
            .unwrap();
        stream.play().unwrap();
        stream
    }

    fn run_i16(
        metronome: Arc<Mutex<MetronomeCore>>,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> cpal::Stream {
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);
        let nchannels = config.channels as usize;

        let stream: cpal::Stream = device
            .build_output_stream(
                config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    //let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
                    let mut core_lock: MutexGuard<MetronomeCore> = metronome.lock().unwrap();

                    for frame in data.chunks_mut(nchannels) {
                        let s = core_lock.get_next_sample() as i16;
                        for sample in frame.iter_mut() {
                            *sample = s;
                        }
                    }
                },
                err_fn,
            )
            .unwrap();
        stream.play().unwrap();
        stream
    }

    fn run_u16(
        metronome: Arc<Mutex<MetronomeCore>>,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> cpal::Stream {
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);
        let nchannels = config.channels as usize;

        let stream: cpal::Stream = device
            .build_output_stream(
                config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    //let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
                    let mut core_lock: MutexGuard<MetronomeCore> = metronome.lock().unwrap();

                    for frame in data.chunks_mut(nchannels) {
                        let s = core_lock.get_next_sample() as u16;
                        for sample in frame.iter_mut() {
                            *sample = s;
                        }
                    }
                },
                err_fn,
            )
            .unwrap();
        stream.play().unwrap();
        stream
    }

    // fn run<T>(
    //     metronome: Arc<Mutex<MetronomeCore>>,
    //     device: &cpal::Device,
    //     config: &cpal::StreamConfig,
    // ) -> cpal::Stream
    // where
    //     T: cpal::Sample,
    // {
    //     let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    //     let stream: cpal::Stream = device
    //         .build_output_stream(
    //             config,
    //             move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
    //                 //let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
    //                 let mut core = metronome.lock().unwrap();
    //                 let nchannels = config.channels as usize;
    //             },
    //             err_fn,
    //         )
    //         .unwrap();
    //     stream.play().unwrap();
    //     stream
    // }
}
