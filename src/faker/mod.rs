pub mod each_locale;

pub mod category;
pub mod fake_options;
pub mod locale;

use crate::helper::{split, string_formatted, trim_double_quoted};

use crate::faker::each_locale::Generator;
use crate::faker::fake_options::FakeOption;
use crate::faker::locale::Locale;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Faker<R: Rng> {
    rng: R,
    locale: Locale,
    generator: Generator,
}

impl Default for Faker<ThreadRng> {
    fn default() -> Self {
        Faker {
            rng: thread_rng(),
            locale: Locale::default(),
            generator: Generator::new(Locale::default()),
        }
    }
}

impl<R: Rng> Faker<R> {
    pub fn new(rng: R, locale: Locale) -> Faker<R> {
        Faker {
            rng,
            locale,
            generator: Generator::new(locale),
        }
    }

    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// one data
    pub fn gen(&mut self, option: &FakeOption) -> String {
        self.generator.gen(&mut self.rng, option)
    }

    /// one record
    pub fn gen_record(&mut self, options: &[FakeOption]) -> Vec<String> {
        let mut record: Vec<String> = Vec::new();
        let person_name: Option<PersonName> = if options.iter().any(|op| op.is_person_name()) {
            Some(PersonName::new(&mut self.rng, &mut self.generator))
        } else {
            None
        };

        if person_name.is_none() {
            for option in options {
                record.push(self.gen(option));
            }
            return record;
        } else {
            let person_name: PersonName = person_name.unwrap();
            for option in options {
                use FakeOption::*;
                let dummy: String = match option {
                    FirstName(false) => string_formatted(&person_name.first_name),
                    FirstName(true) => string_formatted(
                        &[
                            person_name.first_name.to_string(),
                            person_name.first_name_furigana.to_string(),
                        ]
                        .join(":"),
                    ),
                    FirstNameFurigana => string_formatted(&person_name.first_name_furigana),
                    LastName(false) => string_formatted(&person_name.last_name),
                    LastName(true) => string_formatted(
                        &[
                            person_name.last_name.to_string(),
                            person_name.last_name_furigana.to_string(),
                        ]
                        .join(":"),
                    ),
                    LastNameFurigana => string_formatted(&person_name.last_name_furigana),
                    FullName(false) => string_formatted(&person_name.full_name),
                    FullName(true) => string_formatted(
                        &[
                            person_name.full_name.to_string(),
                            person_name.full_name_furigana.to_string(),
                        ]
                        .join(":"),
                    ),
                    FullNameFurigana => string_formatted(&person_name.full_name_furigana),
                    _ => self.gen(option),
                };
                record.push(dummy);
            }

            return record;
        }
    }

    /// many record
    pub fn gen_data_set(&mut self, count: usize, options: &[FakeOption]) -> Vec<Vec<String>> {
        let mut data_set: Vec<Vec<String>> = Vec::new();
        for _ in 1..=count {
            data_set.push(self.gen_record(options));
        }
        return data_set;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PersonName {
    first_name: String,
    first_name_furigana: String,
    last_name: String,
    last_name_furigana: String,
    full_name: String,
    full_name_furigana: String,
}

impl PersonName {
    fn new<R: Rng>(rng: &mut R, generator: &mut Generator) -> Self {
        let last_name: (String, String) = split(&generator.gen(rng, &FakeOption::LastName(true)));
        let first_name: (String, String) = split(&generator.gen(rng, &FakeOption::LastName(true)));
        let full_name: (String, String) = (
            generator.build_name(
                &trim_double_quoted(&last_name.0),
                &trim_double_quoted(&first_name.0),
            ),
            generator.build_name(
                &trim_double_quoted(&last_name.1),
                &trim_double_quoted(&first_name.1),
            ),
        );
        return PersonName {
            first_name: trim_double_quoted(&first_name.0),
            first_name_furigana: trim_double_quoted(&first_name.1),
            last_name: trim_double_quoted(&last_name.0),
            last_name_furigana: trim_double_quoted(&last_name.1),
            full_name: full_name.0,
            full_name_furigana: full_name.1,
        };
    }
}
