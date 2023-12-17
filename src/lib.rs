use clap::{command, Arg, ArgAction };

#[derive(Debug)]
pub struct Input {
  files: Vec<String>,
  numbered_lines: bool,
  numbered_nonblank_lines: bool,
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
    .arg(
      Arg::new("input").action(ArgAction::Append).required(true)
    )
    .get_matches();

    Input {
      files: matches.get_many::<String>("input")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>(),
      numbered_lines: *matches.get_one::<bool>("numbered_lines").unwrap(),
      numbered_nonblank_lines: *matches.get_one::<bool>("numbered_nonblank_lines").unwrap(),
    }
}

