use clap::{Parser, ValueEnum};
use game_of_life_lib::{GameState, Generation, Point};
use std::collections::HashSet;
use std::fmt;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, ValueEnum)]
enum OutputMode {
    Ascii,
    Unicode,
}

impl fmt::Display for OutputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputMode::Ascii => write!(f, "ascii"),
            OutputMode::Unicode => write!(f, "unicode"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "life")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Grid width
    #[arg(short, long)]
    width: usize,

    /// Grid height
    #[arg(short = 'H', long)]
    height: usize,

    /// Initial live cells in format "x1,y1;x2,y2;..."
    #[arg(short, long, default_value = "")]
    cells: String,

    /// Output mode
    #[arg(short, long, default_value = "ascii")]
    mode: OutputMode,

    /// Number of generations to run
    #[arg(short, long, default_value = "1")]
    generations: usize,

    /// Delay between generations in milliseconds
    #[arg(short, long, default_value = "100")]
    delay: u64,
}

#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid cell format")
    }
}

impl std::error::Error for ParseError {}

fn parse_point(s: &str) -> Result<Point, ParseError> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(ParseError);
    }
    let x = parts[0].parse().map_err(|_| ParseError)?;
    let y = parts[1].parse().map_err(|_| ParseError)?;
    Ok(Point::new(x, y))
}

fn parse_cells(input: &str) -> Result<HashSet<Point>, ParseError> {
    if input.is_empty() {
        return Ok(HashSet::new());
    }

    let cells: Result<HashSet<Point>, _> = input
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| parse_point(s))
        .collect();

    cells.map_err(|_| ParseError)
}

fn print_grid(gen: &Generation, unicode: bool) {
    for y in 0..gen.y_max {
        for x in 0..gen.x_max {
            let point = Point::new(x, y);
            if gen.live_cells.contains(&point) {
                if unicode {
                    print!("● ");
                } else {
                    print!("O ");
                }
            } else {
                if unicode {
                    print!("  ");
                } else {
                    print!("  ");
                }
            }
        }
        println!();
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = std::io::stdout().flush();
}

fn main() {
    let args = Args::parse();

    let live_cells = parse_cells(&args.cells).expect("Failed to parse cells");

    let unicode = matches!(args.mode, OutputMode::Unicode);

    let generation = Generation::new(live_cells, args.width, args.height);
    let mut state = GameState::new(generation.clone());

    print_grid(&generation, unicode);
    println!();
    clear_screen();

    for gen_idx in 0..args.generations {
        if gen_idx > 0 {
            clear_screen();
        }
        println!("Generation {}:", gen_idx);

        let generation = state.next().unwrap();
        print_grid(&generation, unicode);
        println!();

        thread::sleep(Duration::from_millis(args.delay));
    }
}
