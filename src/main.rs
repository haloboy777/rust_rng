use anyhow::{ensure, Ok, Result};
use rand::Rng;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};

use clap::Parser;

/// Simple program to generate random numbers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of items to generate
    #[arg(short = 'n', long)]
    number_of_items: u32,

    /// Filename to write the numbers to
    #[arg(short = 'o', long = "out")]
    filename: Option<String>,

    /// Starting point in range
    #[arg(short = 's', long, default_value_t = 1)]
    start: isize,

    /// Ending point in range (included)
    #[arg(short = 'e', long, default_value_t = 100)]
    end: isize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let filename = args.filename;
    let start = args.start;
    let end = args.end;
    let number_of_items = args.number_of_items;

    ensure!(
        start < end,
        "Range start is greater than or equal to range end: {} {}",
        start,
        end
    );

    let mut rng = rand::thread_rng();
    if filename.is_some() {
        let f = File::create(filename.unwrap_or("random_number".into()))
            .expect("unable to create file");
        let mut f = BufWriter::new(f);
        for _ in 1..=number_of_items {
            let x: isize = rng.gen_range(start..=end);
            writeln!(f, "{x}").expect("unable to write");
        }
    } else {
        let mut lock = stdout().lock();
        for _ in 1..=number_of_items {
            let x: isize = rng.gen_range(start..=end);
            // println!("{}", x);
            writeln!(lock, "{x}").expect("unable to write");
        }
    }
    Ok(())
}
