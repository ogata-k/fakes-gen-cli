mod cli;

use cli::file_convert::convert_from_record;

use crate::cli::file_convert::convert_from_data_set;
use crate::cli::file_type::FileType;
use fakes_gen::date_time_format::DEFAULT_DATE_FORMAT;
use fakes_gen::fake_options::FakeOption;
use fakes_gen::faker::Faker;
use rand::prelude::ThreadRng;

fn main() {
    let mut faker: Faker<ThreadRng> = Faker::default();
    let header: Vec<String> = vec![
        "name".to_string(),
        "furigana".to_string(),
        "date time".to_string(),
    ];
    let data_set: Vec<Vec<String>> = faker.gen_data_set(
        10,
        &vec![
            FakeOption::FullName(true),
            FakeOption::DateTime(DEFAULT_DATE_FORMAT.to_string()),
        ],
    );
    println!(
        "{}",
        convert_from_data_set(FileType::CSV, &header, &data_set)
    );
}

// TODO Next CLI argument(locale, generate-file-type, option-list, count), Scan for converting argument to FakeOption
