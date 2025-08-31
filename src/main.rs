mod utils;

use crate::utils::{get_output, get_output_size, get_volume};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device found");
    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("Error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("No supported config")
        .with_max_sample_rate();
    let config = supported_config.config();

    let (terminal_size, _) = term_size::dimensions().expect("Could not get dimensions");
    let terminal_size_f32 = terminal_size as f32;

    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let volume = get_volume(&data);
                let output_size = get_output_size(terminal_size_f32, volume);
                println!("{}", "█".repeat(output_size as usize));
            },
            move |err| {
                eprintln!("Error: {:?}", err);
            },
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .expect("Failed to build stream");

    stream.play().expect("Failed to read stream");
    std::thread::sleep(std::time::Duration::from_secs(10));
}
