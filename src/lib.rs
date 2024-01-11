use clap::{command, Arg, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Args {
    pub files: Vec<String>,
    numbered_lines: bool,
    numbered_nonblank_lines: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

pub fn get_args() -> Args {
    let matches = command!()
        .arg(
            Arg::new("numbered_lines")
                .help("number all output lines")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("numbered_nonblank_lines")
                .help("number nonempty output lines, overrides -n")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show_ends")
                .help("display $ at end of each line")
                .short('E')
                .long("show-ends")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("squeeze_blank")
                .help("suppress repeated empty output lines")
                .short('s')
                .long("squeeze-blank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .get_matches();

    Args {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        numbered_lines: *matches.get_one::<bool>("numbered_lines").unwrap(),
        numbered_nonblank_lines: *matches.get_one::<bool>("numbered_nonblank_lines").unwrap(),
        show_ends: *matches.get_one::<bool>("show_ends").unwrap(),
        squeeze_blank: *matches.get_one::<bool>("squeeze_blank").unwrap(),
    }
}

pub fn display(file: &str, args: &Args) {
    if file == "-" {
        display_from_stdin(args);
    } else {
        display_from_file(file, args);
    }
}

fn display_from_stdin(args: &Args) {
    let stdin = io::stdin();
    let mut counter = 0;
    for line in stdin.lines() {
        match line {
            Ok(content) => format_line_to_display(&content, args, &mut counter),
            Err(error) => eprintln!("{error}"),
        }
    }
}

fn display_from_file(file: &str, args: &Args) {
    if let Err(error) = File::open(file) {
        return eprintln!("rcat: {}: {}", file, error);
    }

    let content = File::open(file).unwrap();
    let buffer = BufReader::new(content);
    let mut counter = 0;

    let mut current_line = String::new();

    for line in buffer.lines() {
        match line {
            Ok(sentence) => {
                if args.squeeze_blank && sentence.is_empty() && sentence == current_line {
                    continue;
                };
                current_line = sentence.clone();
                format_line_to_display(&sentence, args, &mut counter);
            }
            Err(error) => eprintln!("{error}"),
        }
    }
}

fn format_line_to_display(text: &str, args: &Args, counter: &mut u32) {
    if args.show_ends {
        if args.numbered_nonblank_lines {
            if text.is_empty() {
                println!("{text}$")
            } else {
                *counter += 1;
                println!("{counter} {text}$")
            }
        } else if args.numbered_lines {
            *counter += 1;
            println!("{counter} {text}$")
        } else {
            println!("{text}$")
        }
    } else if args.numbered_nonblank_lines {
        if text.is_empty() {
            println!("{text}")
        } else {
            *counter += 1;
            println!("{counter} {text}")
        }
    } else if args.numbered_lines {
        *counter += 1;
        println!("{counter} {text}")
    } else {
        println!("{text}")
    }
}
