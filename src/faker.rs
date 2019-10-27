use rand::Rng;
use crate::locale::Locale;
use crate::fake_options::Option;
use rand::prelude::ThreadRng;
use crate::each_lang::Generator;
use std::ops::Range;

#[derive(Debug)]
pub struct Faker<R: Rng>{
    rnd: R,
    pub generator: Generator
}

impl Default for Faker<ThreadRng> {
    fn default() -> Self {
        Faker{
            rnd: rand::thread_rng(),
            generator: Generator::new(Locale::default())
        }
    }
}

impl <R: Rng> Faker<R> {
    pub fn new(rnd: R, locale: Locale) -> Faker<R> {
        Faker{
            rnd,
            generator: Generator::new(locale)
        }
    }

    pub fn gen(&mut self, option: Option) -> String {
        self.generator.gen(&mut self.rnd, option)
    }

    pub fn gen_range<Idx>(&mut self, option: Option, range: Range<Idx>) -> String {
        self.generator.gen_range(&mut self.rnd, option, range)
    }
}