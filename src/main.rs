mod utils;

use crate::utils::{generate_color, get_output_size, get_volume, print_devices};
use clap::{Parser, Subcommand};
use cpal::Device;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::panic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(short = 'd', long, default_value_t = 1)]
        device: usize,

        #[arg(short = 'b', long, default_value_t = 5)]
        samples_per_bar: i8,

        #[arg(short = 's', long, default_value_t = 10)]
        seconds: u64,

        #[arg(short = 'f', long, default_value_t = false)]
        run_forever: bool,
    },
    ViewDevices,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::ViewDevices => {
            let host = cpal::default_host();
            let devices: Vec<Device> = host
                .input_devices()
                .expect("Failed to get devices")
                .collect();
            print_devices(&devices);
        }
        Commands::Run {
            device,
            samples_per_bar,
            seconds,
            run_forever,
        } => {
            let host = cpal::default_host();
            let devices: Vec<Device> = host
                .input_devices()
                .expect("Failed to get devices")
                .collect();
            let input_device;

            match devices.len() {
                0 => panic!("No devices were found"),
                _ => {
                    if (device > devices.len()) | (device == 0) {
                        panic!(
                            "You chose an invalid device number. See available devices by using the view-devices subcommand."
                        );
                    }

                    let input_device_idx = device - 1;

                    println!(
                        "Using device {} - {}",
                        device,
                        devices[input_device_idx]
                            .name()
                            .expect("Device name not found")
                    );
                    input_device = &devices[input_device_idx];
                }
            }

            let mut supported_configs_range = input_device
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

            let mut sample_count = 0;
            let mut volume_sum = 0.0;

            let stream = match supported_config.sample_format() {
                cpal::SampleFormat::F32 => input_device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        sample_count += 1;
                        volume_sum += get_volume(&data);

                        if sample_count == samples_per_bar {
                            let average_volume = volume_sum / (sample_count as f32);
                            let output_size = get_output_size(terminal_size_f32, average_volume);
                            print!("{}", color);
                            println!("{}", "█".repeat(output_size as usize));
                            sample_count = 0;
                            volume_sum = 0.0;
                        }
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
            if run_forever {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
            } else {
                std::thread::sleep(std::time::Duration::from_secs(seconds));
            }
        }
    }
}
