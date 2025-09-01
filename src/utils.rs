use rand::Rng;

const COLORS: [&str; 12] = [
    "\x1b[38;5;196m", // bright red
    "\x1b[38;5;202m", // orange
    "\x1b[38;5;226m", // yellow
    "\x1b[38;5;46m",  // bright green
    "\x1b[38;5;51m",  // cyan
    "\x1b[38;5;21m",  // blue
    "\x1b[38;5;201m", // bright magenta
    "\x1b[38;5;208m", // orange-red
    "\x1b[38;5;82m",  // lime green
    "\x1b[38;5;93m",  // pink
    "\x1b[38;5;199m", // hot pink
    "\x1b[38;5;214m", // bright peach
];

pub fn get_volume(data: &[f32]) -> f32 {
    let mut total_sum = 0.0;
    let mut total_count = 0.0;

    for value in data.iter() {
        total_sum += value.abs();
        total_count += 1.0;
    }

    total_sum / total_count
}

pub fn get_output_size(terminal_size: f32, volume: f32) -> i32 {
    (volume * terminal_size) as i32
}

pub fn generate_color() -> &'static str {
    let mut rng = rand::rng();
    let index = rng.random_range(0..COLORS.len());
    COLORS[index]
}
