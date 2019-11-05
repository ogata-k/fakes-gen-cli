use fakes_gen::converter::file_convert::{to_data_set, to_full_form, to_record};
use fakes_gen::converter::file_type::FileType;
use fakes_gen::date_time_format::DEFAULT_DATE_TIME_FORMAT;
use fakes_gen::faker::fake_options::FakeOption;
use fakes_gen::faker::Faker;
use rand::rngs::ThreadRng;

fn main() {
    let mut faker: Faker<ThreadRng> = Faker::default();
    let header: Vec<String> = vec![
        "full name".to_string(),
        "last name".to_string(),
        "first name".to_string(),
        "full name furigana".to_string(),
        "last name furigana".to_string(),
        "first name furigana".to_string(),
        "full name with furigana".to_string(),
        "date time".to_string(),
    ];
    let options: Vec<FakeOption> = vec![
        FakeOption::FullName(false),
        FakeOption::LastName(false),
        FakeOption::FirstName(false),
        FakeOption::FullNameFurigana,
        FakeOption::LastNameFurigana,
        FakeOption::FirstNameFurigana,
        FakeOption::FullName(true),
        FakeOption::DateTime(DEFAULT_DATE_TIME_FORMAT.to_string()),
    ];

    println!(
        "record:\n{}",
        to_record(&header, &faker.gen_record(&options), FileType::JSON).unwrap()
    );
    println!(
        "\ndata_set:\n{}",
        to_data_set(&header, &faker.gen_data_set(10, &options), FileType::JSON).unwrap()
    );
    println!(
        "\nfull_form:\n{}",
        to_full_form(&header, &faker.gen_data_set(10, &options), FileType::JSON).unwrap()
    );
}
