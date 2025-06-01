use std::{
    fs::File,
    io::{BufReader, prelude::*, stdin},
};

use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(Arg::new("input").help("File to search").required(false))
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    if let Some(i) = args.get_one::<String>("input") {
        let f = File::open(i).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    };
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
