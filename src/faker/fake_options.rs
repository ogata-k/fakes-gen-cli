use crate::date_time_format::{DEFAULT_DATE_FORMAT, DEFAULT_DATE_TIME_FORMAT, DEFAULT_TIME_FORMAT};
use crate::faker::category::Category;
use crate::helper::{not_string_formatted, string_formatted};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FakeOption {
    // Fixed Value
    FixedString(String),
    FixedNotString(String),

    // Random select from user value
    SelectString(Vec<String>),
    SelectNotString(Vec<String>),

    // Lorem
    Word,
    Words(usize, usize),
    Sentence,
    Sentences(usize, usize),
    Paragraph,
    Paragraphs(usize, usize),

    // Name(use furigana)
    // generate "name":"furigana"
    FirstName(bool),
    FirstNameFurigana,
    LastName(bool),
    LastNameFurigana,
    FullName(bool),
    FullNameFurigana,

    // Primitive
    Integer,
    IntegerRange(isize, isize),
    Float,
    FloatRange(isize, isize),
    Ascii(usize, usize),
    Boolean,

    // Internet
    Email,
    UserName,
    Password(usize, usize),
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
    ZipCode(bool),
    // use hyphen?
    DomesticPhoneNumber(bool),
    // use hyphen?
    Latitude,
    Longitude,

    // Date Time
    // format-str: https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers
    // String is format. default is "%Y-%m-%d %H:%I:%M"'s sub-format.
    // But, When Time(Date, DateTime), use only Time(Date, Time/Date)-formatter.
    // ex. "219-11-02 21:09:31"
    Time(String),
    // now - 100year ~ now
    Date(String),
    // now - 100year ~ now
    DateTime(String),

    // FileSystem
    FileName,
    Extension,
}

impl std::fmt::Display for FakeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use FakeOption::*;
        let cat: Category = self.category();
        let s: String = match self {
            FixedString(s) => format!("{}.String(target: \"{}\")", cat, s),
            FixedNotString(s) => format!("{}.NotString(target: \"{}\")", cat, s),
            SelectString(list) => format!("{}.SelectString(list: {:?})", cat, list),
            SelectNotString(list) => format!("{}.SelectNotString(list: {:?})", cat, list),
            Word => format!("{}.Word", cat),
            Words(from, to) => format!("{}.Words(count: {}<=n<={})", cat, from, to),
            Sentence => format!("{}.Sentence", cat),
            Sentences(from, to) => format!("{}.Sentences(count: {}<=n<={})", cat, from, to),
            Paragraph => format!("{}.Paragraph", cat),
            Paragraphs(from, to) => format!("{}.Paragraphs(count: {}<=n<={})", cat, from, to),
            FirstName(furigana) => format!("{}.FirstName(with_furigana: {})", cat, furigana),
            FirstNameFurigana => format!("{}.FirstNameFurigana", cat),
            LastName(furigana) => format!("{}.LastName(with_furigana: {})", cat, furigana),
            LastNameFurigana => format!("{}.LastNameFurigana", cat),
            FullName(furigana) => format!("{}.FullName(with_furigana: {})", cat, furigana),
            FullNameFurigana => format!("{}.FullNameFurigana", cat),
            Integer => format!("{}.Integer", cat),
            IntegerRange(from, to) => format!("{}.Integer(range: {}<=n<={})", cat, from, to),
            Float => format!("{}.Float", cat),
            FloatRange(from, to) => format!("{}.Float(range: {}<=n<={})", cat, from, to),
            Ascii(from, to) => format!("{}.Ascii(length: {}<=n<={})", cat, from, to),
            Boolean => format!("{}.Boolean", cat),
            Email => format!("{}.Email", cat),
            UserName => format!("{}.UserName", cat),
            Password(from, to) => format!("{}.Password(length: {}<=n<={})", cat, from, to),
            CreditCard => format!("{}.CreditCard", cat),
            URL => format!("{}.URL", cat),
            IPv4 => format!("{}.IPv4", cat),
            IPv6 => format!("{}.IPv6", cat),
            RGB => format!("{}.RGB", cat),
            RGBA => format!("{}.RGBA", cat),
            UserAgent => format!("{}.UserAgent", cat),
            StatusCode => format!("{}.StatusCode", cat),
            CompanySuffix => format!("{}.CompanySuffix", cat),
            CompanyName => format!("{}.CompanyName", cat),
            Industry => format!("{}.Industry", cat),
            Building => format!("{}.Building", cat),
            StreetName => format!("{}.StreetName", cat),
            CityName => format!("{}.CityName", cat),
            StateName => format!("{}.StateName", cat),
            CountryCode => format!("{}.CountryCode", cat),
            CountryName => format!("{}.CountryName", cat),
            TimeZone => format!("{}.TimeZone", cat),
            Address => format!("{}.Address", cat),
            ZipCode(hyphen) => format!("{}.ZipCode(use_hyphen: {})", cat, hyphen),
            DomesticPhoneNumber(hyphen) => {
                format!("{}.DomesticPhoneNumber(use_hyphen: {})", cat, hyphen)
            }
            Latitude => format!("{}.Latitude", cat),
            Longitude => format!("{}.Longitude", cat),
            Time(format) => format!("{}.Time(format: {})", cat, format),
            Date(format) => format!("{}.Date(format: {})", cat, format),
            DateTime(format) => format!("{}.DateTime(format: {})", cat, format),
            FileName => format!("{}.FileName", cat),
            Extension => format!("{}.Extension", cat),
        };

        write!(f, "{}", s)
    }
}

impl FakeOption {
    pub fn category(&self) -> Category {
        use FakeOption::*;
        match self {
            FixedString(_) | FixedNotString(_) => Category::Fixed,
            SelectString(_) | SelectNotString(_) => Category::Select,
            Word | Words(_, _) | Sentence | Sentences(_, _) | Paragraph | Paragraphs(_, _) => {
                Category::Lorem
            }
            FirstName(_) | FirstNameFurigana | LastName(_) | LastNameFurigana | FullName(_)
            | FullNameFurigana => Category::Name,
            Integer | IntegerRange(_, _) | Float | FloatRange(_, _) | Ascii(_, _) | Boolean => {
                Category::Primitive
            }
            Email
            | UserName
            | Password(_, _)
            | CreditCard
            | URL
            | IPv4
            | IPv6
            | RGB
            | RGBA
            | UserAgent
            | StatusCode => Category::Internet,
            CompanySuffix | CompanyName | Industry => Category::Company,
            Building
            | StreetName
            | CityName
            | StateName
            | CountryCode
            | CountryName
            | TimeZone
            | Address
            | ZipCode(_)
            | DomesticPhoneNumber(_)
            | Latitude
            | Longitude => Category::Address,
            Time(_) | Date(_) | DateTime(_) => Category::DateTime,
            FileName | Extension => Category::FileSystem,
        }
    }

    pub fn example_all_options() -> Vec<Self> {
        use FakeOption::*;
        vec![
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
            FirstNameFurigana,
            LastName(true),
            LastName(false),
            LastNameFurigana,
            FullName(true),
            FullName(false),
            FullNameFurigana,
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
        ]
    }

    fn is_string_type(&self) -> bool {
        use FakeOption::*;
        match self {
            FixedNotString(_)
            | SelectNotString(_)
            | Integer
            | IntegerRange(_, _)
            | Float
            | FloatRange(_, _)
            | Boolean => false,
            _ => true,
        }
    }

    pub fn with_format(&self, data: &str) -> String {
        if self.is_string_type() {
            string_formatted(data)
        } else {
            not_string_formatted(data)
        }
    }

    pub fn is_person_name(&self) -> bool {
        use FakeOption::*;
        match self {
            FirstName(_) | FirstNameFurigana | LastName(_) | LastNameFurigana | FullName(_)
            | FullNameFurigana => true,
            _ => false,
        }
    }
}
