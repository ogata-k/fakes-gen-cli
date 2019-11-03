use crate::each_locale::Generator;
use crate::fake_options::FakeOption;
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

    pub fn gen(&mut self, option: &FakeOption) -> String {
        self.generator.gen(&mut self.rng, option)
    }
}
