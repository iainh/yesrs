#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::io::{self, Write};

fn parse_args() -> ArgMatches<'static> {
    App::new("yes")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Repeatedly output a line with all specified STRING(s), or 'y'.")
        .arg(Arg::with_name("STRING")
            .index(1))
        .get_matches()
}

fn main() {
    let args = parse_args();
    let mut value = String::from(args.value_of("STRING").unwrap_or("y"));
    value.push_str("\n");

    let repeated_string = value.repeat(value.len() * 1000);
    let output = repeated_string.as_bytes();

    let mut writer = io::stdout();
    loop {
        writer.write_all(output).unwrap();
    }
}
