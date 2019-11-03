use crate::each_locale::Generator;
use crate::fake_options::FakeOption;
use crate::locale::Locale;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use crate::category::Category;
use crate::helper::split;

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

    pub fn gen(&mut self, option: &FakeOption) -> String {
        self.generator.gen(&mut self.rng, option)
    }

    pub fn gen_record(&mut self, options: &[FakeOption]) -> Vec<String> {
        let mut record: Vec<String> = Vec::new();

        for option in options {
            if option.category() == Category::Name {
                use FakeOption::*;
                match option {
                    // フリガナを持つ名前は分離する必要があるので別途処理
                    FirstName(true)|LastName(true)|FullName(true) => {
                        let (name, furigana) = split(&self.gen(option));
                        record.push(name);
                        record.push(furigana);
                    },
                    _ => record.push(self.gen(option)),
                }
            } else {
                record.push(self.gen(option));
            }
        }

        return record;
    }
}
