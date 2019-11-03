use fakes_gen::faker::Faker;
use fakes_gen::fake_options::FakeOption;
use rand::prelude::ThreadRng;
use fakes_gen::date_time_format::DEFAULT_DATE_TIME_FORMAT;

fn main() {
    let mut faker: Faker<ThreadRng> = Faker::default();
    for i in 1..=10 {
        print!("{}: ", i);
        println!("{}", faker.gen(FakeOption::DateTime(DEFAULT_DATE_TIME_FORMAT.to_string())));
    }
}
