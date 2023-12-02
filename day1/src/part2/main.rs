use std::fs::File;
use std::io::BufReader;

fn word_to_digit(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seve" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn to_digit(buf: &[u8]) -> Option<u32> {
    (buf[0] as char)
        .to_digit(10)
        .or_else(|| {
            let max_length = 5;
            for i in 2..(max_length+1) {
                if i >= buf.len() {
                    return None;
                }
                let r = std::str::from_utf8(&buf[0..i+1])
                    .ok()
                    .and_then(|s| {
                        word_to_digit(s)
                    });
                if r.is_none() {
                    continue;
                }
                return r;
            }
            None
        })
}

#[derive(clap::Parser)]
#[command(
    author = "Martyrshot",
    version = "1.0",
    about = "Advent of Code day 1 part 2",
    long_about = "Advent of Code solution for day 1 part 2"
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
        
        let mut should_break = false;
        for i in 0..line_len {
            if should_break {
                break;
            }
            if let Some(x) = to_digit(&bytes[i..]) {
                sum += x * 10;
                should_break = true;
            }
        }
        should_break = false;
        for i in (0..line_len).rev() {
            if should_break {
                break;
            }
            if let Some(x) = to_digit(&bytes[i..]) {
                sum += x;
                should_break = true;
            }

        }
    }
    println!("Solution: {sum}");
}
