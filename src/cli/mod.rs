mod helper;
pub mod scanner;

use crate::cli::scanner::Scanner;
use clap::*;
use failure::_core::str::FromStr;
use fakes_gen::converter::file_convert::{
    to_data_set, to_full_form, to_record, to_record_with_header,
};
use fakes_gen::converter::file_type::FileType;
use fakes_gen::faker::category::Category;
use fakes_gen::faker::fake_options::FakeOption;
use fakes_gen::faker::locale::Locale;
use fakes_gen::faker::Faker;
use rand::thread_rng;
use std::io;

pub struct FakerApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> FakerApp<'a, 'b> {
    pub fn new() -> FakerApp<'a, 'b> {
        FakerApp {
            app: app_from_crate!()
                .help_message("Print this message")
                .arg(
                    Arg::with_name("usable")
                        .short("u")
                        .long("usable")
                        .help("show list of all usable options for faker")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("bnf")
                        .short("b")
                        .long("bnf")
                        .help("show Backus-Naur Form and detail format for options")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("converter")
                        .short("c")
                        .long("converter")
                        .help("converter for output")
                        .default_value("csv")
                        .case_insensitive(true)
                        .possible_values(&vec!["csv", "tsv", "json"])
                        .takes_value(true),
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
                    Arg::with_name("fullform")
                        .short("f")
                        .long("fullform")
                        .help("flag for generating as fullform such as body with header")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("option")
                        .help("options with name of column for faker")
                        .multiple(true)
                        .takes_value(true),
                ),
        }
    }

    pub fn run(self) -> io::Result<()> {
        let mut app = self.app.clone();
        let m: ArgMatches = self.app.get_matches();
        if m.is_present("usable") {
            Self::print_usable_options();
            return Ok(());
        }
        if m.is_present("bnf") {
            Self::print_bnf();
            return Ok(());
        }
        if !m.is_present("option") {
            app.print_help().unwrap();
            return Ok(());
        }

        let locale: Locale = match m.value_of("locale").unwrap() {
            "jpn" => Locale::Japan,
            _ => unreachable!(),
        };

        let converter: FileType = match m.value_of("converter").unwrap() {
            "csv" => FileType::CSV,
            "tsv" => FileType::TSV,
            "json" => FileType::JSON,
            _ => unreachable!(),
        };

        let size = usize::from_str(m.value_of("size").unwrap());
        if size.is_err() {
            let size = size.err().unwrap();
            eprintln!("Parse Err: {}", size);
            return Ok(());
        }
        let size: usize = size.unwrap();
        if size == 0 {
            eprintln!("Size Error: size is 0");
            return Ok(());
        }

        let mut header_options: Vec<(String, FakeOption)> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        if m.is_present("option") {
            for option in m.values_of("option").unwrap() {
                let scan_res = Scanner::new(option).scan();
                if scan_res.is_err() {
                    errors.push(format!("Format Err: {}", scan_res.err().unwrap()));
                } else {
                    header_options.push(scan_res.unwrap());
                }
            }
            if !errors.is_empty() {
                for error in errors {
                    eprintln!("{}", error);
                }
                return Ok(());
            }
        }

        let mut faker = Faker::new(thread_rng(), locale);
        let mut writer = io::stdout();
        if size == 1 {
            if m.is_present("fullform") {
                to_record_with_header(&mut writer, &mut faker, converter, &header_options)
            } else {
                to_record(&mut writer, &mut faker, converter, &header_options)
            }
        } else {
            if m.is_present("fullform") {
                to_full_form(&mut writer, &mut faker, converter, &header_options, size)
            } else {
                to_data_set(&mut writer, &mut faker, converter, &header_options, size)
            }
        }
    }

    fn print_usable_options() {
        for category in Category::all_list() {
            println!("Category:\n {}", category.to_string());
            println!("Options:");
            for op in Scanner::readable_options(category) {
                println!("・{}", op);
            }
            println!();
        }
    }

    fn print_bnf() {
        println!("All Backus-Naur Form for Option(<option>)");
        for bnf in Scanner::all_format_bnf() {
            println!("{}", bnf);
        }
        println!("If you want to know <format_string>, please reference to https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers");
    }
}
