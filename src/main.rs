mod utils;

use crate::utils::{generate_color, get_output_size, get_volume};
use cpal::Device;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{io, panic};

fn main() {
    let host = cpal::default_host();
    let devices: Vec<Device> = host
        .input_devices()
        .expect("Failed to get devices")
        .collect();
    let device;

    match devices.len() {
        0 => panic!("No devices found"),
        1 => device = &devices[0],
        _ => {
            println!("\nMultiple input devices were detected");

            let mut input_string = String::new();

            for (idx, potential_device) in devices.iter().enumerate() {
                println!(
                    "{}. {}",
                    idx + 1,
                    potential_device.name().expect("Could not get name")
                );
            }

            println!("\nEnter the number of your desired device below: ");
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let input_number: usize = input_string.trim().parse().expect("Could not parse input");

            if (input_number > devices.len()) | (input_number == 0) {
                panic!("You entered an invalid number");
            }

            device = &devices[input_number - 1];
        }
    }

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
    let color = generate_color();

    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let volume = get_volume(&data);
                let output_size = get_output_size(terminal_size_f32, volume);
                print!("{}", color);
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
