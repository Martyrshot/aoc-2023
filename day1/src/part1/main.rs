use std::fs::File;
use std::io::BufReader;

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 1 part 1",
    long_about = "Advent of Code solution for day 1 part 1"
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
        let bytes = line.as_bytes();
        let line_len = line.len();
        
        for b in bytes.iter().take(line_len) {
            if let Some(x) = (*b as char).to_digit(10) {
                sum += x * 10;
                break;
            }
        }
        
        for i in (0..line_len).rev() {
            if let Some(x) = (bytes[i] as char).to_digit(10) {
                sum += x;
                break;
            }

        }
    }
    println!("Solution: {sum}");
}
