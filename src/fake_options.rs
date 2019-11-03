#[derive(Eq, PartialEq, Debug)]
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
    LastName(bool),
    FullName(bool),

    // Primitive
    Integer,
    IntegerRange(isize, isize),
    Float,
    FloatRange(isize, isize),
    Ascii(usize, usize),
    Boolean,

    // Internet
    Email,
    Username,
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
    ZipCode(bool), // use hyphen?
    DomesticPhoneNumber(bool), // use hyphen?
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

    // Filesystem
    FileName,
    FileExtension,
}

impl FakeOption {
    // category() -> Category
}