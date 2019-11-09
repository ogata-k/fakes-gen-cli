pub mod scanner;

use crate::cli::scanner::Scanner;
use clap::*;

pub struct FakerApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> FakerApp<'a, 'b> {
    pub fn new() -> FakerApp<'a, 'b> {
        FakerApp {
            app: app_from_crate!()
                .help_short("H")
                .arg(
                    Arg::with_name("usable")
                        .short("u")
                        .long("usable")
                        .help("show list of all usable options for faker")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("formatter")
                        .short("f")
                        .long("formatter")
                        .help("formatter or output")
                        .default_value("csv")
                        .case_insensitive(true)
                        .possible_values(&vec!["csv", "tsv", "json"])
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("header")
                        .short("h")
                        .long("header")
                        .help("flag of use generate header")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("size")
                        .short("s")
                        .long("size")
                        .help(
                            "data size. If 1, generate as record. If over 1, generate as data_set.",
                        )
                        .default_value("1"),
                )
                .arg(
                    Arg::with_name("locale")
                        .short("l")
                        .long("locale")
                        .help("3-char's country code.")
                        .default_value("jpn")
                        .case_insensitive(true)
                        .possible_values(&vec!["jpn"])
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("option")
                        .help("options with name of column for faker")
                        .multiple(true)
                        .takes_value(true),
                ),
        }
    }

    /// TODO 実際の挙動
    pub fn run(self) {
        let m: ArgMatches = self.app.get_matches();
        if m.is_present("usable") {
            return Self::print_usable_options();
        }
        // TODO scan args to get header and FakeOptions
        // TODO make new faker
        // TODO run the faker
        println!("matches: {:?}", m); // TODO remove
        if m.is_present("option") {
            for option in m.values_of("option").unwrap() {
                let scan_res = Scanner::new(option).scan();
                if scan_res.is_err() {
                    print!("ERR: {}", scan_res.err().unwrap());
                } else {
                    let (h, op) = scan_res.unwrap();
                    println!("OK: ({}, {})", h, op);
                }
            }
        }
    }

    fn print_usable_options() {
        // TODO show usable options
        println!("all options"); // TODO remove
    }
}
