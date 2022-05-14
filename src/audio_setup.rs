use crate::metronome_core::new_metronome_core;
use crate::metronome_core::MetronomeCore;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::SampleFormat;
use cpal::SupportedStreamConfig;

pub struct AudioSetup {
    host: cpal::Host,
    device: cpal::Device,
    config: SupportedStreamConfig,
    format: SampleFormat,
    nchannels: usize,
    pub sample_rate: f32,
    // pub stream: cpal::Stream,
}

impl AudioSetup {
    // pub fn host_device_setup(&mut self) -> (cpal::Device, cpal::StreamConfig, cpal::SampleFormat) {
    //     let host = cpal::default_host();

    //     let device = host
    //         .default_output_device()
    //         .expect("Default output device is not available");

    //     println!("Output device : {:?}", device.name());

    //     let config = device.default_output_config().unwrap();

    //     println!("Default output config : {:?}", config);

    //     let format = config.sample_format();

    //     (device, config.config(), format)
    // }

    pub fn new_audio_setup() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Default output device is not available");

        println!("Output device : {:?}", device.name());

        let config = device.default_output_config().unwrap();

        println!("Default output config : {:?}", config);

        let format = config.sample_format();

        let sample_rate = config.config().sample_rate.0 as f32;
        let nchannels = config.config().channels as usize;

        // let stream: cpal::Stream = ();

        println!("new audio_setup");

        Self {
            host: host,
            device: device,
            config: config,
            format: format,
            // stream: stream,
            sample_rate: sample_rate,
            nchannels: nchannels,
        }
    }

    pub fn new_stream(
        &mut self,
        /*bpm: usize,
        time_signature: usize,
        subdiv: usize,*/
        metronome: &mut MetronomeCore,
    ) -> cpal::Stream {
        println!("new stream");
        make_stream(self, /*bpm, time_signature, subdiv, */ metronome).unwrap()
    }
}

fn make_stream(
    audio_setup: &mut AudioSetup,
    // bpm: usize,
    // time_signature: usize,
    // subdiv: usize,
    metronome: &mut MetronomeCore,
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    // let sample_rate = config.sample_rate.0 as f32;
    // let nchannels = config.channels as usize;

    // let mut metronome: MetronomeCore = new_metronome_core();
    // metronome.set_sample_rate(sample_rate);
    // metronome.set_time_per_bar(time_signature);
    // metronome.set_time_subdivision(subdiv);
    // metronome.set_bpm(bpm);
    // metronome.init_score();

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let mut metron: MetronomeCore = metronome.clone();

    let mut nchannels = audio_setup.nchannels;

    let stream = match audio_setup.format {
        // cpal::SampleFormat::F32 => audio_setup.device.build_output_stream(
        //     &audio_setup.config.config(),
        //     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        //         write_data_f32(data, &mut metron, nchannels)
        //     },
        //     err_fn,
        // ),
        cpal::SampleFormat::F32 => audio_setup.device.build_output_stream(
            &audio_setup.config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                println!("ttt");
                for frame in data.chunks_mut(nchannels) {
                    let s: f32;
                    s = metron.get_next_sample();
                    for sample in frame.iter_mut() {
                        *sample = s;
                    }
                }
            },
            err_fn,
        ),

        cpal::SampleFormat::I16 => audio_setup.device.build_output_stream(
            &audio_setup.config.config(),
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                write_data_i16(data, &mut metron, nchannels)
            },
            err_fn,
        ),
        cpal::SampleFormat::U16 => audio_setup.device.build_output_stream(
            &audio_setup.config.config(),
            move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                write_data_u16(data, &mut metron, nchannels)
            },
            err_fn,
        ),
    };

    stream
}

// pub fn make_strem(
//     device: &cpal::Device,
//     config: &cpal::StreamConfig,
//     format: &cpal::SampleFormat,
//     bpm: usize,
//     time_signature: usize,
//     subdiv: usize,
// ) -> Result<cpal::Stream, cpal::BuildStreamError> {
//     let sample_rate = config.sample_rate.0 as f32;
//     let nchannels = config.channels as usize;

//     let mut metronome: MetronomeCore = new_metronome_core();
//     metronome.set_sample_rate(sample_rate);
//     metronome.set_time_per_bar(time_signature);
//     metronome.set_time_subdivision(subdiv);
//     metronome.set_bpm(bpm);
//     metronome.init_score();

//     let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

//     let stream = match format {
//         cpal::SampleFormat::F32 => device.build_output_stream(
//             &config,
//             move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//                 write_data_f32(data, &mut metronome, nchannels)
//             },
//             err_fn,
//         ),
//         cpal::SampleFormat::I16 => device.build_output_stream(
//             &config,
//             move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
//                 write_data_i16(data, &mut metronome, nchannels)
//             },
//             err_fn,
//         ),
//         cpal::SampleFormat::U16 => device.build_output_stream(
//             &config,
//             move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
//                 write_data_u16(data, &mut metronome, nchannels)
//             },
//             err_fn,
//         ),
//     };

//     stream
// }

fn write_data_f32(data: &mut [f32], metronome: &mut MetronomeCore, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s: f32;
        s = metronome.get_next_sample();
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}

fn write_data_i16(data: &mut [i16], metronome: &mut MetronomeCore, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = metronome.get_next_sample() as i16;
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}

fn write_data_u16(data: &mut [u16], metronome: &mut MetronomeCore, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = metronome.get_next_sample() as u16;
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}
