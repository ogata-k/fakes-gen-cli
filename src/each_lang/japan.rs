use crate::each_lang::{Rand, Data};
use crate::fake_options::Option;
use rand::Rng;
use std::ops::Range;

pub struct JapanData {}

impl Rand for JapanData {
    fn gen<R: Rng>(rng: &mut R, option: Option) -> String {
        unimplemented!()
    }

    fn gen_range<R: Rng, Idx>(rng: &mut R, option: Option, range: Range<Idx>) -> String {
        unimplemented!()
    }
}

impl Data for JapanData{
    // TODO
}

