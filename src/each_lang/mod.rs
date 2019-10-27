use crate::fake_options::Option;
use std::ops::Range;
use rand::Rng;
use crate::locale::Locale;
use crate::each_lang::japan::JapanData;

pub mod japan;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Generator {
    locale: Locale
}

impl Generator {
    pub fn new(locale: Locale) -> Self {
        Generator{
            locale
        }
    }
    pub fn gen<R: Rng>(&mut self, rng: &mut R, option: Option) -> String{
        match self.locale {
            Locale::Japan => JapanData::gen(rng, option),
        }
    }
    pub fn gen_range<R: Rng, Idx>(&mut self, rng: &mut R, option: Option, range: Range<Idx>) -> String{
        match self.locale {
            Locale::Japan => JapanData::gen_range(rng, option, range),
        }
    }
}

trait Rand{
    fn gen<R: Rng>(rnd: &mut R, option: Option) -> String;
    fn gen_range<R: Rng, Idx>(rnd: &mut R, option: Option, range: Range<Idx>) -> String;
}

trait Data{
    // TODO
}