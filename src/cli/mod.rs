pub mod scanner;

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

    pub fn run(self) {
        let m: ArgMatches = self.app.get_matches();
        if m.is_present("usable") {
            return Self::print_usable_options();
        }
        // TODO scan args to get header and FakeOptions
        // TODO make new faker
        // TODO run the faker
        println!("matches: {:?}", m); // TODO remove
    }

    fn print_usable_options() {
        // TODO show usable options
        println!("all options"); // TODO remove
    }
}
