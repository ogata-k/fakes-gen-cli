use rand::{thread_rng, Rng};
use crate::locale::Locale;
use crate::fake_options::FakeOption;
use rand::prelude::ThreadRng;
use crate::each_lang::Generator;

#[derive(Debug)]
pub struct Faker<R: Rng>{
    rnd: R,
    generator: Generator
}

impl Default for Faker<ThreadRng> {
    fn default() -> Self {
        Faker{
            rnd: thread_rng(),
            generator: Generator::new(Locale::default())
        }
    }
}

impl<R: Rng> Faker<R> {
    pub fn new(rng: R, locale: Locale) -> Faker<R> {
        Faker{
            rnd: rng,
            generator: Generator::new(locale)
        }
    }

    pub fn gen(&mut self, option: FakeOption) -> String {
        self.generator.gen(&mut self.rnd, option)
    }
}