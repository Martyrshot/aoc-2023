extern crate clap;

use std::fs::File;
use std::io::BufReader;

#[derive(Clone)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

impl Bag {
    fn min_cubes_power(&self, game: String) -> usize {
        let mut tmp = self.clone();
        let game_pair = game
            .split(": ")
            .collect::<Vec<&str>>();
        let groups = game_pair[1]
            .split("; ")
            .collect::<Vec<&str>>();

        for group in groups {
            let reveals = group.split(", ").collect::<Vec<&str>>();
            for reveal in reveals {
                let pair = reveal.split(' ').collect::<Vec<&str>>();
                let count = pair[0].parse::<usize>().expect("failed to parse count");
                let colour = pair[1];
                match colour {
                    "red" => tmp.red = std::cmp::max(tmp.red, count),
                    "green" => tmp.green = std::cmp::max(tmp.green, count),
                    "blue" => tmp.blue = std::cmp::max(tmp.blue, count),
                    _ => unreachable!(),
                }
            }
        }
        tmp.red * tmp.green * tmp.blue
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 2 part 2",
    long_about = "Advent of Code solution for day 2 part 2"
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
    let bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for l in reader.lines() {
        let line = l.expect("Failed to read line");
        sum += bag.min_cubes_power(line);
    }
    println!("Solution: {sum}");
}
