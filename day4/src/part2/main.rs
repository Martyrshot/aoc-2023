use std::collections::{HashMap, HashSet};
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
    winning_numbers_count
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 4 part 2",
    long_about = "Advent of Code solution for day 4 part 2"
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
    let mut tickets = HashMap::<usize, usize>::new();
    // insert the first ticket
    tickets.insert(0, 1);
    use std::io::BufRead;
    let mut max_cards = 0u64;
    for (i, l) in reader.lines().enumerate() {
        let line = l.expect("Failed to read line");
        let winning_numbers = calculate_points(line);
        let shifted_index = i + 1;
        if let std::collections::hash_map::Entry::Vacant(e) = tickets.entry(i) {
            e.insert(1);
        }
        for j in shifted_index..shifted_index + (winning_numbers as usize) {
            if let std::collections::hash_map::Entry::Vacant(e) = tickets.entry(j) {
                e.insert(1);
            }
            let sum = tickets.get(&i).expect("should contain index: {i}")
                + tickets.get(&j).expect("should contain index: {j}");
            tickets
                .insert(j, sum)
                .expect("failed to insert sum into tickets");
        }
        max_cards += 1;
    }
    for (i, c) in tickets {
        if i as u64 >= max_cards {
            continue;
        }
        sum += c;
    }
    println!("Solution: {sum}");
}
