use rand::Rng;
use std::fmt::Display;
use rand::seq::SliceRandom;
use rand::distributions::uniform::SampleUniform;
use num_traits::Num;

pub fn string_formatted<T: Display + ?Sized>(text: &T) -> String {
    format!("\"{}\"", text)
}

pub fn not_string_formatted<T: Display + ?Sized>(text: &T) -> String {
    text.to_string()
}

pub fn split(text: &str) -> (String, String) {
    let list: Vec<&str> = text.split(':').map(|s: &str| s.trim()).collect();
    return if list.len() == 1 {
        (list[0].to_string(), list[0].to_string())
    } else {
        (list[0].to_string(), list[1].to_string())
    };
}

pub fn select<'a, R: Rng, I: ?Sized>(rng: &'a mut R, data: &'a [&I]) -> &'a I {
    return data.choose(rng).unwrap();
}

/// minimum <= n <= maximum
pub fn gen_range<R: Rng, T: SampleUniform + Sized + Num>(rng: &mut R, minimum: T, maximum: T) -> T {
    rng.gen_range::<T, T, T>(minimum, maximum + T::one())
}

pub fn gen_fraction_part<R: Rng>(rng: &mut R) -> f64 {
    gen_range(rng, 0 as f64, 0 as f64)
}

pub fn select_many<'a, R: Rng, I: ?Sized>(rng: &'a mut R, data: &'a [&I], minimum: usize, maximum: usize) -> Vec<&'a I> {
    let size: usize = gen_range(rng, minimum, maximum);
    return data.choose_multiple(rng, size).map(|i| *i).collect();
}

const ASCII: &'static str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const ALPHA_NUM: &'static str = "0123456789ABCDEFGHIJKLMNOPWRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const PASSWORD_CHAR: &'static str = "0123456789ABCDEFGHIJKLMNOPWRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()+-={}[]:;<>,./?_~|";

fn gen_chars<R: Rng>(base: &str, rng: &mut R, minimum: usize, maximum: usize) -> String {
    let size: usize = gen_range(rng, minimum, maximum);
    return String::from_utf8(
        base.as_bytes()
            .choose_multiple(rng, size)
            .cloned()
            .collect()
    ).unwrap()
}

pub fn gen_ascii_chars<R: Rng>(rng: &mut R, from: usize, to: usize) -> String {
    gen_chars(ASCII, rng, from, to)
}

pub fn gen_alpha_num_chars<R: Rng>(gnd: &mut R, from: usize, to: usize) -> String {
    gen_chars(ALPHA_NUM, gnd, from, to)
}

pub fn gen_password_chars<R: Rng>(rng: &mut R, from: usize, to: usize) -> String {
    gen_chars(PASSWORD_CHAR, rng, from, to)
}

