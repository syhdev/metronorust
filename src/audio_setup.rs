use crate::click_widget;
use crate::metronome_core::new_metronome_core;
use crate::metronome_core::MetronomeCore;
use cpal::traits::{DeviceTrait, HostTrait};

pub fn host_device_setup() -> (cpal::Device, cpal::StreamConfig, cpal::SampleFormat) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Default output device is not available");

    println!("Output device : {:?}", device.name());

    let config = device.default_output_config().unwrap();

    println!("Default output config : {:?}", config);

    let format = config.sample_format();

    (device, config.config(), format)
}

pub fn make_strem(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    format: &cpal::SampleFormat,
    bpm: usize,
    time_signature: usize,
    subdiv: usize,
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    let sample_rate = config.sample_rate.0 as f32;
    let nchannels = config.channels as usize;

    let mut metronome: MetronomeCore = new_metronome_core();
    metronome.set_sample_rate(sample_rate);
    metronome.set_time_per_bar(time_signature);
    metronome.set_time_subdivision(subdiv);
    metronome.set_bpm(bpm);
    metronome.init_score();

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = match format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data_f32(data, &mut metronome, nchannels)
            },
            err_fn,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                write_data_i16(data, &mut metronome, nchannels)
            },
            err_fn,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config,
            move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                write_data_u16(data, &mut metronome, nchannels)
            },
            err_fn,
        ),
    };

    stream
}

fn write_data_f32(data: &mut [f32], metronome: &mut MetronomeCore, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = metronome.get_next_sample();
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
