use rcat::get_args;
use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn main() {
    let input = get_args();
    for file in &input.files {
        if file == "-" {
            let stdin = io::stdin();
            for line in stdin.lines() {
                match line {
                    Ok(content) => println!("{content}"),
                    Err(error) => eprintln!("{error}"),
                }
            }
        } else {
            if let Err(error) = File::open(file) {
                eprintln!("rcat: {}: {}", file, error);
                continue
            }
            let content = File::open(file).unwrap();
            let buffer = BufReader::new(content);
            for line in buffer.lines() {
                match line {
                    Ok(sentence) => println!("{sentence}"),
                    Err(error) => eprintln!("{error}"),
                }
            }
        }
    }
}
