use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

fn calculate_points(l: String) -> u64 {
    let numbers = l.split(": ").collect::<Vec<&str>>()[1]
        .split(" | ")
        .collect::<Vec<&str>>();
    let winning_set = numbers[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("failed to parse int"))
        .collect::<HashSet<u64>>();
    let mut winning_numbers_count = 0;
    for n in numbers[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("failed to parse int"))
        .collect::<HashSet<u64>>()
    {
        if winning_set.contains(&n) {
            winning_numbers_count += 1;
        }
    }
    if winning_numbers_count == 0 {
        0
    } else {
        2u64.pow(winning_numbers_count - 1)
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 4 part 1",
    long_about = "Advent of Code solution for day 4 part 1"
)]
struct Args {
    #[arg(long)]
    input_path: String,
}

fn main() {
    let package_name = env!("CARGO_PKG_NAME");
    let binary_name = env!("CARGO_BIN_NAME");
    let args = <Args as clap::Parser>::parse();
    let input_path = args.input_path;
    println!("Starting solution for {package_name} {binary_name} using file: {input_path}");

    let f = File::open(input_path).expect("Failed to open file");
    let reader = BufReader::new(f);
    let mut sum = 0;
    use std::io::BufRead;
    for l in reader.lines() {
        let line = l.expect("Failed to read line");
        sum += calculate_points(line);
    }
    println!("Solution: {sum}");
}
