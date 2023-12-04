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

    fn is_digit(c: char) -> bool {
        matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
    }

    fn check_up(&self, (x, y): (usize, usize)) -> bool {
        if y == 0 {
            false
        } else {
            Self::is_digit(self.data[y - 1][x])
        }
    }

    fn check_down(&self, (x, y): (usize, usize)) -> bool {
        if y == self.column_len - 1 {
            false
        } else {
            Self::is_digit(self.data[y + 1][x])
        }
    }

    fn check_left(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 {
            false
        } else {
            Self::is_digit(self.data[y][x - 1])
        }
    }

    fn check_right(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 {
            false
        } else {
            Self::is_digit(self.data[y][x + 1])
        }
    }

    fn check_upleft(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 || y == 0 {
            false
        } else {
            Self::is_digit(self.data[y - 1][x - 1])
        }
    }

    fn check_upright(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 || y == 0 {
            false
        } else {
            Self::is_digit(self.data[y - 1][x + 1])
        }
    }

    fn check_downleft(&self, (x, y): (usize, usize)) -> bool {
        if x == 0 || y == self.column_len - 1 {
            false
        } else {
            Self::is_digit(self.data[y + 1][x - 1])
        }
    }

    fn check_downright(&self, (x, y): (usize, usize)) -> bool {
        if x == self.row_len - 1 || y == self.column_len - 1 {
            false
        } else {
            Self::is_digit(self.data[y + 1][x + 1])
        }
    }

    fn find_up(&self, (x, y): (usize, usize)) -> u128 {
        if y == 0 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x < self.row_len - 1 && Self::is_digit(self.data[y - 1][x + 1]) {
            starting_x = x + 1;
            ending_x = starting_x;
        } else if x > 0 && Self::is_digit(self.data[y - 1][x - 1]) {
            starting_x = x - 1;
            ending_x = starting_x;
        } else if Self::is_digit(self.data[y - 1][x]) {
            starting_x = x;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y - 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y - 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y - 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_down(&self, (x, y): (usize, usize)) -> u128 {
        if y == self.column_len - 1 {
            return 0;
        }
        let mut num = 0;
        let mut digit = 1;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x < self.row_len - 1 && Self::is_digit(self.data[y + 1][x + 1]) {
            starting_x = x + 1;
            ending_x = starting_x;
        } else if x > 0 && Self::is_digit(self.data[y + 1][x - 1]) {
            starting_x = x - 1;
            ending_x = starting_x;
        } else if Self::is_digit(self.data[y + 1][x]) {
            starting_x = x;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y + 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y + 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y + 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_left(&self, (x, y): (usize, usize)) -> u128 {
        if x <= 1 {
            return 0;
        }
        let mut num = 0;
        let mut digit = 1;
        let starting_x = x - 1;
        let mut ending_x = starting_x;
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y][ending_x]) {
            ending_x -= 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_right(&self, (x, y): (usize, usize)) -> u128 {
        if x >= self.row_len - 1 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x = x + 1;
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y][starting_x]) {
            starting_x += 1;
        }
        for i in (x..starting_x + 1).rev() {
            if let Some(d) = self.data[y][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_upleft(&self, (x, y): (usize, usize)) -> u128 {
        if y == 0 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x > 0 && Self::is_digit(self.data[y - 1][x - 1]) {
            starting_x = x - 1;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y - 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y - 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y - 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_upright(&self, (x, y): (usize, usize)) -> u128 {
        if y == 0 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x < self.row_len - 1 && Self::is_digit(self.data[y - 1][x + 1]) {
            starting_x = x + 1;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y - 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y - 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y - 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_downleft(&self, (x, y): (usize, usize)) -> u128 {
        if y == self.column_len - 1 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x > 0 && Self::is_digit(self.data[y + 1][x - 1]) {
            starting_x = x - 1;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y + 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y + 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y + 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn find_downright(&self, (x, y): (usize, usize)) -> u128 {
        if y == self.column_len - 1 {
            return 0;
        }
        let mut num = 0u128;
        let mut digit = 1u128;
        let mut starting_x: usize;
        let mut ending_x: usize;
        if x < self.row_len - 1 && Self::is_digit(self.data[y + 1][x + 1]) {
            starting_x = x + 1;
            ending_x = starting_x;
        } else {
            return 0;
        }
        // Sweep left to find the end
        while ending_x > 0 && Self::is_digit(self.data[y + 1][ending_x]) {
            ending_x -= 1;
        }
        // Sweep right to find the start
        while starting_x < self.row_len - 1 && Self::is_digit(self.data[y + 1][starting_x]) {
            starting_x += 1;
        }
        for i in (ending_x..starting_x + 1).rev() {
            if let Some(d) = self.data[y + 1][i].to_digit(10) {
                num += (d as u128) * digit;
                digit *= 10;
            }
        }
        num
    }

    fn gear_ratio(&self, idx: (usize, usize)) -> u128 {
        let mut nums: Vec<u128> = Vec::with_capacity(2);
        if self.check_up(idx) {
            let num = self.find_up(idx);
            nums.push(num);
        } else {
            if self.check_upleft(idx) {
                let num = self.find_upleft(idx);
                nums.push(num);
            }
            if self.check_upright(idx) {
                let num = self.find_upright(idx);
                nums.push(num);
            }
        }
        if self.check_left(idx) {
            let num = self.find_left(idx);
            nums.push(num);
        }
        if self.check_right(idx) {
            let num = self.find_right(idx);
            nums.push(num);
        }
        if self.check_down(idx) {
            let num = self.find_down(idx);
            nums.push(num);
        } else {
            if self.check_downleft(idx) {
                let num = self.find_downleft(idx);
                nums.push(num);
            }
            if self.check_downright(idx) {
                let num = self.find_downright(idx);
                nums.push(num);
            }
        }
        if nums.len() != 2 {
            0
        } else {
            nums[0] * nums[1]
        }
    }

    fn gear_ratio_sum(self) -> u128 {
        let mut sum = 0u128;
        for (y, row) in self.data.iter().enumerate() {
            for (x, _) in row.iter().enumerate().rev() {
                if self.data[y][x] == '*' {
                    sum += self.gear_ratio((x, y));
                }
            }
        }
        sum
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 3 part 2",
    long_about = "Advent of Code solution for day 3 part 2"
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
    println!("Solution: {}", schematic.gear_ratio_sum());
}
