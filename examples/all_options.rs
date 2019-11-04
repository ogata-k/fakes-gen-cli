use fakes_gen::date_time_format::{
    DEFAULT_DATE_FORMAT, DEFAULT_DATE_TIME_FORMAT, DEFAULT_TIME_FORMAT,
};
use fakes_gen::faker::fake_options::FakeOption;
use fakes_gen::faker::locale::Locale;
use fakes_gen::faker::Faker;
use rand::rngs::ThreadRng;
use rand::thread_rng;

fn main() {
    use FakeOption::*;
    let all_options: Vec<FakeOption> = vec![
        // Fixed
        FixedString("Dummy String".to_string()),
        FixedNotString("Dummy not String".to_string()),
        // Select
        SelectString(vec![
            "str1".to_string(),
            "str2".to_string(),
            "str3".to_string(),
        ]),
        SelectNotString(vec![
            "not-str1".to_string(),
            "not-str2".to_string(),
            "not-str3".to_string(),
        ]),
        // Lorem
        Word,
        Words(3, 10),
        Sentence,
        Sentences(3, 10),
        Paragraph,
        Paragraphs(3, 10),
        // Name(use furigana)
        // generate "name":"furigana"
        FirstName(true),
        FirstName(false),
        LastName(true),
        LastName(false),
        FullName(true),
        FullName(false),
        // Primitive
        Integer,
        IntegerRange(-10, 10),
        Float,
        FloatRange(-10, 10),
        Ascii(8, 15),
        Boolean,
        // Internet
        Email,
        UserName,
        Password(8, 15),
        CreditCard,
        URL,
        IPv4,
        IPv6,
        RGB,
        RGBA,
        UserAgent,
        StatusCode,
        // Company
        CompanySuffix,
        CompanyName,
        Industry,
        // Address
        Building,
        StreetName,
        CityName,
        StateName,
        CountryCode,
        CountryName,
        TimeZone,
        Address,
        // use hyphen?
        ZipCode(true),
        ZipCode(false),
        // use hyphen?
        DomesticPhoneNumber(true),
        DomesticPhoneNumber(false),
        Latitude,
        Longitude,
        // Date Time
        // format-str: https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers
        // String is format. default is "%Y-%m-%d %H:%I:%M"'s sub-format.
        // But, When Time(Date, DateTime), use only Time(Date, Time/Date)-formatter.
        // ex. "219-11-02 21:09:31"
        Time(DEFAULT_TIME_FORMAT.to_string()),
        // now - 100year ~ now
        Date(DEFAULT_DATE_FORMAT.to_string()),
        // now - 100year ~ now
        DateTime(DEFAULT_DATE_TIME_FORMAT.to_string()),
        // FileSystem
        FileName,
        Extension,
    ];

    // init
    let mut faker: Faker<ThreadRng> = Faker::new(thread_rng(), Locale::Japan);
    println!("locale: {}", faker.locale());

    // generate
    for option in all_options {
        let res: String = faker.gen(&option);
        print_with_category(&option, &res);
    }
}

fn print_with_category(option: &FakeOption, res: &str) {
    println!("{}: {}", option, res);
}
