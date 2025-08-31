pub fn get_volume(data: &[f32]) -> f32 {
    let mut total_sum = 0.0;
    let mut total_count = 0.0;

    for value in data.iter() {
        total_sum += value.abs();
        total_count += 1.0;
    }

    total_sum / total_count
}

pub fn get_output_size(terminal_size: i32, volume: f32) -> i32 {
    (volume * (terminal_size as f32)) as i32
}

pub fn get_output(terminal_size: i32, output_size: i32) -> String {
    let mut output = String::new();
    for _ in 0..output_size {
        output.push('*');
    }
    for _ in output_size..terminal_size {
        output.push(' ');
    }
    output
}
