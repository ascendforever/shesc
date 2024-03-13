extern crate shlex;
extern crate structopt;
use std::io::{BufReader, BufRead};
use crate::structopt::StructOpt;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLIArguments::from_args();
    let separator = args.separator.unwrap_or('\n');

    if args.lines.is_empty() {
        escape_stdin(separator)?;
    } else {
        escape_lines(separator, &args.lines)?;
        if args.both {
            escape_stdin(separator)?;
        }
    }

    Ok(())
}

fn escape_lines(separator: char, buf: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for line in buf {
        print!(
            "{}{}",
            shlex::try_quote(line)?,
            separator
        );
    }
    Ok(())
}

fn escape_stdin(separator: char) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    for line in reader.split(separator as u8) {
        print!(
            "{}{}",
            shlex::try_quote(std::str::from_utf8(&line?)?)?,
            separator
        )
    }
    Ok(())
}

fn parse_char(s: &str) -> Result<char, String> {
    if s.len() != 1 {
        return Err(String::from("Separator must be a single character"));
    }
    let c = s.chars().next().unwrap();
    if !c.is_ascii() {
        return Err(String::from("Character must be ASCII"));
    }
    Ok(c)
}

#[derive(StructOpt)]
#[structopt(
    about="Simply escapes input\nControl characters may not be escaped",
)]
struct CLIArguments {
    #[structopt(long,
                help="Escape CLI arg `lines` and then standard input (default is one or the other, not both)")]
    both: bool,

    #[structopt(short, long, value_name="SEPARATOR",
                help="Separator [default: '\\n']",
                parse(try_from_str=parse_char))]
    separator: Option<char>, // need this to be Option so that we can manually put the default or else it will be displayed as a newline

    #[structopt(value_name="LINE",
                help="Lines to print; if none are provided lines are read from standard input")]
    lines: Vec<String>,
}
