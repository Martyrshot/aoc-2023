use std::fs::File;
use std::io::BufReader;

struct Schematic {
    data: Vec<Vec<char>>,
    row_len: usize,
    column_len: usize,
}

impl Schematic {
    fn new() -> Self {
        Schematic {
            data: Vec::new(),
            row_len: 0,
            column_len: 0,
        }
    }

    fn insert_row(&mut self, row: Vec<char>) {
        self.row_len = row.len();
        self.column_len += 1;
        self.data.append(&mut vec![row])
    }

    fn is_symbol(c: char) -> bool {
        !matches!(
            c,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.'
        )
    }

    fn check_up(&self, (x, y): (usize, usize)) -> bool {
        if y == 0 {
            false
        } else {
            Self::is_symbol(self.data[y - 1][x])
        }
    }

    fn check_down(&self, (x, y): (usize, usize)) -> bool {
        if y == self.column_len - 1 {
            false
        } else {
            Self::is_symbol(self.data[y + 1][x])
        }
    }

    fn check_left(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 {
            false
        } else {
            Self::is_symbol(self.data[y][x - 1])
        }
    }

    fn check_right(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 {
            false
        } else {
            Self::is_symbol(self.data[y][x + 1])
        }
    }

    fn check_upleft(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 || y == 0 {
            false
        } else {
            Self::is_symbol(self.data[y - 1][x - 1])
        }
    }

    fn check_upright(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 || y == 0 {
            false
        } else {
            Self::is_symbol(self.data[y - 1][x + 1])
        }
    }

    fn check_downleft(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 || y == self.column_len - 1 {
            false
        } else {
            Self::is_symbol(self.data[y + 1][x - 1])
        }
    }

    fn check_downright(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 || y == self.column_len - 1 {
            false
        } else {
            Self::is_symbol(self.data[y + 1][x + 1])
        }
    }

    fn symbol_adjacent(&self, idx: (usize, usize)) -> bool {
        self.check_upleft(idx)
            || self.check_up(idx)
            || self.check_upright(idx)
            || self.check_right(idx)
            || self.check_downright(idx)
            || self.check_down(idx)
            || self.check_downleft(idx)
            || self.check_left(idx)
    }

    fn part_number_sum(self) -> u64 {
        let mut sum = 0u64;
        for (y, row) in self.data.iter().enumerate() {
            let mut num = 0u64;
            let mut adjacent = false;
            let mut digit = 1u64;
            for (x, c) in row.iter().enumerate().rev() {
                c.to_digit(10)
                    .map(|d| {
                        num += (d as u64) * digit;
                        digit *= 10;
                        if self.symbol_adjacent((x, y)) {
                            adjacent = true;
                        }
                        x
                    })
                    .or_else(|| {
                        if adjacent {
                            sum += num;
                            adjacent = false;
                        }
                        num = 0;
                        digit = 1;
                        None
                    });
            }
            if adjacent {
                sum += num;
            }
        }
        sum
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 3 part 1",
    long_about = "Advent of Code solution for day 3 part 1"
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
    let mut schematic = Schematic::new();
    use std::io::BufRead;
    for l in reader.lines() {
        let line = l.expect("Failed to read line");
        schematic.insert_row(line.chars().collect::<Vec<char>>());
    }
    println!("Solution: {}", schematic.part_number_sum());
}
