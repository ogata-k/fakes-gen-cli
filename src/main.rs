use fakes_gen::faker::Faker;
use fakes_gen::fake_options::FakeOption;
use rand::prelude::ThreadRng;

fn main() {
    let mut faker: Faker<ThreadRng> = Faker::default();
    for i in 1..=10 {
        print!("{}: ", i);
        println!("{}", faker.gen(FakeOption::FileName));
    }
}
