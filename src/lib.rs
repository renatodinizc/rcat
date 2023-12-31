use clap::{command, Arg, ArgAction };
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
pub struct Input {
  pub files: Vec<String>,
  pub numbered_lines: bool,
  pub numbered_nonblank_lines: bool,
}

pub fn get_args() -> Input {
  let matches = command!()
    .arg(
        Arg::new("numbered_lines")
        .help("number all output lines")
        .short('n').long("number")
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("numbered_nonblank_lines")
        .help("number nonempty output lines, overrides -n")
        .short('b').long("number-nonblank")
        .action(ArgAction::SetTrue)
    )
    .arg(Arg::new("files").action(ArgAction::Append).default_value("-"))
    .get_matches();

    Input {
      files: matches.get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>(),
      numbered_lines: *matches.get_one::<bool>("numbered_lines").unwrap(),
      numbered_nonblank_lines: *matches.get_one::<bool>("numbered_nonblank_lines").unwrap(),
    }
}

pub fn display(file: &str, numbered_lines: bool, numbered_nonblank_lines: bool ) {
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
          return eprintln!("rcat: {}: {}", file, error);
      }
      let content = File::open(file).unwrap();
      let buffer = BufReader::new(content);
      let mut counter = 0;

      for line in buffer.lines() {
          match line {
              Ok(sentence) => format_line_to_display(sentence, numbered_lines, numbered_nonblank_lines, &mut counter),
              Err(error) => eprintln!("{error}"),
          }
      }
  }
}

fn format_line_to_display(text: String, numbered_lines: bool, numbered_nonblank_lines: bool, counter: &mut u32) {
    if numbered_nonblank_lines {
        if text == "" {
            println!("{text}")
        } else {
            *counter += 1;
            println!("{counter} {text}")
        }
     } else if numbered_lines {
        *counter += 1;
        println!("{counter} {text}")
    } else {
        println!("{text}")
    }
}
