use std::fs::File;
use std::io::BufReader;

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

impl Bag {
    fn is_valid_game(&self, game: String) -> (bool, usize) {
        let mut res = true;
        let game_pair = game.split(": ").collect::<Vec<&str>>();
        let game_id = game_pair[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .expect("failed to parse game id");
        let groups = game_pair[1].split("; ").collect::<Vec<&str>>();

        for group in groups {
            let reveals = group.split(", ").collect::<Vec<&str>>();
            for reveal in reveals {
                let pair = reveal.split(' ').collect::<Vec<&str>>();
                let count = pair[0].parse::<usize>().expect("failed to parse count");
                let colour = pair[1];
                match colour {
                    "red" => res = res && (self.red >= count),
                    "green" => res = res && (self.green >= count),
                    "blue" => res = res && (self.blue >= count),
                    _ => unreachable!(),
                }
            }
        }
        (res, game_id)
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 2 part 1",
    long_about = "Advent of Code solution for day 2 part 1"
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
        red: 12,
        green: 13,
        blue: 14,
    };
    for l in reader.lines() {
        let line = l.expect("Failed to read line");
        let r = bag.is_valid_game(line);
        if r.0 {
            sum += r.1;
        }
    }
    println!("Solution: {sum}");
}
