use fakes_gen::converter::file_convert::{
    to_data_set, to_full_form, to_record, to_record_with_header,
};
use fakes_gen::converter::file_type::FileType;
use fakes_gen::date_time_format::DEFAULT_DATE_TIME_FORMAT;
use fakes_gen::faker::fake_options::FakeOption;
use fakes_gen::faker::Faker;
use rand::rngs::ThreadRng;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut faker: Faker<ThreadRng> = Faker::default();
    let header_options: Vec<(String, FakeOption)> = vec![
        (
            "first_name_with_furigana".to_string(),
            FakeOption::FirstName(true),
        ),
        ("furigana".to_string(), FakeOption::FirstNameFurigana),
        (
            "date time".to_string(),
            FakeOption::DateTime(DEFAULT_DATE_TIME_FORMAT.to_string()),
        ),
    ];

    let mut writer = std::io::stdout();

    write!(writer, "record:\n")?;
    to_record(&mut writer, &mut faker, FileType::TSV, &header_options)?;

    write!(writer, "\n\n")?;
    write!(writer, "record_with_header:\n")?;
    to_record_with_header(&mut writer, &mut faker, FileType::TSV, &header_options)?;

    write!(writer, "\n\n")?;
    write!(writer, "data_set:\n")?;
    to_data_set(&mut writer, &mut faker, FileType::TSV, &header_options, 10)?;

    write!(writer, "\n\n")?;
    write!(writer, "full_form:\n")?;
    to_full_form(&mut writer, &mut faker, FileType::TSV, &header_options, 10)?;
    writer.flush()
}
