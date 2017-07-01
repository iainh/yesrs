#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};

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

    let value = args.value_of("STRING");
    match value {
        Some(s) => {
            loop {
                println!("{}", s);
            }
        }
        None => {
            loop {
                println!("y");
            }
        }
    }
}
