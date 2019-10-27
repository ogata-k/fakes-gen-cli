use std::str::FromStr;
use crate::error::OptionParseError;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Option {
    // Lorem
    Word,
    Words(usize, usize),
    Sentence,
    Sentences(usize, usize),
    Paragraph,
    Paragraphs(usize, usize),

    // Name
    FirstName,
    LastName,
    Title,
    Suffix,
    Name,
    NameWithTitle,

    // Number
    Integer,
    IntegerRange(isize, isize),
    Double,
    DoubleRange(isize, isize),

    // Boolean
    Boolean,

    // Internet
    FreeEmailProvider,
    DomainSuffix,
    FreeEmail,
    SafeEmail,
    Username,
    Password(usize, usize),
    IPv4,
    IPv6,
    IP,
    Color,
    UserAgent,

    // HTTP
    RfcStatusCode,
    ValidStatusCode,

    // Company
    CompanySuffix,
    CompanyName,
    Buzzword,
    BuzzwordMiddle,
    BuzzwordTail,
    CatchPhase,
    BsVerb,
    BsAdj,
    BsNoun,
    Bs,
    Profession,
    Industry,

    // Address
    CityPrefix,
    CitySuffix,
    CityName,
    CountryName,
    CountryCode,
    StreetSuffix,
    StreetName,
    TimeZone,
    StateName,
    StateAbbr,
    SecondaryAddressType,
    SecondaryAddress,
    ZipCode,
    PostCode,
    BuildingNumber,
    Latitude,
    Longitude,

    // Phone Number
    PhoneNumber,
    CellNumber,

    // Date / Time
    Time,
    Date,
    DateTime,
    // from day + first isize, to day + second isize based on now
    DateTimeBetween(isize, isize),

    // Filesystem
    FilePath,
    FileName,
    FileExtension,
    DirPath,
}

impl FromStr for Option {
    type Err = OptionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Option {
    pub fn has_range(&self) -> bool{
        unimplemented!()
    }

    pub fn validate_range(&self) -> bool {
        unimplemented!()
    }

    pub fn is_lang_data(&self) -> bool {
        unimplemented!()
    }
}