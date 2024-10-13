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
        escape_iter(separator, &args.lines)?;
        if args.both {
            escape_stdin(separator)?;
        }
    }

    Ok(())
}


fn escape_iter<T>(separator: char, it: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    let mut buf: String;
    for line in it {
        let line = line.as_ref();
        print!(
            "{}{}",
            match shlex::try_quote(line) {
                Ok(escaped) => escaped,
                Err(_) => {
                    buf = line.replace('\0', "");
                    shlex::try_quote(&buf).unwrap()
                },
            },
            separator,
        );
    }
    Ok(())
}

fn escape_stdin(separator: char) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut buf: String;
    for line in reader.split(separator as u8) {
        let line = line?;
        let line = std::str::from_utf8(&line)?;
        print!(
            "{}{}",
            match shlex::try_quote(line) {
                Ok(escaped) => escaped,
                Err(_) => {
                    buf = line.replace('\0', "");
                    shlex::try_quote(&buf).unwrap()
                },
            },
            separator,
        );
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
    about="Simply escapes input\nControl characters are removed",
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
