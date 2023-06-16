use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::env::args;

fn main() {
    println!("==========================================================");
    print!("\n");
    println!("                 Random number generator                  ");
    print!("\n\n");
    println!("Usage: rng <NUMBER_OF_ITEMS> [FILENAME] [start] [end]");
    println!("\nBy default it'll generate random");
    println!("numbers between 0 to 100 unless specified");
    println!("==========================================================");

    let mut args = args();
    let _ = args.next();
    let number_or_items = args
        .next()
        .expect("Number of items is required")
        .parse::<i64>()
        .unwrap();
    let filename = args.next().unwrap_or("random_numbers".into());
    let start = args.next().unwrap_or(0.to_string()).parse::<u32>().unwrap();
    let end = args
        .next()
        .unwrap_or(100.to_string())
        .parse::<u32>()
        .unwrap();

    let mut rng = rand::thread_rng();
    let f = File::create(filename).expect("unable to create file");
    let mut f = BufWriter::new(f);

    for _ in 1..=number_or_items {
        let x: u32 = rng.gen_range(start..end);
        writeln!(f, "{}", x).expect("unable to write");
    }

}
