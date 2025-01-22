
// this is our first command application
use clap::Parser;
use colored::*;
use indicatif::ProgressBar;
use log::info;
use std::thread;
use std::time::Duration;

/// Command-line arguments
#[derive(Parser)]
struct Cli {
    /// Height of the triangle
    height: u32,
    /// Character to draw the triangle (default: '*')
    #[clap(default_value = "*")]
    character: String,
}

/// Function to draw a triangle
fn draw_triangle(height: u32, character: &str, pb: &ProgressBar) {
    for i in 0..height {
        let spaces = height - i - 1;
        let chars = 2 * i + 1;

        // Log the current step
        info!("Drawing row {}: {} spaces, {} characters", i + 1, spaces, chars);

        // Simulate some delay for visualization
        thread::sleep(Duration::from_millis(100));
        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            character.repeat(chars as usize).green()
        );

        // Update the progress bars
        pb.inc(1);
    }
    pb.finish_with_message("Drawing completed!");
}

fn main() {
    // Initialize logging
    env_logger::init();

    // Parse CLI arguments
    let args = Cli::parse();

    // Log the input
    info!("Starting triangle drawing with height: {} and character: '{}'", args.height, args.character);

    // Create and configure the progress bar
    let pb = ProgressBar::new(args.height as u64);
    pb.set_message("Drawing the triangle...");

    // Draw the triangle
    draw_triangle(args.height, &args.character, &pb);

    // Log completion
    info!("Triangle drawing finished successfully.");
}
