use crate::category::Category;
use crate::each_locale::Generator;
use crate::fake_options::FakeOption;
use crate::helper::split;
use crate::locale::Locale;
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

        for option in options {
            if option.category() == Category::Name {
                use FakeOption::*;
                match option {
                    // フリガナを持つ名前は分離する必要があるので別途処理
                    FirstName(true) | LastName(true) | FullName(true) => {
                        let (name, furigana) = split(&self.gen(option));
                        record.push(name);
                        record.push(furigana);
                    }
                    _ => record.push(self.gen(option)),
                }
            } else {
                record.push(self.gen(option));
            }
        }

        return record;
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
