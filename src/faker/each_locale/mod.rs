use crate::faker::each_locale::japan::JapanData;
use crate::faker::fake_options::FakeOption;
use crate::faker::locale::Locale;
use crate::helper::{
    gen_alpha_num_chars, gen_ascii_chars, gen_fraction_part, gen_password_chars, gen_range, select,
    select_many, split,
};

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use rand::Rng;
use std::net::{Ipv4Addr, Ipv6Addr};

pub mod japan;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Generator {
    locale: Locale,
}

impl Generator {
    pub fn new(locale: Locale) -> Self {
        Generator { locale }
    }
    pub fn gen<R: Rng>(&mut self, rng: &mut R, option: &FakeOption) -> String {
        match self.locale {
            Locale::Japan => JapanData::gen(rng, option),
        }
    }
    pub fn locale(&self) -> Locale {
        self.locale
    }
    pub fn build_name(&self, last_name: &str, first_name: &str) -> String {
        match self.locale {
            Locale::Japan => JapanData::build_name(last_name, first_name),
        }
    }
}

trait Rand: Data {
    fn gen<R: Rng>(rng: &mut R, option: &FakeOption) -> String {
        match option {
            // With
            FakeOption::Join(sep, data) => {
                return data
                    .iter()
                    .map(|d| Self::gen(rng, d))
                    .collect::<Vec<String>>()
                    .join(sep);
            }

            // Fixed Value
            FakeOption::FixedString(s) => {
                return s.clone();
            }
            FakeOption::FixedNotString(s) => {
                return s.clone();
            }

            // Random select
            FakeOption::SelectString(user_values) => {
                return select(
                    rng,
                    user_values
                        .iter()
                        .map(|s| s as &str)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                )
                .to_string();
            }
            FakeOption::SelectNotString(user_values) => {
                return select(
                    rng,
                    user_values
                        .iter()
                        .map(|s| s as &str)
                        .collect::<Vec<&str>>()
                        .as_slice(),
                )
                .to_string();
            }

            // Lorem
            FakeOption::Word => {
                return select(rng, Self::WORD).to_string();
            }
            FakeOption::Words(minimum, maximum) => {
                return select_many(rng, Self::WORD, *minimum, *maximum).join(" ");
            }
            FakeOption::Sentence => {
                return select(rng, Self::SENTENCE).to_string();
            }
            FakeOption::Sentences(minimum, maximum) => {
                return select_many(rng, Self::SENTENCE, *minimum, *maximum).join(" ");
            }
            FakeOption::Paragraph => {
                return select(rng, Self::PARAGRAPH).to_string();
            }
            FakeOption::Paragraphs(minimum, maximum) => {
                return select_many(rng, Self::PARAGRAPH, *minimum, *maximum).join("\n");
            }

            // Name
            FakeOption::FirstName(use_furigana) => {
                let target: String = select(rng, Self::FIRST_NAME).to_string();
                return if *use_furigana {
                    target
                } else {
                    split(&target).0.to_string()
                };
            }
            FakeOption::FirstNameFurigana => {
                let (_, furigana): (String, String) = split(select(rng, Self::FIRST_NAME));
                furigana
            }
            FakeOption::LastName(use_furigana) => {
                let target: String = select(rng, Self::LAST_NAME).to_string();
                return if *use_furigana {
                    target
                } else {
                    split(&target).0.to_string()
                };
            }
            FakeOption::LastNameFurigana => {
                let (_, furigana): (String, String) = split(select(rng, Self::LAST_NAME));
                furigana
            }
            FakeOption::FullName(use_furigana) => {
                let first: (String, String) = split(select(rng, Self::FIRST_NAME));
                let last: (String, String) = split(select(rng, Self::LAST_NAME));
                let name: (String, String) = (
                    Self::build_name(&last.0, &first.0),
                    Self::build_name(&last.1, &first.1),
                );
                return if *use_furigana {
                    [name.0, name.1].join(":")
                } else {
                    name.0
                };
            }
            FakeOption::FullNameFurigana => {
                let first: (String, String) = split(select(rng, Self::FIRST_NAME));
                let last: (String, String) = split(select(rng, Self::LAST_NAME));
                let furigana: String = Self::build_name(&last.1, &first.1);
                furigana
            }

            // Primitive
            FakeOption::Integer => {
                return format!("{}", rng.gen::<i16>());
            }
            FakeOption::IntegerRange(minimum, maximum) => {
                return format!("{}", gen_range(rng, *minimum..=*maximum));
            }
            FakeOption::Float => {
                let i: i16 = rng.gen::<i16>();
                let d: f64 = rng.gen::<f64>();
                return format!("{:.2}", i as f64 + d);
            }
            FakeOption::FloatRange(minimum, maximum) => {
                let diff: f64 = (maximum - minimum) as f64 * gen_fraction_part(rng);
                return format!("{:.2}", *minimum as f64 + diff);
            }
            FakeOption::Ascii(minimum, maximum) => gen_ascii_chars(rng, *minimum, *maximum),
            FakeOption::Boolean => {
                return format!("{}", rng.gen::<u8>() % 2 == 0);
            }

            // Internet
            FakeOption::Email => {
                let s: String = gen_alpha_num_chars(rng, 8, 20);
                return [&s, "example.com"].join("@");
            }
            FakeOption::UserName => gen_alpha_num_chars(rng, 4, 15),
            FakeOption::Password(minimum, maximum) => gen_password_chars(rng, *minimum, *maximum),
            FakeOption::CreditCard => return select(rng, Self::CREDIT_CARD).to_string(),
            FakeOption::URL => {
                let domain: String = select(rng, Self::URL).to_string();
                let first: String = gen_alpha_num_chars(rng, 1, 10);
                let second: String = gen_alpha_num_chars(rng, 1, 10);
                return format!("http://{}/{}/{}", domain, first, second);
            }
            FakeOption::IPv4 => {
                let d: u8 = rng.gen();
                let (a, b, c): &(u8, u8, u8) = select(rng, Self::IPV4);
                return Ipv4Addr::new(*a, *b, *c, d).to_string();
            }
            FakeOption::IPv6 => {
                let c: u16 = rng.gen();
                let d: u16 = rng.gen();
                let e: u16 = rng.gen();
                let f: u16 = rng.gen();
                let g: u16 = rng.gen();
                let h: u16 = rng.gen();
                return Ipv6Addr::new(2001_u16, 0xdb8, c, d, e, f, g, h).to_string();
            }
            FakeOption::RGB => {
                let r: u8 = rng.gen();
                let g: u8 = rng.gen();
                let b: u8 = rng.gen();
                return format!("#{:>02X}{:>02X}{:>02X}", r, g, b);
            }
            FakeOption::RGBA => {
                let r: u8 = rng.gen();
                let g: u8 = rng.gen();
                let b: u8 = rng.gen();
                let a: u8 = rng.gen();
                return format!("#{:>02X}{:>02X}{:>02X}{:>02X}", r, g, b, a);
            }
            FakeOption::UserAgent => {
                return select(rng, Self::USER_AGENT).to_string();
            }
            FakeOption::StatusCode => {
                return format!("{}", select(rng, Self::HTTP_STATUS_CODE));
            }

            // Company
            FakeOption::CompanySuffix => {
                return select(rng, Self::COMPANY_SUFFIX).to_string();
            }
            FakeOption::CompanyName => {
                let name: String = select(rng, Self::COMPANY_NAME).to_string();
                let suffix: String = select(rng, Self::COMPANY_SUFFIX).to_string();
                return [name, suffix].join("");
            }
            FakeOption::Industry => {
                return select(rng, Self::INDUSTRY).to_string();
            }

            // Address
            FakeOption::Building => {
                return select(rng, Self::BUILDING).to_string();
            }
            FakeOption::StreetName => {
                return select(rng, Self::STREET_NAME).to_string();
            }
            FakeOption::CityName => {
                return select(rng, Self::CITY_NAME).to_string();
            }
            FakeOption::StateName => {
                return select(rng, Self::STATE_NAME).to_string();
            }
            FakeOption::CountryName => {
                return select(rng, Self::COUNTRY_NAME).to_string();
            }
            FakeOption::CountryCode => {
                return select(rng, Self::COUNTRY_CODE).to_string();
            }
            FakeOption::TimeZone => {
                return select(rng, Self::TIME_ZONE).to_string();
            }
            FakeOption::Address => {
                let street: String = select(rng, Self::STREET_NAME).to_string();
                let city: String = select(rng, Self::CITY_NAME).to_string();
                let state: String = select(rng, Self::STATE_NAME).to_string();
                return Self::build_address(&street, &city, &state);
            }
            FakeOption::ZipCode(hyphen) => {
                return Self::gen_zip_code(rng, *hyphen);
            }
            FakeOption::DomesticPhoneNumber(hyphen) => {
                return Self::gen_domestic_phone_number(rng, *hyphen);
            }
            FakeOption::Latitude => {
                let diff: f64 = 180 as f64 * gen_fraction_part(rng);
                return format!("{:<+010.6}", -90 as f64 + diff);
            }
            FakeOption::Longitude => {
                let diff: f64 = 360 as f64 * gen_fraction_part(rng);
                return format!("{:<+011.6}", -90 as f64 + diff);
            }

            // DateTime
            FakeOption::Time(format) => {
                let hour: u32 = gen_range(rng, 0..=23);
                let minute: u32 = gen_range(rng, 0..=59);
                let second: u32 = gen_range(rng, 0..=59);
                let time: NaiveTime = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
                return time.format(&format).to_string();
            }
            FakeOption::Date(format) => {
                let now_year: i32 = Local::now().year();
                let year: i32 = gen_range(rng, now_year - 100..=now_year);
                let month: u32 = gen_range(rng, 1..=12);
                let day: u32 = gen_range(rng, 1..=31);
                let mut date: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
                while date.is_none() {
                    date = NaiveDate::from_ymd_opt(year, month, gen_range(rng, 1..=31));
                }
                return date.unwrap().format(&format).to_string();
            }
            FakeOption::DateTime(format) => {
                let now_year: i32 = Local::now().year();
                let year: i32 = gen_range(rng, now_year - 100..=now_year);
                let month: u32 = gen_range(rng, 1..=12);
                let day: u32 = gen_range(rng, 1..=31);
                let mut date: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
                while date.is_none() {
                    date = NaiveDate::from_ymd_opt(year, month, gen_range(rng, 1..=31));
                }
                let hour: u32 = gen_range(rng, 0..=23);
                let minute: u32 = gen_range(rng, 0..=59);
                let second: u32 = gen_range(rng, 0..=59);
                let time: NaiveTime = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
                let date_time: NaiveDateTime = NaiveDateTime::new(date.unwrap(), time);
                return date_time.format(&format).to_string();
            }

            // FileSystem
            FakeOption::FileName => {
                let filename: String = gen_alpha_num_chars(rng, 3, 15);
                let ext: String = select(rng, Self::EXTENSION).to_string();
                return [filename, ext].join(".");
            }
            FakeOption::Extension => {
                return select(rng, Self::EXTENSION).to_string();
            }
        }
    }
}

trait Data {
    // Lorem
    const WORD: &'static [&'static str];
    const SENTENCE: &'static [&'static str];
    const PARAGRAPH: &'static [&'static str];

    // Name
    // name data is made from target:furigana.
    // If you set the furigana, use target as furigana.
    // Name data is combine FIRST_NAME and LAST_NAME
    fn build_name(last_name: &str, first_name: &str) -> String;
    const FIRST_NAME: &'static [&'static str];
    const LAST_NAME: &'static [&'static str];

    // Internet
    const CREDIT_CARD: &'static [&'static str] = &[
        "4111111111111111",
        "4242424242424242",
        "4012888888881881",
        "4222222222222",
        "5555555555554444",
        "5105105105105100",
        "5431111111111111",
        "5111111111111118",
        "3530111333300000",
        "3566002020360505",
        "378282246310005",
        "371449635398431",
        "341111111111111",
        "30569309025904",
        "38520000023237",
        "6111111111111116",
        "6011111111111117",
        "6011000990139424",
        "6011601160116611",
        "1354 1234 5678 911",
        "5033 9619 8909 17",
        "5868 2416 0825 5333 38",
        "6759 0000 0000 0000 00",
        "6759 0000 0000 0000 000",
        "6304985028090561515",
        "378734493671000",         // 法人用クレジットカード
        "4000000000000010",        // 住所と郵便番号のチェックに失敗
        "4000000000000028",        // 住所のチェックに失敗
        "4000000000000036",        // 郵便番号のチェックに失敗
        "4000000000000101",        // セキュリティナンバーのチェックに失敗
        "4000000000000341",        // 受付は通るが請求に失敗
        "4000000000000002",        // 認証を常に拒否し、拒否コードを返す
        "4000000000000127",        // 認証を拒否し、セキュリティナンバー違いのエラーを返す
        "4000000000000069",        // 認証を拒否し、期限切れエラーを返す
        "4000000000000119",        // 認証を拒否し、処理中のエラーを返す
        "6759 4111 0000 0008",     // Issue No.が必要ない場合
        "6759 5600 4500 5727 054", // 1桁のIssue No.が必要な場合
        "5641 8211 1116 6669",     // 2桁のIssue No.が必要な場合
        "6334 5898 9800 0001",     // Issue No.が必要ない場合
        "6767 8200 9988 0077 06",  // 1桁のIssue No.が必要な場合
        "6334 9711 1111 1114",     // 2桁のIssue No.が必要な場合
    ];
    const URL: &'static [&'static str] = &["example.com", "example.net", "example.org"];
    const IPV4: &'static [&'static (u8, u8, u8)] = &[
        // [data].0
        &(192, 0, 2),
        &(198, 51, 100),
        &(203, 0, 113),
    ];
    const USER_AGENT: &'static [&'static str] = &[
        "Mozilla/5.0 (iPhone; U; CPU like Mac OS X; en) AppleWebKit/420+ (KHTML, like Gecko) Version/3.0 Mobile/1C28 Safari/419.3", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 2_0 like Mac OS X; ja-jp) AppleWebKit/525.18.1 (KHTML, like Gecko) Version/3.1.1 Mobile/5A347 Safari/52", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 2_0 like Mac OS X; ja-jp) AppleWebKit/525.18.1 (KHTML, like Gecko) Version/3.1.1 Mobile/5A345 Safari/525.20", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 2_0_1 like Mac OS X; ja-jp) AppleWebKit/525.18.1 (KHTML, like Gecko) Version/3.1.1 Mobile/5B108 Safari/525.20", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 2_1 like Mac OS X; ja-jp) AppleWebKit/525.18.1 (KHTML, like Gecko) Version/3.1.1 Mobile/5F136 Safari/525.20", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 3_0 like Mac OS X; en-us) AppleWebKit/528.18 (KHTML, like Gecko) Version/4.0 Mobile/7A341 Safari/528.16", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 3_1_3 like Mac OS X; ja-jp) AppleWebKit/528.18 (KHTML, like Gecko) Version/4.0 Mobile/7E18 Safari/528.16", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_0_1 like Mac OS X; ja-jp) AppleWebKit/532.9 (KHTML, like Gecko) Version/4.0.5 Mobile/8A306 Safari/6531.22.7", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_0_2 like Mac OS X; ja-jp) AppleWebKit/532.9 (KHTML, like Gecko) Version/4.0.5 Mobile/8A400 Safari/6531.22.7", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_1 like Mac OS X; ja-jp) AppleWebKit/532.9 (KHTML, like Gecko) Version/4.0.5 Mobile/8B117 Safari/6531.22.7", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_2_1 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8F190 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_1 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8G4 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_2 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8H7 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_3 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_4 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8K2 Safari/6533.18.5", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_5 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Mobile/8L1", "Mozilla/5.0 (iPhone; U; CPU iPhone OS 5_0 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Mobile/9A334 Safari/7534.48.3", "Mozilla/5.0 (iPhone; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Mobile/9A405 Safari/7534.48.3", "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3", "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A403 Safari/8536.25", "Mozilla/5.0 (iPod; U; CPU like Mac OS X; en) AppleWebKit/420.1 (KHTML, like Gecko) Version/3.0 Mobile/3A100a Safari/419.3", "Mozilla/5.0 (iPod; U; CPU iPhone OS 2_1 like Mac OS X; ja-jp) AppleWebKit/525.18.1 (KHTML, like Gecko) Version/3.1.1 Mobile/5F137 Safari/525.20", "Mozilla/5.0 (iPod; U; CPU iPhone OS 4_1 like Mac OS X; ja-jp) AppleWebKit/532.9 (KHTML, like Gecko) Version/4.0.5 Mobile/8B118 Safari/6531.22.7", "Mozilla/5.0 (iPod; U; CPU iPhone OS 4_2_1 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148 Safari/6533.18.5", "Mozilla/5.0 (iPod; U; CPU iPhone OS 4_3_5 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8L1 Safari/6533.18.5", "Mozilla/5.0 (iPod; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3", "Mozilla/5.0 (iPad; U; CPU OS 3_2 like Mac OS X; en-us) AppleWebKit/531.21.10 (KHTML, like Gecko) Version/4.0.4 Mobile/7B367 Safari/531.21.10", "Mozilla/5.0 (iPad; U; CPU OS 3_2_1 like Mac OS X; en-us) AppleWebKit/531.21.10 (KHTML, like Gecko) Mobile/7B405", "Mozilla/5.0 (iPad; U; CPU OS 4_2 like Mac OS X; zh-cn) AppleWebKit/533.17.9 (KHTML, like Gecko) Mobile/8C134", "Mozilla/5.0 (iPad; U; CPU OS 4_3 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8F190 Safari/6533.18.5", "Mozilla/5.0 (iPad; U; CPU OS 4_3_1 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8G4 Safari/6533.18.5", "Mozilla/5.0 (iPad; U; CPU OS 4_3_2 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8H7 Safari/6533.18.5", "Mozilla/5.0 (iPad; U; CPU OS 4_3_3 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5", "Mozilla/5.0 (iPad; U; CPU OS 4_3_4 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8K2 Safari/6533.18.5", "Mozilla/5.0 (iPad; U; CPU OS 4_3_5 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8L1 Safari/6533.18.5", "Mozilla/5.0 (iPad; CPU OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3", "Mozilla/5.0 (iPad; CPU OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3", "Mozilla/5.0 (iPad; CPU OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A403 Safari/8536.25", "Mozilla/5.0 (Linux; U; Android 1.5; ja-jp; GDDJ-09 Build/CDB56) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; IS01 Build/S3082) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; IS01 Build/SA180) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; Docomo HT-03A Build/DRD08) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; SonyEricssonSO-01B Build/R1EA029) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; generic Build/Donut) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1", "Mozilla/5.0 (Linux; U; Android 2.1-update1; ja-jp; SonyEricssonSO-01B Build/2.0.2.B.0.29) AppleWebKit/530.17 (KHTML, like Gecko) Version/4.0 Mobile Safari/530.17", "Mozilla/5.0 (Linux; U; Android 2.2.1; ja-jp; Full Android Build/MASTER) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.2.1; ja-jp; IS03 Build/S9090) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; SC-02C Build/GINGERBREAD) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; INFOBAR A01 Build/S9081) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; 001HT Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; SonyEricssonX10i Build/3.0.1.G.0.75) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.4; ja-jp; SonyEricssonIS11S Build/4.0.1.B.0.112) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.4; ja-jp; IS05 Build/S9290) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; F-05D Build/F0001) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; T-01D Build/F0001) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1", "Mozilla/5.0 (Linux; U; Android 3.0.1; ja-jp; MZ604 Build/H.6.2-20) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.1; en-us; K1 Build/HMJ37) AppleWebKit/534.13(KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; AT100 Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; Sony Tablet S Build/THMAS10000) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.2; ja-jp; SC-01D Build/MASTER) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.2; ja-jp; AT1S0 Build/HTJ85B) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.2; ja-jp; F-01D Build/F0001) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.2; ja-jp; Sony Tablet S Build/THMAS11000) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 3.2; ja-jp; A01SH Build/HTJ85B) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Safari/533.1", "Mozilla/5.0 (Linux; U; Android 3.2.1; ja-jp; Transformer TF101 Build/HTK75) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13", "Mozilla/5.0 (Linux; U; Android 4.0.1; ja-jp; Galaxy Nexus Build/ITL41D) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30", "Mozilla/5.0 (Linux; U; Android 4.0.3; ja-jp; URBANO PROGRESSO Build/010.0.3000) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30", "Mozilla/5.0 (Linux; U; Android 4.0.3; ja-jp; Sony Tablet S Build/TISU0R0110) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Safari/534.30", "Mozilla/5.0 (Linux; U; Android 4.0.4; ja-jp; SC-06D Build/IMM76D) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30", "Mozilla/5.0 (Linux; U; Android 4.1.1; ja-jp; Galaxy Nexus Build/JRO03H) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30", "Mozilla/5.0 (Linux; Android 4.1.1; Nexus 7 Build/JRO03S) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Safari/535.19", "Opera/9.80 (Android 2.3.3; Linux; Opera Mobi/ADR-1111101157; U; ja) Presto/2.9.201 Version/11.50", "Opera/9.80 (Android 3.2.1; Linux; Opera Tablet/ADR-1109081720; U; ja) Presto/2.8.149 Version/11.10", "Mozilla/5.0 (Android; Linux armv7l; rv:9.0) Gecko/20111216 Firefox/9.0 Fennec/9.0", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; KDDI-TS01; Windows Phone 6.5.3.5)", "Mozilla/5.0 (compatible; MSIE 9.0; Windows Phone OS 7.5; Trident/5.0; IEMobile/9.0; FujitsuToshibaMobileCommun; IS12T; KDDI)", "BlackBerry9000/4.6.0.294 Profile/MIDP-2.0 Configuration/CLDC-1.1 VendorID/220", "BlackBerry9300/5.0.0.1007 Profile/MIDP-2.1 Configuration/CLDC-1.1 VendorID/220", "BlackBerry9700/5.0.0.1014 Profile/MIDP-2.1 Configuration/CLDC-1.1 VendorID/220", "Mozilla/5.0 (BlackBerry; U; BlackBerry 9700; ja) AppleWebKit/534.8+ (KHTML, like Gecko) Version/6.0.0.570 Mobile Safari/534.8+", "Mozilla/5.0 (BlackBerry; U; BlackBerry 9780; ja) AppleWebKit/534.8+ (KHTML, like Gecko) Version/6.0.0.587 Mobile Safari/534.8+", "Mozilla/5.0 (BlackBerry; U; BlackBerry 9900; ja) AppleWebKit/534.11+ (KHTML, like Gecko) Version/7.1.0.74 Mobile Safari/534.11+", "Opera/9.80 (BlackBerry; Opera Mini/6.1.25376/26.958; U; en) Presto/2.8.119 Version/10.54", "Mozilla/5.0 (Symbian/3; Series60/5.2 NokiaN8-00/013.016; Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/525 (KHTML, like Gecko) Version/3.0 BrowserNG/7.2.8.10 3gpp-gba", "Mozilla/5.0 (Symbian/3; Series60/5.2 NokiaN8-00/012.002; Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/533.4 (KHTML, like Gecko) NokiaBrowser/7.3.0 Mobile Safari/533.4 3gpp-gba", "Mozilla/5.0 (Symbian/3; Series60/5.3 Nokia701/111.020.0307; Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/533.4 (KHTML, like Gecko) NokiaBrowser/7.4.1.14 Mobile Safari/533.4 3gpp-gba", "Nokia6600/1.0 (4.03.24) SymbianOS/6.1 Series60/2.0 Profile/MIDP-2.0 Configuration/CLDC-1.0", "Mozilla/5.0 (SymbianOS/9.1; U; [en]; SymbianOS/91 Series60/3.0) AppleWebkit/413 (KHTML, like Gecko) Safari/413", "Mozilla/5.0 (SymbianOS/9.1; U; [en]; Series60/3.0 NokiaE60/4.06.0) AppleWebKit/413 (KHTML, like Gecko) Safari/413", "Mozilla/5.0 (SymbianOS/9.2; U; Series60/3.1 NokiaN95/10.0.018; Profile/MIDP-2.0 Configuration/CLDC-1.1) AppleWebKit/413 (KHTML, like Gecko) Safari/413", "Mozilla/5.0 (SymbianOS/9.3; Series60/3.2 NokiaE5-00.2/071.003; Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/533.4 (KHTML, like Gecko) NokiaBrowser/7.3.1.26 Mobile Safari/533.4 3gpp-gba", "Mozilla/5.0 (SymbianOS/9.3; U; Series60/3.2 NokiaE75-1/110.48.125 Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/413 (KHTML, like Gecko) Safari/413", "Mozilla/5.0 (SymbianOS/9.4; U; Series60/5.0 Nokia5800d-1/21.0.025; Profile/MIDP-2.1 Configuration/CLDC-1.1 ) AppleWebKit/413 (KHTML, like Gecko) Safari/413", "Mozilla/5.0 (SymbianOS/9.4; Series60/5.0 NokiaN97-1/12.0.024; Profile/MIDP-2.1 Configuration/CLDC-1.1; en-us) AppleWebKit/525 (KHTML, like Gecko) BrowserNG/7.1.12344", "Mozilla/4.0 (compatible; MSIE 4.0; MSN 2.5; Windows 95)", "Mozilla/4.0 (compatible; MSIE 4.0; Windows 95; DigExt);", "Mozilla/4.0 (compatible; MSIE 4.0; Windows 95)", "Mozilla/4.0 (compatible; MSIE 4.01; MSN 2.5; MSN 2.5; Windows 98)", "Mozilla/4.0 (compatible; MSIE 4.01; MSN 2.5; Windows 95)", "Mozilla/4.0 (compatible; MSIE 4.01; MSN 2.5; Windows 98)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 95)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 95; Yahoo! JAPAN Version Windows 95/NT CD-ROM Edition 1.0.)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 98)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 98; BIGLOBE)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 98; canoncopyer)", "Mozilla/4.0 (compatible; MSIE 4.01; Windows 98; Compaq)", "Mozilla/4.0 (compatible; MSIE 4.0; Windows 98; DigExt);", "Mozilla/4.0 (compatible; MSIE 4.01; Windows NT)", "Mozilla/4.0 (compatible; MSIE 4.01; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 4.5; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.0; AOL 5.0; Windows 98; DigExt)2.26.2000 19:46:43", "Mozilla/4.0 (compatible; MSIE 5.0; MSN 2.5; MSN 2.5; Windows 98; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; MSN 2.5; Windows 95; DigExt)3.2.2000 16:2:20 asdf", "Mozilla/4.0 (compatible; MSIE 5.0; MSN 2.5; Windows 98)", "Mozilla/4.0 (compatible; MSIE 5.0; MSN 2.5; Windows 98; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95; DigExt; i-CABLE)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95; DigExt; ocnie5-1)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95; Yahoo! JAPAN Version Windows 95/NT CD-ROM Edition 1.0.)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 95; DigExt);", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; CNETHomeBuild051099)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; DigExt; ocnie5-1)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; wn_ie5_ja_v1)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; Yahoo! JAPAN Version Windows 95/NT CD-ROM Edition 1.0.; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows NT)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows NT; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; DigExt);", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 98; DigExt); Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1);", "Mozilla/4.0 (compatible; MSIE 5.00; Windows 98", "Mozilla/4.0 (compatible; MSIE 5.01; MSN 2.5; Windows 98)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows 95)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows 98)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT 5.0)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT 5.0; DigExt)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT 5.0; TUCOWS)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT)", "Mozilla/4.0 (compatible; MSIE 5.01; Windows NT 5.0; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 5.0; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.16; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.17; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.22; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.23; Mac_PowerPC)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows 95)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows 98)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT 5.0)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT 5.0; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT 5.0; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT 5.0; by TSG)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.0.3705; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.40607)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.40607; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; NOKTURNAL KICKS ASS)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; FDM;", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; Maxthon; .NET CLR 1.1.4322; .NET CLR 2.0.41115)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; .NET CLR 1", "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98)", "Mozilla/4.0 (compatible; MSIE 6.0; AOL 9.0; Windows NT 5.1; iebar; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Win32);", "Mozilla/4.0 (compatible; MSIE 6.0; Win32); .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Hotbar 4.4.6.0)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Win 9x 4.90)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 4.0)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 4.0; BVG", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; .NET CLR 1.0.3705; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; .NET CLR 1.1.4322; FDM)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; {FF0C8E09-3C86-44CB-834A-B8CEEC80A1D7}; iOpus-I-M)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; i-Nav 3.0.1.0F; .NET CLR 1.0.3705; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; MathPlayer 2.0; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; Maxthon; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; T312461)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0;)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1);", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.0.3705; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.1.4322; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 2.0.40607)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; Alexa Toolbar)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; BrowserBob)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; DFO-MPO Internet Explorer 6.0)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; ENGINE; Mozilla/4.0 (compatible; MSIE 6.0; Windows NT))", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; ESB{F40811EE-DF17-4BC9-8785-B362ABF34098}; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FDM)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FTDv3 Browser; .NET CLR 1.0.3705; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FunWebProducts; .NET CLR 1.1.4322; .NET CLR 2.0.40607)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; FunWebProducts; AtHome033)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; HCI0449; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; i-NavFourF; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; Maxthon;", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; Maxthon; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; MyIE2; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; MyIE2; Maxthon; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; Q312461; FunWebProducts; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; Woningstichting Den Helder; .NET CLR 1.0.3705)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; .NET CLR 1.1.4322; .NET CLR 2.0.41115)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; MyIE2; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; MyIE2; Maxthon; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows XP)", "MSIE 6.0; Windows 2000", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.0.3705; .NET CLR 2.0.50727; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; .NET CLR 3.0.04506.30)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0; SLCC1; .NET CLR 2.0.50727; Media Center PC 5.0; .NET CLR 3.0.04506)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0; SLCC1; .NET CLR 2.0.50727; Media Center PC 5.0; .NET CLR 3.0.04506; InfoPath.1)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; .NET CLR 1.1.4322; .NET CLR 2.0.50727)", "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; .NET CLR 2.0.50727; .NET CLR 3.0.04506.30; .NET CLR 3.0.04506.648)", "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; .NET CLR 2.0.50727; InfoPath.1", "Mozilla/4.0 (compatible; GoogleToolbar 5.0.2124.2070; Windows 6.0; MSIE 8.0.6001.18241)", "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; EasyBits GO v1.0; InfoPath.1; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)", "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0)", "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0; Sleipnir/2.9.8)", "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.0; Trident/5.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; Trident/6.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Trident/6.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; ARM; Trident/6.0)", "Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko", "Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US) AppleWebKit/525.13 (KHTML, like Gecko) Chrome/0.2.149.27 Safari/525.13", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/525.19 (KHTML, like Gecko) Chrome/1.0.154.48 Safari/525.19", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/530.5 (KHTML, like Gecko) Chrome/2.0.172.33 Safari/530.5", "Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US) AppleWebKit/532.0 (KHTML, like Gecko) Chrome/3.0.195.38 Safari/532.0", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; en-US) AppleWebKit/533.4 (KHTML, like Gecko) Chrome/5.0.375.55 Safari/533.4", "Mozilla/5.0 (X11; U; Linux i686; en-US) AppleWebKit/534.3 (KHTML, like Gecko) Chrome/6.0.472.63 Safari/534.3", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/534.3 (KHTML, like Gecko) Chrome/6.0.472.55 Safari/534.3", "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US) AppleWebKit/534.7 (KHTML, like Gecko) Chrome/7.0.517.43 Safari/534.7", "Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US) AppleWebKit/534.10 (KHTML, like Gecko) Chrome/8.0.552.224 Safari/534.10 ChromePlus/1.5.2.0", "Mozilla/5.0 (en-us) AppleWebKit/534.14 (KHTML, like Gecko; Google Wireless Transcoder) Chrome/9.0.597 Safari/534.14", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/534.16 (KHTML, like Gecko) Chrome/10.0.648.151 Safari/534.16", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.24 (KHTML, like Gecko) Chrome/11.0.696.71 Safari/534.24", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.24 (KHTML, like Gecko) Iron/11.0.700.2 Chrome/11.0.700.2 Safari/534.24", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/534.24 (KHTML, like Gecko) Chrome/11.0.696.65 Safari/534.24", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/534.30 (KHTML, like Gecko) Chrome/12.0.742.122 Safari/534.30 ChromePlus/1.6.3.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.30 (KHTML, like Gecko) Chrome/12.0.742.112 Safari/534.30", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/13.0.782.107 Safari/535.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/535.1 (KHTML, like Gecko) RockMelt/0.9.64.361 Chrome/13.0.782.218 Safari/535.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/13.0.782.112 Safari/535.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/13.0.782.220 Safari/535.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/14.0.835.202 Safari/535.1", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/14.0.835.202 Safari/535.1", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/535.2 (KHTML, like Gecko) Chrome/15.0.874.120 Safari/535.2", "Mozilla/5.0 (X11; Linux i686) AppleWebKit/535.2 (KHTML, like Gecko) Ubuntu/10.04 Chromium/15.0.874.106 Chrome/15.0.874.106 Safari/535.2", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_2) AppleWebKit/535.2 (KHTML, like Gecko) Chrome/15.0.874.106 Safari/535.2", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/535.7 (KHTML, like Gecko) Chrome/16.0.912.75 Safari/535.7", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/535.7 (KHTML, like Gecko) Chrome/16.0.912.75 Safari/535.7", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/535.11 (KHTML, like Gecko) Chrome/17.0.963.83 Safari/535.11", "Mozilla/5.0 (Linux; Android 4.2.2; ja-jp; SC-04E Build/JDQ39) AppleWebKit/535.19 (KHTML, like Gecko) Version/1.0 Chrome/18.0.1025.308 Mobile Safari/535.19", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.1634 Safari/535.19 YE", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.52 Safari/536.5", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/536.5 (KHTML, like Gecko) Chrome/19.0.1084.46 Safari/536.5 Nichrome/self/19", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/536.5 (KHTML, like Gecko) Iron/19.0.1100.0 Chrome/19.0.1100.0 Safari/536.5", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/536.11 (KHTML, like Gecko) Chrome/20.0.1132.57 Safari/536.11", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.79 Safari/537.1", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/537.4 (KHTML, like Gecko) Chrome/22.0.1229.92 Safari/537.4", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.4 (KHTML, like Gecko) Chrome/22.0.1229.94 Safari/537.4 Sleipnir/3.8.4", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.4 (KHTML, like Gecko) Chrome/22.0.1250.0 Iron/22.0.2150.0 Safari/537.4", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.11 (KHTML, like Gecko) Chrome/23.0.1300.0 Iron/23.0.1300.0 Safari/537.11", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.11 (KHTML, like Gecko) Chrome/23.0.1271.97 Safari/537.11", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.17 (KHTML, like Gecko) Chrome/24.0.1312.57 Safari/537.17", "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.17 (KHTML, like Gecko) Chrome/24.0.1312.57 Safari/537.17", "Mozilla/5.0 (Linux; Android 4.2.2; Nexus 7 Build/JDQ39) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.169 Safari/537.22", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.97 Safari/537.22", "Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.31 (KHTML, like Gecko) Chrome/26.0.1410.63 Safari/537.31", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_3) AppleWebKit/537.31 (KHTML, like Gecko) Chrome/26.0.1410.43 Safari/537.31", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.31 (KHTML, like Gecko) Chrome/26.0.1410.64 Safari/537.31", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.116 Safari/537.36", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.116 Safari/537.36", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.110 Safari/537.36 Sleipnir/4.1.4", "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.116 Safari/537.36", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/28.0.1500.52 Safari/537.36", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/28.0.1500.63 Safari/537.36", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0; SLCC1; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30618; Lunascape 4.7.3)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; Lunascape 5.0 alpha2)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729; Lunascape 5.0 alpha2)", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0; Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1) ; SLCC1; .NET CLR 2.0.50727; Media Center PC 5.0; .NET CLR 3.0.04506; Tablet PC 2.0; Lunascape 5.0 alpha2)", "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0; SLCC2; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30729; Media Center PC 6.0; MAGW; .NET4.0C; Lunascape 6.5.8.24780)", "Mozilla/2.02 (Macintosh; I; PPC)", "Mozilla/3.01 (Macintosh; I; PPC)", "Mozilla/4.01 (Macintosh; I; PPC)", "Mozilla/4.03 [en]C-IMS (Win95; I)", "Mozilla/4.04 [ja] (Win95; I ;Nav)", "Mozilla/4.04 [ja] (Win95; I)", "Mozilla/4.04 [ja] (WinNT; I ;Nav)", "Mozilla/4.04 [ja] (WinNT; I)", "Mozilla/4.04 [ja] (Macintosh; I; PPC Nav)", "Mozilla/4.04 [en] (X11; I; SunOS 5.5 sun4u)", "Mozilla/4.05 [ja] (Win95; I)", "Mozilla/4.05 (Macintosh; I; PPC)", "Mozilla/4.06 [ja] (Win98; I)", "Mozilla/4.06 [ja] (Macintosh; I; PPC)", "User-Agent: Mozilla/4.07 [ja_JP.EUC] (X11; I; MarkAgent FreeBSD 2.2.8-RELEASE i386; Nav)", "User-Agent: Mozilla/4.07 [ja_JP.EUC] (X11; I; FreeBSD 2.2.8-RELEASE i386; Nav)", "Mozilla/4.08 (Macintosh; I; PPC)", "Mozilla/4.5 [ja] (Win95; I)", "Mozilla/4.5 [ja] (Win98; I)", "Mozilla/4.5 [ja] (WinNT; I)", "Mozilla/4.5 (Macintosh; I; PPC)", "Mozilla/4.51 [ja] (Win95; I)", "Mozilla/4.51 [ja] (Win98; I)", "Mozilla/4.51 [ja] (WinNT; I)", "Mozilla/4.51 [ja] (X11; I; SunOS 5.8 sun4u)", "Mozilla/4.6 [ja] (Win95; I)", "Mozilla/4.6 [ja] (Win98; I)", "Mozilla/4.6 [ja] (WinNT; I)", "Mozilla/4.6 [ja] (WinNT; I)", "Mozilla/4.7 [en] (WinNT; I)", "Mozilla/4.7 [ja] (Win95; I)", "Mozilla/4.7 [ja] (Win98; I)", "Mozilla/4.7 [ja] (WinNT; I)", "Mozilla/4.7 [ja] (WinNT; I)", "Mozilla/4.7 [ja] (WinNT; U)", "Mozilla/4.7 [ja] (Macintosh; I; PPC)", "Mozilla/4.76 [en_jp] (X11; U; SunOS 5.8 sun4u)", "Mozilla/4.76 [ja] (X11; U; SunOS 5.8 sun4u)", "Mozilla/4.78 [ja] (X11; U; SunOS 5.9 sun4u)", "Mozilla/4.8 [ja] (X11; U; SunOS 5.7 sun4u)", "Mozilla/5.0 (Windows; U; Win98; ja-JP; m18) Gecko/20001108 Netscape6/6.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; ja-JP; m18) Gecko/20010131 Netscape6/6.01", "Mozilla/5.0 (Windows; U; Win 9x 4.90; ja-JP; rv:0.9.4) Gecko/20011128 Netscape6/6.2.1", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:0.9.4.1) Gecko/20020508 Netscape6/6.2.3", "Mozilla/5.0 (Macintosh; N; PPC; ja-JP; macja-pub12) Gecko/20001108 Netscape6/6.0", "Mozilla/5.0 (Macintosh; U; PPC; ja-JP; rv:0.9.2) Gecko/20010726 Netscape6/6.1", "Mozilla/5.0 (Macintosh; U; PPC; ja-JP; rv:0.9.4) Gecko/20011022 Netscape6/6.2", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en-US; rv:0.9.4.1) Gecko/20020315 Netscape6/6.2.2", "Mozilla/5.0 (Windows; U; Win98; en-US; rv:1.0.2) Gecko/20030208 Netscape/7.02", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:1.4) Gecko/20030624 Netscape/7.1 (ax)", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.4) Gecko/20030624 Netscape/7.1 (ax)", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.2) Gecko/20040804 Netscape/7.2 (ax)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.2) Gecko/20040805 Netscape/7.2", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-JP; rv:1.0.2) Gecko/20021120 Netscape/7.01", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; ja-JP; rv:1.4) Gecko/20030624 Netscape/7.1", "Mozilla/5.0 (X11; U; SunOS sun4u; ja-JP; rv:1.0.1) Gecko/20020921 Netscape/7.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; ja-JP; rv:1.5) Gecko/20031007", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.6) Gecko/20040113", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.7) Gecko/20040616", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; en-US; rv:1.6) Gecko/20040113", "Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.2.1) Gecko/20030225", "Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.4.1) Gecko/20031030", "Mozilla/5.0 (X11; U; FreeBSD i386; en-US; rv:1.7.1) Gecko/20040805", "Mozilla/5.0 (X11; U; SunOS sun4u; ja-JP; rv:1.4) Gecko/20040414", "Mozilla/5.0 (X11; U; Linux i686; rv:1.7.3) Gecko/20040913 Firefox/0.10", "Mozilla/5.0 (Windows; U; Windows NT 5.0; rv:1.7.3) Gecko/20040913 Firefox/0.10", "Mozilla/5.0 (Windows; U; Windows NT 5.1; rv:1.7.3) Gecko/20040913 Firefox/0.10", "Mozilla/5.0 (X11; U; Linux i686; rv:1.7.3) Gecko/20041001 Firefox/0.10.1", "Mozilla/5.0 (Windows; U; Windows NT 5.1; rv:1.7.3) Gecko/20041001 Firefox/0.10.1", "Mozilla/5.0 (Windows; U; Windows NT 5.0; rv:1.7.3) Gecko/20041001 Firefox/0.10.1", "Mozilla/5.0 (Windows; U; Windows NT 5.2; rv:1.7.3) Gecko/20041001 Firefox/0.10.1", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:1.6) Gecko/20040206 Firefox/0.8", "Mozilla/5.0 (Windows; U; Win98; en-US; rv:1.6) Gecko/20040206 Firefox/0.8", "Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.6) Gecko/20040207 Firefox/0.8", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; en-US; rv:1.6) Gecko/20040206 Firefox/0.8", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:1.7) Gecko/20040614 Firefox/0.9", "Mozilla/5.0 (X11; U; Linux i686; es-ES; rv:1.7) Gecko/20040708 Firefox/0.9", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:1.7) Gecko/20040707 Firefox/0.9.2", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.5) Gecko/20041107 Firefox/0.9.2 StumbleUpon/1.994", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja-JP; rv:1.7) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.7) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (Windows; U; Win98; en-US; rv:1.7) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.7) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.4) Gecko/20040803 Firefox/0.9.3", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7) Gecko/20041013 Firefox/0.9.3 (Ubuntu)", "Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.7) Gecko/20041013 Firefox/0.9.3 (Ubuntu)", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Win 9x 4.90; nl-NL; rv:1.7.5) Gecko/20041202 Firefox/1.0", "Mozilla/5.0 (Windows; U; Win 9x 4.90; nl-NL; rv:1.7.5) Gecko/20041202 Firefox/1.0", "Mozilla/5.0 (Windows; U; Win98; nl-NL; rv:1.7.5) Gecko/20041202 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; de-DE; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; de-DE; rv:1.7.5) Gecko/20041122 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-GB; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-GB; rv:1.7.5) Gecko/20041110 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; fr-FR; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; it-IT; rv:1.7.5) Gecko/20041110 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; de-DE; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; de-DE; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; de-DE; rv:1.7.5) Gecko/20041122 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-GB; rv:1.7.5) Gecko/20041110 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0 StumbleUpon/1.999", "Mozilla/5.0 (Windows; U; Windows NT 5.1; es-ES; rv:1.7.5) Gecko/20041210 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; fr-FR; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; nl-NL; rv:1.7.5) Gecko/20041202 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.1; sv-SE; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.2; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.2; en-US; rv:1.8b) Gecko/20050212 Firefox/1.0+ (MOOX M3)", "Mozilla/5.0 (Windows; U; WinNT4.0; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (Windows; U; Windows NT 5.0; en-US; rv:1.8b) Gecko/20050118 Firefox/1.0+", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.8b) Gecko/20050118 Firefox/1.0+", "Mozilla/5.0 (X11; U; FreeBSD i386; en-US; rv:1.7.5) Gecko/20050103 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i386; en-US; rv:1.7.5) Gecko/20041109 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; de-DE; rv:1.7.5) Gecko/20041128 Firefox/1.0 (Debian package 1.0-4)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.7.5) Gecko/20041108 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041111 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041111 Firefox/1.0 (Debian package 1.0-2)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041116 Firefox/1.0 (Ubuntu) (Ubuntu package 1.0-2ubuntu3)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041119 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041123 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041128 Firefox/1.0 (Debian package 1.0-4)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041130 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041214 Firefox/1.0 StumbleUpon/1.999", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20041219 Firefox/1.0 (Debian package 1.0+dfsg.1-1)", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.5) Gecko/20050110 Firefox/1.0 (Debian package 1.0+dfsg.1-2)", "Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.7.5) Gecko/20041107 Firefox/1.0", "Mozilla/5.0 (X11; U; Linux i686; nl-NL; rv:1.7.5) Gecko/20050221 Firefox/1.0 (Ubuntu) (Ubuntu package 1.0+dfsg.1-6ubuntu1)", "Mozilla/5.0 (X11; U; Linux i686; rv:1.8b) Gecko/20050124 Firefox/1.0+", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.6) Gecko/20050306 Firefox/1.0.1 (Debian package 1.0.1-2)", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.6) Gecko/20050223 Firefox/1.0.1", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.6) Gecko/20050225 Firefox/1.0.1", "Mozilla/5.0 (Windows; U; Windows NT 5.1; de-DE; rv:1.7.6) Gecko/20050226 Firefox/1.0.1", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; ja-JP-mac; rv:1.8) Gecko/20051111 Firefox/1.5", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja; rv:1.8) Gecko/20051111 Firefox/1.5", "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.8) Gecko/20051111 Firefox/1.5", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja; rv:1.8.0.1) Gecko/20060111 Firefox/1.5.0.1", "Mozilla/5.0 (X11; U; Linux i686; ja; rv:1.8.0.2) Gecko/20060308 Firefox/1.5.0.2", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja; rv:1.8.0.3) Gecko/20060426 Firefox/1.5.0.3", "Mozilla/5.0 (X11; U; Linux i686; ja; rv:1.8.0.4) Gecko/20060508 Firefox/1.5.0.4", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja; rv:1.8.1.20) Gecko/20081217 Firefox/2.0.0.20", "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.8.1.12) Gecko/20080201 Firefox/2.0.0.12", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X; ja-JP-mac; rv:1.8.1.20) Gecko/20081217 Firefox/2.0.0.20", "Mozilla/5.0 (Windows; U; Windows NT 5.1; ja; rv:1.9.0.6) Gecko/2009011913 Firefox/3.0.6", "Mozilla/5.0 (Windows; U; Windows NT 6.0; ja; rv:1.9.0.6) Gecko/2009011913 Firefox/3.0.6 (.NET CLR 3.5.30729)", "Mozilla/5.0 (Windows; U; Windows NT 6.0; ja; rv:1.9.0.17) Gecko/2009122116 Firefox/3.0.17 GTB6 (.NET CLR 3.5.30729)", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.5; ja-JP-mac; rv:1.9.0.6) Gecko/2009011912 Firefox/3.0.6", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.5; ja-JP-mac; rv:1.9.0.6) Gecko/2009011912 Firefox/3.0.6 GTB5", "Mozilla/5.0 (Windows NT 6.1; rv:2.0) Gecko/20100101 Firefox/4.0", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.5; rv:2.0) Gecko/20100101 Firefox/4.0", "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:5.0) Gecko/20100101 Firefox/5.0", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.5; rv:5.0.1) Gecko/20100101 Firefox/5.0.1", "Mozilla/5.0 (Windows NT 5.1; rv:6.0) Gecko/20100101 Firefox/6.0", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv:6.0.2) Gecko/20100101 Firefox/6.0.2", "Mozilla/5.0 (Windows NT 6.0; rv:7.0.1) Gecko/20100101 Firefox/7.0.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.7; rv:7.0.1) Gecko/20100101 Firefox/7.0.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.5; rv:8.0.1) Gecko/20100101 Firefox/8.0.1", "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:9.0.1) Gecko/20100101 Firefox/9.0.1", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.7; rv:9.0.1) Gecko/20100101 Firefox/9.0.1", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/85.7 (KHTML, like Gecko) Safari/85.6", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/85.7 (KHTML, like Gecko) Safari/85.7", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; de-de) AppleWebKit/85.8.2 (KHTML, like Gecko) Safari/85.8", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en-us) AppleWebKit/85.8.2 (KHTML, like Gecko) Safari/85.8.1", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en-us) AppleWebKit/85.8.5 (KHTML, like Gecko) Safari/85.8", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/103u (KHTML, like Gecko) Safari/100.1", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/124 (KHTML, like Gecko) Safari/125.1", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/125.2 (KHTML, like Gecko) Safari/125.8", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/125.4 (KHTML, like Gecko) Safari/125.9", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en-us) AppleWebKit/125.5.5 (KHTML, like Gecko) Safari/125.11", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; de-de) AppleWebKit/125.5.5 (KHTML, like Gecko) Safari/125.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en-us) AppleWebKit/125.5.5 (KHTML, like Gecko) Safari/125.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; nl-nl) AppleWebKit/125.5.5 (KHTML, like Gecko) Safari/125.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; en) AppleWebKit/125.5.6 (KHTML, like Gecko) Safari/125.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.1 (KHTML, like Gecko) Safari/312", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.1.1 (KHTML, like Gecko) Safari/312", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.5 (KHTML, like Gecko) Safari/312.3", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.8 (KHTML, like Gecko) Safari/312.5", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.8 (KHTML, like Gecko) Safari/312.6", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/412 (KHTML, like Gecko) Safari/412", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/412.6.2 (KHTML, like Gecko) Safari/125.11", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/412.6.2 (KHTML, like Gecko) Safari/412.2.2", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/412.7 (KHTML, like Gecko) Safari/412.5", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/416.11 (KHTML, like Gecko) Safari/416.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/418 (KHTML, like Gecko) Safari/417.9.2", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X; ja-jp) AppleWebKit/418 (KHTML, like Gecko) Safari/417.9.3", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/522.11.1 (KHTML, like Gecko) Version/3.0.3 Safari/522.12.1", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X; ja-jp) AppleWebKit/523.10.3 (KHTML, like Gecko) Version/3.0.4 Safari/523.10", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/523.10.6 (KHTML, like Gecko) Version/3.0.4 Safari/523.10.6", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/523.12 (KHTML, like Gecko) Version/3.0.4 Safari/523.12", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/523.12.2 (KHTML, like Gecko) Version/3.0.4 Safari/523.12.2", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X 10_4_11; ja-jp) AppleWebKit/525.13 (KHTML, like Gecko) Version/3.1 Safari/525.13", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_3; ja-jp) AppleWebKit/525.18 (KHTML, like Gecko) Version/3.1.1 Safari/525.20", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X 10_4_11; ja-jp) AppleWebKit/525.18 (KHTML, like Gecko) Version/3.1.2 Safari/525.22", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_5; ja-jp) AppleWebKit/525.26.2 (KHTML, like Gecko) Version/3.2 Safari/525.26.12", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_6; ja-jp) AppleWebKit/525.27.1 (KHTML, like Gecko) Version/3.2.1 Safari/525.27.1", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_6; ja-jp) AppleWebKit/528.16 (KHTML, like Gecko) Version/4.0 Safari/528.16", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_2; ja-jp) AppleWebKit/531.21.8 (KHTML, like Gecko) Version/4.0.4 Safari/531.21.10", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; ja-jp) AppleWebKit/531.21.11 (KHTML, like Gecko) Version/4.0.4 Safari/531.21.10", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; ja-jp) AppleWebKit/533.16 (KHTML, like Gecko) Version/5.0 Safari/533.16", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.52.7 (KHTML, like Gecko) Version/5.1.2 Safari/534.52.7", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.57.2 (KHTML, like Gecko) Version/5.1.7 Safari/534.57.2", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8) AppleWebKit/536.25 (KHTML, like Gecko) Version/6.0 Safari/536.25", "Mozilla/4.0 (Windows 95;US) Opera 3.62 [en]", "Mozilla/4.0 (compatible; MSIE 5.0; Mac_PowerPC) Opera 6.0 [en]", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 2000) Opera 6.01 [ja]", "Mozilla/4.0 (compatible; MSIE 5.0; Windows ME) Opera 6.03 [ja]", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 2000) Opera 6.05 [ja]", "Mozilla/4.0 (compatible; MSIE 5.0; Windows XP) Opera 6.06 [ja]", "Mozilla/4.0 (compatible; MSIE 5.0; Windows 2000) Opera 6.06 [ja]", "Opera 7.11", "Opera/7.23 (Windows NT 5.0; U) [ja]", "Opera/7.52 (Windows NT 5.1; U) [en]", "Opera/7.53 (Windows NT 5.0; U) [ja]", "Opera/7.54 (Windows NT 5.0; U) [ja]", "Opera/7.54 (Windows NT 5.1; U) [en]", "Opera/7.54 (Windows NT 5.1; U) [pl]", "Opera/7.54 (X11; Linux i686; U) [en]", "Opera/7.54 (X11; Linux i686; U) [sv]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.11 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.21 [pt-BR]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.22 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0) Opera 7.23 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0) Opera 7.23 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; X11; Linux i686) Opera 7.23 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.23 [fr]", "Mozilla/4.0 (compatible; MSIE 6.0; Mac_PowerPC) Opera 7.50 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0) Opera 7.53 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.53 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.54 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0) Opera 7.54 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1) Opera 7.54 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0) Opera 7.54u1 [ja]", "Opera/7.54 (Windows 98; U) [ja]", "Mozilla/4.78 (Windows NT 5.1; U) Opera 7.23 [ja]", "Mozilla/5.0 (Windows NT 5.0; U) Opera 7.54 [en]", "Mozilla/4.0 (compatible; MSIE 6.0; X11; OpenBSD i386) Opera 7.54 [en]", "Opera/8.0 (X11; Linux i686; U; ja)", "Opera/8.01 (Windows ME; U; ja)", "Opera/8.01 (Windows NT 5.1; U; ja)", "Mozilla/4.0 (compatible; MSIE 6.0; Mac_PowerPC Mac OS X; ja) Opera 8.01", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; ja) Opera 8.01", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; tr) Opera 8.02", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; ja) Opera 8.02", "Opera/8.5 (Windows NT 5.0; U; ja)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; ja) Opera 8.5", "Opera/8.51 (Windows NT 5.1; U; ja)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; ja) Opera 8.51", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; de) Opera 8.52", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; pl) Opera 8.52", "Mozilla/5.0 (Windows NT 5.1; U; ja) Opera 8.52", "Opera/8.53 (Windows NT 5.1; U; ja)", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; ja) Opera 8.53", "Opera/8.54 (Windows NT 5.1; U; ja)", "Opera/8.54 (Windows NT 5.0; U; ja)", "Mozilla/5.0 (X11; Linux i686; U; cs) Opera 8.54", "Mozilla/4.0 (compatible; MSIE 6.0; KDDI-SA39) Opera 8.60 [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; ja) Opera 9.00", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; en) Opera 9.00", "Opera/9.00 (Windows NT 5.1; U; ja)", "Opera/9.0 (Windows NT 5.1; U; en)", "Opera/9.00 (Macintosh; PPC Mac OS X; U; ja)", "Opera/9.02 (Macintosh; PPC Mac OS X; U; ja)", "Opera/9.02 (Windows NT 5.1; U; zh-tw)", "Opera/9.10 (Windows NT 6.0; U; ja)", "Opera/9.21 (Windows NT 6.0; U; ja)", "Opera/9.22 (Windows NT 5.1; U; ja)", "Opera/9.23 (Windows NT 5.1; U; ja)", "Opera/9.23 (Windows ME; U; ja)", "Opera/9.26 (Windows NT 5.1; U; ja)", "Opera/9.51 (Windows NT 5.1; U; ja)", "Opera/9.52 (Macintosh; Intel Mac OS X; U; ja)", "Opera/9.52 (Windows NT 5.1; U; ja)", "Opera/9.60 (Macintosh; Intel Mac OS X; U; ja) Presto/2.1.1", "Opera/9.60 (Windows NT 5.1; U; ja) Presto/2.1.1", "Opera/9.61 (Windows NT 5.1; U; ja) Presto/2.1.1", "Opera/9.62 (Windows NT 5.1; U; ja) Presto/2.1.1", "Mozilla/4.0 (compatible; MSIE 6.0; X11; Linux i686; ja) Opera 10.10", "Opera/9.80 (Windows NT 6.1; U; ja) Presto/2.9.168 Version/11.50", "Opera/9.80 (Windows NT 6.1; U; ja) Presto/2.10.229 Version/11.60", "Opera/9.80 (Macintosh; Intel Mac OS X 10.6.8; U; ja) Presto/2.10.289 Version/12.00", "Konqueror/3.1; (Konqueror/3.1; i686 Linux;;datecode)", "Mozilla/5.0 (compatible; Konqueror/3.1; Linux 2.6.5-1.358; X11; i686; , ja_JP.UTF-8, ja_JP, ja)", "Mozilla/5.0 (compatible; Konqueror/3.2; Linux) (KHTML, like Gecko)", "Mozilla/5.0 (compatible; Konqueror/3.2; Linux; X11) (KHTML, like Gecko)", "Mozilla/5.0 (compatible; konqueror/3.3; linux 2.4.21-243-smp46) (KHTML, like Gecko)", "Mozilla/5.0 (compatible; Konqueror/3.3; Linux) (KHTML, like Gecko)", "Mozilla/3.0 (DreamPassport/2.0)", "Mozilla/3.0 (DreamPassport/2.1)", "Mozilla/3.0 (DreamPassport/2.1; SEGA/ROOMMANIA203)", "Mozilla/3.0 (PowerPC [ja] Mac OS 8.1; Sun)", "Mozilla/2.0 (BTRON Basic Browser Version 1.0 Beta; B-right/V)", "iCab J/Pre1.9 (Macintosh; I; PPC)", "iCab J/2.9.8 (Macintosh; U; PPC)", "Mozilla/4.5 (compatible; iCab 2.9.8; Macintosh; U; PPC)", "Mozilla/4.5 (compatible; iCab 2.9.8; Macintosh; U; PPC; Mac OS X)", "Lynx/2.8.5rel.1 libwww-FM/2.14 SSL-MM/1.4.1 GNUTLS/1.0.16", "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.0.3705; .NET CLR 1.1.4322; Media Center PC 4.0) Sleipnir/2.30", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1) Sleipnir/2.41", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.2; SV1; .NET CLR 1.1.4322) Sleipnir/2.41", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; InfoPath.1; .NET CLR 2.0.50727) Sleipnir/2.5.17", "Mozilla/5.0 (Macintosh; U; PPC Mac OS X; ja-jp) AppleWebKit/312.8 (KHTML, like Gecko) Shiira/1.2.1 Safari/125", "Mozilla/3.0 (compatible; WebCapture 1.0; Windows)", "Mozilla/4.0 (compatible; WebCapture 3.0; Macintosh)", "Mozilla/4.0 (compatible; MSIE 6.0; Mac_PowerPC; Bridge/2.1.1.9; ja) Opera 9.20", "Googlebot/2.1 (+http://www.google.com/bot.html)", "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)", "msnbot/0.11 (+http://search.msn.com/msnbot.htm)", "msnbot/0.3 (+http://search.msn.com/msnbot.htm)", "msnbot/1.0 (+http://search.msn.com/msnbot.htm)", "proodleBot (www.proodle.com)", "psbot/0.1 (+http://www.picsearch.com/bot.html)", "ScSpider/0.2", "TutorGigBot/1.5 ( +http://www.tutorgig.info )", "YottaShopping_Bot/4.12 (+http://www.yottashopping.com) Shopping Search Engine", "Faxobot/1.0", "Gigabot/2.0", "MJ12bot/v0.8.7 (http://www.majestic12.co.uk/projects/dsearch/mj12bot.php?V=v0.8.7&NID=B0E44C4EE98B33C4&MID=EE1DD60ABC2AE863&BID=4B63485ECF966068726CCEAA8B8D2509&+)", "Mozilla/2.0 (compatible; Ask Jeeves/Teoma; +http://about.ask.com/en/docs/about/webmasters.shtml)", "Mozilla/6.0 (compatible; arameda.com Spider)", "ELinks/0.9.3 (textmode; Linux 2.6.8-1-k7-smp i686; 132x44)", "Mozilla/4.0 (compatible; Cerberian Drtrs Version-3.1-Build-17)", "Opera/9.10 (Nintendo Wii; U; ; 1621; ja)", "Mech.Mozilla/20.02 (Nintendo GameBoy; U) [ja]", "Mozilla/4.0 (compatible; MSIE 6.0; Nitro) Opera 8.50 [ja]", "Mozilla/4.0 (PSP PlayStation Portable); 2.00)", "Mozilla/4.0 (PS2; PlayStation BB Navigator 1.0) NetFront/3.0", "Mozilla/5.0 (PLAYSTATION 3; 1.00)", "Mozilla/5.0 (PDA; NF34PPC/1.0; like Gecko) NetFront/3.4", "SIE-M65/12 UP.Browser/7.0.0.1.c.3 (GUI) MMP/2.0 Profile/MIDP-2.0 Configuration/CLDC-1.1", "UP.Browser/6.2.0.5 (GUI) MMP/2.0", "UP.Browser/6.2.0.5.c.1.100 (GUI) MMP/2.0", "UP.Browser/6.2.0.5 (GUI) MMP/2.0", "UP.Browser/6.0.8.3 (GUI) MMP/1.1", "UP.Browser/6.2.0.7.3.129 (GUI) MMP/2.0", "UP.Browser/6.0.8.2 (GUI) MMP/1.1", "UP.Browser/6.0.8.2 (GUI) MMP/1.1", "UP.Browser/6.2.0.4 (GUI) MMP/2.0", "UP.Browser/6.0.8.2 (GUI) MMP/1.1", "UP.Browser/6.0.8.3 (GUI) MMP/1.1", "UP.Browser/6.2.0.5.1 (GUI) MMP/2.0", "UP.Browser/6.2.0.5 (GUI) MMP/2.0", "UP.Browser/6.0.8.2 (GUI) MMP/1.1", "UP.Browser/6.0.8.3 (GUI) MMP/1.1", "DoCoMo/1.0/D501i", "DoCoMo/1.0/F501i", "DoCoMo/1.0/N501i", "DoCoMo/1.0/P501i", "DoCoMo/1.0/D502i", "DoCoMo/1.0/D502i/c10", "DoCoMo/1.0/F502i", "DoCoMo/1.0/F502i/c10", "DoCoMo/1.0/N502i", "DoCoMo/1.0/N502i/c08", "DoCoMo/1.0/P502i", "DoCoMo/1.0/P502i/c10", "DoCoMo/1.0/NM502i", "DoCoMo/1.0/NM502i/c10", "DoCoMo/1.0/SO502i", "DoCoMo/1.0/F502it", "DoCoMo/1.0/F502it/c10", "DoCoMo/1.0/N502it", "DoCoMo/1.0/N502it/c10", "DoCoMo/1.0/SO502iWM/c10", "DoCoMo/1.0/SH821i", "DoCoMo/1.0/SH821i/c10", "DoCoMo/1.0/N821i", "DoCoMo/1.0/N821i/c08", "DoCoMo/1.0/P821i", "DoCoMo/1.0/P821i/c08", "DoCoMo/1.0/D209i", "DoCoMo/1.0/D209i/c10", "DoCoMo/1.0/ER209i", "DoCoMo/1.0/ER209i/c15", "DoCoMo/1.0/F209i", "DoCoMo/1.0/F209i/c10", "DoCoMo/1.0/KO209i", "DoCoMo/1.0/N209i", "DoCoMo/1.0/N209i/c08", "DoCoMo/1.0/P209i", "DoCoMo/1.0/P209i/c10", "DoCoMo/1.0/P209is", "DoCoMo/1.0/P209is/c10", "DoCoMo/1.0/R209i", "DoCoMo/1.0/P651ps", "DoCoMo/1.0/P651ps/c10", "DoCoMo/1.0/R691i", "DoCoMo/1.0/ R691i/c10", "DoCoMo/1.0/F210i/c10", "DoCoMo/1.0/N210i", "DoCoMo/1.0/N210i/c10", "DoCoMo/1.0/P210i", "DoCoMo/1.0/P210i/c10", "DoCoMo/1.0/KO210i", "DoCoMo/1.0/KO210i/c10", "DoCoMo/1.0/F671i/c10", "DoCoMo/1.0/D210i/c10", "DoCoMo/1.0/SO210i/c10", "DoCoMo/1.0/F503i/c10", "DoCoMo/1.0/F503iS/c10", "DoCoMo/1.0/P503i/c10", "DoCoMo/1.0/P503iS/c10", "DoCoMo/1.0/N503i/c10", "DoCoMo/1.0/N503iS/c10", "DoCoMo/1.0/SO503i/c10", "DoCoMo/1.0/SO503iS/c10", "DoCoMo/1.0/D503i/c10", "DoCoMo/1.0/D503iS/c10", "DoCoMo/1.0/F211i/c10", "DoCoMo/1.0/D211i/c10", "DoCoMo/1.0/N211i/c10", "DoCoMo/1.0/N211iS/c10", "DoCoMo/1.0/P211i/c10", "DoCoMo/1.0/P211iS/c10", "DoCoMo/1.0/SO211i/c10", "DoCoMo/1.0/R211i/c10", "DoCoMo/1.0/SH251i/c10", "DoCoMo/1.0/SH251iS/c10", "DoCoMo/1.0/R692i/c10", "DoCoMo/1.0/D504i/c10", "DoCoMo/1.0/D504i/c30/TD", "DoCoMo/1.0/D504i/c10/TJ", "DoCoMo/1.0/F504i/c10/TB", "DoCoMo/1.0/F504i/c30/TD", "DoCoMo/1.0/F504i/c10/TJ", "DoCoMo/1.0/F504iS/c10/TB", "DoCoMo/1.0/F504iS/c30/TD", "DoCoMo/1.0/F504iS/c10/TJ", "DoCoMo/1.0/N504i/c10/TB", "DoCoMo/1.0/N504i/c30/TD", "DoCoMo/1.0/N504i/c10/TJ", "DoCoMo/1.0/N504iS/c10/TB", "DoCoMo/1.0/N504iS/c30/TD", "DoCoMo/1.0/N504iS/c10/TJ", "DoCoMo/1.0/SO504i/c10/TB", "DoCoMo/1.0/SO504i/c30/TD", "DoCoMo/1.0/SO504i/c10/TJ", "DoCoMo/1.0/P504i/c10/TB", "DoCoMo/1.0/P504i/c30/TD", "DoCoMo/1.0/P504i/c10/TJ", "DoCoMo/1.0/P504iS/c10/TB", "DoCoMo/1.0/P504iS/c30/TD", "DoCoMo/1.0/P504iS/c10/TJ", "DoCoMo/1.0/D251i/c10", "DoCoMo/1.0/D251iS/c10/TB", "DoCoMo/1.0/F251i/c10/TB", "DoCoMo/1.0/F251i/c10/c10/TB", "DoCoMo/1.0/N251i/c10/TB", "DoCoMo/1.0/N251iS/c10/TB", "DoCoMo/1.0/P251iS/c10/TB", "DoCoMo/1.0/F671iS/c10/TB", "DoCoMo/1.0/F212i/c10/TB", "DoCoMo/1.0/SO212i/c10/TB", "DoCoMo/1.0/F661i/c10/TB", "DoCoMo/1.0/F672i/c10/TB", "DoCoMo/1.0/SO213i/c10/TB", "DoCoMo/1.0/N2701/c10/TB", "DoCoMo/1.0/D505i/c20/TB/W20H10", "DoCoMo/1.0/D505i/c30/TD", "DoCoMo/1.0/D505i/c20/TJ", "DoCoMo/1.0/SO505i/c20/TB/W21H09", "DoCoMo/1.0/SO505i/c20/TB/W18H08", "DoCoMo/1.0/SO505i/c30/TD", "DoCoMo/1.0/SO505i/c20/TJ", "DoCoMo/1.0/SH505i/c20/TB/W24H12", "DoCoMo/1.0/SH505i2/c20/TB/W24H12", "DoCoMo/1.0/SH505i/c30/TD", "DoCoMo/1.0/SH505i2/c30/TD", "DoCoMo/1.0/SH505i/c20/TJ", "DoCoMo/1.0/SH505i2/c20/TJ", "DoCoMo/1.0/N505i/c20/TB/W20H10", "DoCoMo/1.0/N505i/c20/TB/W30H14", "DoCoMo/1.0/N505i/c20/TB/W16H08", "DoCoMo/1.0/N505i/c30/TD", "DoCoMo/1.0/N505i/c20/TJ", "DoCoMo/1.0/F505i/c20/TB/W20H10", "DoCoMo/1.0/F505i/c20/TB/W24H12", "DoCoMo/1.0/F505i/c30/TD", "DoCoMo/1.0/F505i/c20/TJ", "DoCoMo/1.0/P505i/c20/TB/W20H10", "DoCoMo/1.0/P505i/c30/TD", "DoCoMo/1.0/P505i/c20/TJ", "DoCoMo/1.0/D505iS/c20/TB/W20H10", "DoCoMo/1.0/D505iS/c30/TD", "DoCoMo/1.0/D505iS/c20/TJ", "DoCoMo/1.0/P505iS/c20/TB/W20H10", "DoCoMo/1.0/P505iS/c30/TD", "DoCoMo/1.0/P505iS/c20/TJ", "DoCoMo/1.0/N505iS/c20/TB/W20H10", "DoCoMo/1.0/N505iS/c20/TB/W30H14", "DoCoMo/1.0/N505iS/c20/TB/W16H08", "DoCoMo/1.0/N505iS/c30/TD", "DoCoMo/1.0/N505iS/c20/TJ", "DoCoMo/1.0/SO505iS/c20/TB/W20H10", "DoCoMo/1.0/SO505iS/c20/TB/W40H21", "DoCoMo/1.0/SO505iS/c20/TB/W30H16", "DoCoMo/1.0/SO505iS/c20/TB/W16H08", "DoCoMo/1.0/SO505iS/c30/TD", "DoCoMo/1.0/SO505iS/c20/TJ", "DoCoMo/1.0/SH505iS/c20/TB/W24H12", "DoCoMo/1.0/SH505iS/c30/TD", "DoCoMo/1.0/SH505iS/c20/TJ", "DoCoMo/1.0/F505iGPS/c20/TB/W20H1", "DoCoMo/1.0/F505iGPS/c20/TB/W24H12", "DoCoMo/1.0/F505iGPS/c30/TD", "DoCoMo/1.0/F505iGPS/c20/TJ", "DoCoMo/1.0/D252i/c10/TB/W25H12", "DoCoMo/1.0/SH252i/c20/TB/W24H12", "DoCoMo/1.0/P252i/c10/TB/W22H10", "DoCoMo/1.0/N252i/c10/TB/W22H10", "DoCoMo/1.0/N252i/c10/TB/W26H11", "DoCoMo/1.0/N252i/c10/TB/W16H07", "DoCoMo/1.0/P252iS/c10/TB/W22H10", "DoCoMo/1.0/D506i/c20/TB/W20H10", "DoCoMo/1.0/D506i/c20/TB/W40H18", "DoCoMo/1.0/D506i/c20/TB/W30H14", "DoCoMo/1.0/D506i/c20/TB/W16H08", "DoCoMo/1.0/D506i/c30/TD", "DoCoMo/1.0/D506i/c20/TJ", "DoCoMo/1.0/F506i/c20/TB/W20H10", "DoCoMo/1.0/F506i/c20/TB/W24H12", "DoCoMo/1.0/F506i/c30/TD", "DoCoMo/1.0/F506i/c20/TJ", "DoCoMo/1.0/N506i/c20/TB/W20H11", "DoCoMo/1.0/N506i/c20/TB/W30H15", "DoCoMo/1.0/N506i/c20/TB/W16H09", "DoCoMo/1.0/N506i/c30/TD", "DoCoMo/1.0/N506i/c20/TJ", "DoCoMo/1.0/P506iC/c20/TB/W20H10", "DoCoMo/1.0/P506iC/c20/TB/W30H14", "DoCoMo/1.0/P506iC/c30/TD", "DoCoMo/1.0/P506iC/c20/TJ", "DoCoMo/1.0/SH506iC/c20/TB/W24H12", "DoCoMo/1.0/SH506iC/c20/TB/W40H19", "DoCoMo/1.0/SH506iC/c20/TB/W20H10", "DoCoMo/1.0/SH506iC/c30/TD", "DoCoMo/1.0/SH506iC/c20/TJ", "DoCoMo/1.0/SO506iC/c20/TB/W20H10", "DoCoMo/1.0/SO506iC/c20/TB/W40H21", "DoCoMo/1.0/SO506iC/c20/TB/W30H16", "DoCoMo/1.0/SO506iC/c20/TB/W16H08", "DoCoMo/1.0/SO506iC/c30/TD", "DoCoMo/1.0/SO506iC/c20/TJ", "DoCoMo/1.0/D253i/c10/TB/W17H09", "DoCoMo/1.0/D253i/c10/TB/W14H07", "DoCoMo/1.0/D253i/c10/TB/W25H12", "DoCoMo/1.0/D253i/c10/TB/W14H06", "DoCoMo/1.0/N253i/c10/TB/W20H10", "DoCoMo/1.0/N253i/c10/TB/W26H12", "DoCoMo/1.0/N253i/c10/TB/W16H08", "DoCoMo/1.0/P253i/c10/TB/W22H10", "DoCoMo/1.0/D253iWM/c10/TB/W27H08", "DoCoMo/1.0/D253iWM/c10/TB/W22H06", "DoCoMo/1.0/D253iWM/c10/TB/W36H10", "DoCoMo/1.0/P213i/c10/TB/W22H10", "DoCoMo/1.0/SH712m/c10", "DoCoMo/1.0/P751v/c100/s32/kPHS-K", "ASTEL/1.0/J-0511.00/c10/smel"
    ];
    const HTTP_STATUS_CODE: &'static [&'static u16] = &[
        &100_u16, &101_u16, &102_u16, &103_u16, &200_u16, &201_u16, &202_u16, &203_u16, &204_u16,
        &205_u16, &206_u16, &207_u16, &208_u16, &226_u16, &300_u16, &301_u16, &302_u16, &303_u16,
        &304_u16, &305_u16, &306_u16, &307_u16, &308_u16, &400_u16, &401_u16, &402_u16, &403_u16,
        &404_u16, &405_u16, &406_u16, &407_u16, &408_u16, &409_u16, &410_u16, &411_u16, &412_u16,
        &413_u16, &414_u16, &415_u16, &416_u16, &417_u16, &418_u16, &421_u16, &422_u16, &423_u16,
        &424_u16, &425_u16, &426_u16, &427_u16, &428_u16, &429_u16, &431_u16, &451_u16, &500_u16,
        &501_u16, &502_u16, &503_u16, &504_u16, &505_u16, &506_u16, &507_u16, &508_u16, &510_u16,
        &511_u16,
    ];

    // Compnay
    const COMPANY_SUFFIX: &'static [&'static str];
    const COMPANY_NAME: &'static [&'static str];
    const INDUSTRY: &'static [&'static str];

    // Address
    const STREET_NAME: &'static [&'static str];
    const CITY_NAME: &'static [&'static str];
    const STATE_NAME: &'static [&'static str];
    const COUNTRY_NAME: &'static [&'static str] = &[
        "Aruba",
        "Afghanistan",
        "Angola",
        "Anguilla",
        "Åland Islands",
        "Albania",
        "Andorra",
        "Netherlands Antilles",
        "United Arab Emirates",
        "Argentina",
        "Armenia",
        "American Samoa",
        "Antarctica",
        "French Southern Territories",
        "Antigua and Barbuda",
        "Australia",
        "Austria",
        "Azerbaijan",
        "Burundi",
        "Belgium",
        "Benin",
        "Burkina Faso",
        "Bangladesh",
        "Bulgaria",
        "Bahrain",
        "Bahamas and Herzegovina",
        "Saint Barthélemy",
        "Belarus",
        "Belize",
        "Bermuda",
        "Bolivia, Plurinational State of",
        "Brazil",
        "Barbados",
        "Brunei Darussalam",
        "Bhutan",
        "Bouvet Island",
        "Botswana",
        "Central African Republic",
        "Canada",
        "Cocos (Keeling) Islands",
        "Switzerland",
        "Chile",
        "China",
        "Côte d'Ivoire",
        "Cameroon",
        "Congo, the Democratic Republic of the",
        "Congo",
        "Cook Islands",
        "Colombia",
        "Comoros",
        "Cape Verde",
        "Costa Rica",
        "Cuba",
        "Christmas Island",
        "Cayman Islands",
        "Cyprus",
        "Czech Republic",
        "Germany",
        "Djibouti",
        "Dominica",
        "Denmark",
        "Dominican Republic",
        "Algeria",
        "Ecuador",
        "Egypt",
        "Eritrea",
        "Western Sahara",
        "Spain",
        "Estonia",
        "Ethiopia",
        "Finland",
        "Fiji",
        "Falkland Islands (Malvinas)",
        "France",
        "Faroe Islands",
        "Micronesia, Federated States of",
        "Gabon",
        "United Kingdom",
        "Georgia",
        "Guernsey",
        "Ghana",
        "Gibraltar",
        "Guinea",
        "Guadeloupe",
        "Gambia",
        "Guinea-Bissau",
        "Equatorial Guinea",
        "Greece",
        "Grenada",
        "Greenland",
        "Guatemala",
        "French Guiana",
        "Guam",
        "Guyana",
        "Hong Kong",
        "Heard Island and McDonald Islands",
        "Honduras",
        "Croatia",
        "Haiti",
        "Hungary",
        "Indonesia",
        "Isle of Man",
        "India",
        "British Indian Ocean Territory",
        "Ireland",
        "Iran, Islamic Republic of",
        "Iraq",
        "Iceland",
        "Israel",
        "Italy",
        "Jamaica",
        "Jersey",
        "Jordan",
        "Japan",
        "Kazakhstan",
        "Kenya",
        "Kyrgyzstan",
        "Cambodia",
        "Kiribati",
        "Saint Kitts and Nevis",
        "Korea, Republic of",
        "Kuwait",
        "Lao People's Democratic Republic",
        "Lebanon",
        "Liberia",
        "Libyan Arab Jamahiriya",
        "Saint Lucia",
        "Liechtenstein",
        "Sri Lanka",
        "Lesotho",
        "Lithuania",
        "Luxembourg",
        "Latvia",
        "Macao",
        "Saint Martin (French part)",
        "Morocco",
        "Monaco",
        "Moldova, Republic of",
        "Madagascar",
        "Maldives",
        "Mexico",
        "Marshall Islands",
        "Macedonia, the former Yugoslav Republic of",
        "Mali",
        "Malta",
        "Myanmar",
        "Montenegro",
        "Mongolia",
        "Northern Mariana Islands",
        "Mozambique",
        "Mauritania",
        "Montserrat",
        "Martinique",
        "Mauritius",
        "Malawi",
        "Malaysia",
        "Mayotte",
        "Namibia",
        "New Caledonia",
        "Niger",
        "Norfolk Island",
        "Nigeria",
        "Nicaragua",
        "Niue",
        "Netherlands",
        "Norway",
        "Nepal",
        "Nauru",
        "New Zealand",
        "Oman",
        "Pakistan",
        "Panama",
        "Pitcairn",
        "Peru",
        "Philippines",
        "Palau",
        "Papua New Guinea",
        "Poland",
        "Puerto Rico",
        "Korea, Democratic People's Republic of",
        "Portugal",
        "Paraguay",
        "Palestinian Territory, Occupied",
        "French Polynesia",
        "Qatar",
        "Réunion",
        "Romania",
        "Russian Federation",
        "Rwanda",
        "Saudi Arabia",
        "Sudan",
        "Senegal",
        "Singapore",
        "South Georgia and the South Sandwich Islands",
        "Saint Helena, Ascension and Tristan da Cunha",
        "Svalbard and Jan Mayen",
        "Solomon Islands",
        "Sierra Leone",
        "El Salvador",
        "San Marino",
        "Somalia",
        "Saint Pierre and Miquelon",
        "Serbia",
        "Sao Tome and Principe",
        "Suriname",
        "Slovakia",
        "Slovenia",
        "Sweden",
        "Swaziland",
        "Seychelles",
        "Syrian Arab Republic",
        "Turks and Caicos Islands",
        "Chad",
        "Togo",
        "Thailand",
        "Tajikistan",
        "Tokelau",
        "Turkmenistan",
        "Timor-Leste",
        "Tonga",
        "Trinidad and Tobago",
        "Tunisia",
        "Turkey",
        "Tuvalu",
        "Taiwan, Province of China",
        "Tanzania, United Republic of",
        "Uganda",
        "Ukraine",
        "United States Minor Outlying Islands",
        "Uruguay",
        "United States",
        "Uzbekistan",
        "Holy See (Vatican City State)",
        "Saint Vincent and the Grenadines",
        "Venezuela, Bolivarian Republic of",
        "Virgin Islands, British",
        "Virgin Islands, U.S.",
        "Viet Nam",
        "Vanuatu",
        "Wallis and Futuna",
        "Samoa",
        "Yemen",
        "South Africa",
        "Zambia",
        "Zimbabwe",
    ];
    const COUNTRY_CODE: &'static [&'static str] = &[
        "ABW", "AFG", "AGO", "AIA", "ALA", "ALB", "AND", "ANT", "ARE", "ARG", "ARM", "ASM", "ATA",
        "ATF", "ATG", "AUS", "AUT", "AZE", "BDI", "BEL", "BEN", "BFA", "BGD", "BGR", "BHR", "BHS",
        "BIH", "BLM", "BLR", "BLZ", "BMU", "BOL", "BRA", "BRB", "BRN", "BTN", "BVT", "BWA", "CAF",
        "CAN", "CCK", "CHE", "CHL", "CHN", "CIV", "CMR", "COD", "COG", "COK", "COL", "COM", "CPV",
        "CRI", "CUB", "CXR", "CYM", "CYP", "CZE", "DEU", "DJI", "DMA", "DNK", "DOM", "DZA", "ECU",
        "EGY", "ERI", "ESH", "ESP", "EST", "ETH", "FIN", "FJI", "FLK", "FRA", "FRO", "FSM", "GAB",
        "GBR", "GEO", "GGY", "GHA", "GIB", "GIN", "GLP", "GMB", "GNB", "GNQ", "GRC", "GRD", "GRL",
        "GTM", "GUF", "GUM", "GUY", "HKG", "HMD", "HND", "HRV", "HTI", "HUN", "IDN", "IMN", "IND",
        "IOT", "IRL", "IRN", "IRQ", "ISL", "ISR", "ITA", "JAM", "JEY", "JOR", "JPN", "KAZ", "KEN",
        "KGZ", "KHM", "KIR", "KNA", "KOR", "KWT", "LAO", "LBN", "LBR", "LBY", "LCA", "LIE", "LKA",
        "LSO", "LTU", "LUX", "LVA", "MAC", "MAF", "MAR", "MCO", "MDA", "MDG", "MDV", "MEX", "MHL",
        "MKD", "MLI", "MLT", "MMR", "MNE", "MNG", "MNP", "MOZ", "MRT", "MSR", "MTQ", "MUS", "MWI",
        "MYS", "MYT", "NAM", "NCL", "NER", "NFK", "NGA", "NIC", "NIU", "NLD", "NOR", "NPL", "NRU",
        "NZL", "OMN", "PAK", "PAN", "PCN", "PER", "PHL", "PLW", "PNG", "POL", "PRI", "PRK", "PRT",
        "PRY", "PSE", "PYF", "QAT", "REU", "ROU", "RUS", "RWA", "SAU", "SDN", "SEN", "SGP", "SGS",
        "SHN", "SJM", "SLB", "SLE", "SLV", "SMR", "SOM", "SPM", "SRB", "STP", "SUR", "SVK", "SVN",
        "SWE", "SWZ", "SYC", "SYR", "TCA", "TCD", "TGO", "THA", "TJK", "TKL", "TKM", "TLS", "TON",
        "TTO", "TUN", "TUR", "TUV", "TWN", "TZA", "UGA", "UKR", "UMI", "URY", "USA", "UZB", "VAT",
        "VCT", "VEN", "VGB", "VIR", "VNM", "VUT", "WLF", "WSM", "YEM", "ZAF", "ZMB", "ZWE",
    ];
    const TIME_ZONE: &'static [&'static str] = &[
        "Etc/GMT+12",
        "Etc/GMT+11",
        "Pacific/Honolulu",
        "America/Anchorage",
        "America/Santa_Isabel",
        "America/Los_Angeles",
        "America/Chihuahua",
        "America/Phoenix",
        "America/Denver",
        "America/Guatemala",
        "America/Chicago",
        "America/Regina",
        "America/Mexico_City",
        "America/Bogota",
        "America/Indiana/Indianapolis",
        "America/New_York",
        "America/Caracas",
        "America/Halifax",
        "America/Asuncion",
        "America/La_Paz",
        "America/Cuiaba",
        "America/Santiago",
        "America/St_Johns",
        "America/Sao_Paulo",
        "America/Godthab",
        "America/Cayenne",
        "America/Argentina/Buenos_Aires",
        "America/Montevideo",
        "Etc/GMT+2",
        "Atlantic/Cape_Verde",
        "Atlantic/Azores",
        "Africa/Casablanca",
        "Atlantic/Reykjavik",
        "Europe/London",
        "Etc/GMT",
        "Europe/Berlin",
        "Europe/Paris",
        "Africa/Lagos",
        "Europe/Budapest",
        "Europe/Warsaw",
        "Africa/Windhoek",
        "Europe/Istanbul",
        "Europe/Kiev",
        "Africa/Cairo",
        "Asia/Damascus",
        "Asia/Amman",
        "Africa/Johannesburg",
        "Asia/Jerusalem",
        "Asia/Beirut",
        "Asia/Baghdad",
        "Europe/Minsk",
        "Asia/Riyadh",
        "Africa/Nairobi",
        "Asia/Tehran",
        "Europe/Moscow",
        "Asia/Tbilisi",
        "Asia/Yerevan",
        "Asia/Dubai",
        "Asia/Baku",
        "Indian/Mauritius",
        "Asia/Kabul",
        "Asia/Tashkent",
        "Asia/Karachi",
        "Asia/Colombo",
        "Asia/Kolkata",
        "Asia/Kathmandu",
        "Asia/Almaty",
        "Asia/Dhaka",
        "Asia/Yekaterinburg",
        "Asia/Yangon",
        "Asia/Bangkok",
        "Asia/Novosibirsk",
        "Asia/Krasnoyarsk",
        "Asia/Ulaanbaatar",
        "Asia/Shanghai",
        "Australia/Perth",
        "Asia/Singapore",
        "Asia/Taipei",
        "Asia/Irkutsk",
        "Asia/Seoul",
        "Asia/Tokyo",
        "Australia/Darwin",
        "Australia/Adelaide",
        "Australia/Hobart",
        "Asia/Yakutsk",
        "Australia/Brisbane",
        "Pacific/Port_Moresby",
        "Australia/Sydney",
        "Asia/Vladivostok",
        "Pacific/Guadalcanal",
        "Etc/GMT-12",
        "Pacific/Fiji",
        "Asia/Magadan",
        "Pacific/Auckland",
        "Pacific/Tongatapu",
        "Pacific/Apia",
    ];
    const BUILDING: &'static [&'static str];
    fn build_address(street: &str, city: &str, state: &str) -> String;
    fn gen_zip_code<R: Rng>(rnd: &mut R, hyphen: bool) -> String;
    fn gen_domestic_phone_number<R: Rng>(rnd: &mut R, hyphen: bool) -> String;

    // FileSystem
    const EXTENSION: &'static [&'static str] = &[
        "htm", "html", "shtml", "mht", "xml", "xhtml", "xht", "txt", "asc", "sjis", "css", "xsl",
        "js", "pl", "pm", "cgi", "asp", "bat", "sh", "php", "tcl", "vbs", "gif", "jpg", "jpeg",
        "jpe", "jfif", "png", "bmp", "dib", "rle", "ico", "ai", "art", "cam", "cdr", "cgm", "cmp",
        "dpx", "fal", "q0", "fpx", "j6i", "mac", "mag", "maki", "mng", "pcd", "pct", "pic", "pict",
        "pcx", "pmp", "pnm", "psd", "ras", "sj1", "tif", "tiff", "nsk", "tga", "wmf", "wpg", "xbm",
        "xpm", "mp3", "mid", "midi", "wav", "aif", "aiff", "aifc", "au", "snd", "mov", "qt", "mpg",
        "mpeg", "wm", "wma", "wmv", "asf", "wax", "wvx", "asx", "ra", "rv", "rm", "ram", "rmm",
        "rpm", "swf", "avi", "dvr-ms", "scr", "smi", "smil", "vdo", "vrml", "wrl", "lzh", "zip",
        "cab", "tar", "gz", "tgz", "tar.gz", "hqx", "sit", "Z", "uu", "pdf", "doc", "xls", "ppt",
        "pps", "dcr", "dir", "dxr", "dwt", "fla", "jxw", "ppd", "ps", "eps", "ai", "rtf", "wri",
        "class", "jar", "java", "c", "cpp", "h", "obj", "hlp", "chm", "man", "exe", "dll", "com",
        "ocx", "sys", "a", "so", "fon", "ttf", "ttc", "ani", "cur", "db", "inf", "ini", "reg",
        "scr", "url", "csv", "cnf", "conf", "cf", "log", "dat", "bak", "bin", "dic", "old", "org",
        "tmp",
    ];
}

impl<D: Data> Rand for D {}
