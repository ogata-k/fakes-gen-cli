mod cli;

use cli::file_convert::convert_from_record;

use fakes_gen::date_time_format::DEFAULT_DATE_FORMAT;
use fakes_gen::fake_options::FakeOption;
use fakes_gen::faker::Faker;
use rand::prelude::ThreadRng;
use crate::cli::file_type::FileType;
use crate::cli::file_convert::convert_from_data_set;

fn main() {
    let mut faker: Faker<ThreadRng> = Faker::default();
    let header: Vec<String> = vec!["name".to_string(), "furigana".to_string(), "date time".to_string()];
    let mut data_set: Vec<Vec<String>> = Vec::new();

    for i in 1..=5 {
            data_set.push(
                faker.gen_record(&vec![FakeOption::FullName(true), FakeOption::DateTime(DEFAULT_DATE_FORMAT.to_string())])
            );

    }

    println!(
        "{}",
        convert_from_data_set(
            FileType::TSV,
            &header,
            &data_set
        )
    );
}

// TODO Next CLI argument(locale, generate-file-type, option-list, count), Scan for converting argument to FakeOption