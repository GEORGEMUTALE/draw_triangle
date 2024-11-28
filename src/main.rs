use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Cli {
    height: u32,
    #[clap(default_value = "*")]
    character: String,
}

fn draw_triangle(height: u32, character: &str) {
    for i in 0..height {
        let spaces = height - i - 1;
        let chars = 2 * i + 1;
        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            character.repeat(chars as usize).green() 
        );
    }
}

fn main() {
    let args = Cli::parse();
    draw_triangle(args.height, &args.character);
}
