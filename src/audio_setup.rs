use crate::oscillator::{new_sound_oscillator, Oscillator};
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
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    let sample_rate = config.sample_rate.0 as f32;
    let nchannels = config.channels as usize;

    println!("{} channels", nchannels);

    let mut osc: Oscillator = new_sound_oscillator();
    osc.set_sample_rate(sample_rate);
    osc.set_frequency(440.0);

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = match format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data_f32(data, &mut osc, nchannels)
            },
            err_fn,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                write_data_i16(data, &mut osc, nchannels)
            },
            err_fn,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config,
            move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                write_data_u16(data, &mut osc, nchannels)
            },
            err_fn,
        ),
    };

    stream
}

fn write_data_f32(data: &mut [f32], osc: &mut Oscillator, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = osc.generate_next_sample();
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}

fn write_data_i16(data: &mut [i16], osc: &mut Oscillator, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = osc.generate_next_sample() as i16;
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}

fn write_data_u16(data: &mut [u16], osc: &mut Oscillator, nchannels: usize) {
    for frame in data.chunks_mut(nchannels) {
        let s = osc.generate_next_sample() as u16;
        for sample in frame.iter_mut() {
            *sample = s;
        }
    }
}
