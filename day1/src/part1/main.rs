use std::fs::File;
use std::io::BufReader;

fn main() {
    let package_name = env!("CARGO_PKG_NAME");
    let binary_name = env!("CARGO_BIN_NAME");
    println!("Starting solution for {package_name}: {binary_name}");

    let f = File::open("src/input.txt").expect("Failed to open file");
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
