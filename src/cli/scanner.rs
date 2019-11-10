use crate::cli::helper::vec_to_str;

use failure::Fail;
use failure::_core::fmt::{Display, Error, Formatter};
use failure::_core::str::FromStr;
use fakes_gen::date_time_format::{DEFAULT_DATE_TIME_FORMAT, DEFAULT_TIME_FORMAT};
use fakes_gen::faker::category::Category;
use fakes_gen::faker::fake_options::FakeOption;
use regex::{Captures, Regex};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Scanner {
    input: String,
}

impl Scanner {
    // option name
    const FIXED_STRING: &'static str = "String";
    const FIXED_NOT_STRING: &'static str = "NotString";
    const SELECT_STRING: &'static str = "String";
    const SELECT_NOT_STRING: &'static str = "NotString";
    const LOREM_WORD: &'static str = "Word";
    const LOREM_SENTENCE: &'static str = "Sentence";
    const LOREM_PARAGRAPH: &'static str = "Paragraph";
    const NAME_FIRST_NAME: &'static str = "FirstName";
    const NAME_FIRST_NAME_FURIGANA: &'static str = "FirstNameFurigana";
    const NAME_LAST_NAME: &'static str = "LastName";
    const NAME_LAST_NAME_FURIGANA: &'static str = "LastNameFurigana";
    const NAME_FULL_NAME: &'static str = "FullName";
    const NAME_FULL_NAME_FURIGANA: &'static str = "FullNameFurigana";
    const PRIMITIVE_INTEGER: &'static str = "Int";
    const PRIMITIVE_FLOAT: &'static str = "Float";
    const PRIMITIVE_ASCII: &'static str = "Ascii";
    const PRIMITIVE_BOOL: &'static str = "Bool";
    const INTERNET_EMAIL: &'static str = "Email";
    const INTERNET_USER_NAME: &'static str = "UserName";
    const INTERNET_PASSWORD: &'static str = "Password";
    const INTERNET_CREDIT_CARD: &'static str = "CreditCard";
    const INTERNET_URL: &'static str = "URL";
    const INTERNET_IPV4: &'static str = "IPv4";
    const INTERNET_IPV6: &'static str = "IPv6";
    const INTERNET_RGB: &'static str = "RGB";
    const INTERNET_RGBA: &'static str = "RGBA";
    const INTERNET_USER_AGENT: &'static str = "UserAgent";
    const INTERNET_STATUS_CODE: &'static str = "StatusCode";
    const COMPANY_SUFFIX: &'static str = "Suffix";
    const COMPANY_NAME: &'static str = "Name";
    const COMPANY_INDUSTRY: &'static str = "Industry";
    const ADDRESS_BUILDING: &'static str = "Building";
    const ADDRESS_STREET_NAME: &'static str = "Street";
    const ADDRESS_CITY_NAME: &'static str = "City";
    const ADDRESS_STATE_NAME: &'static str = "State";
    const ADDRESS_COUNTRY_CODE: &'static str = "CountryCode";
    const ADDRESS_COUNTRY_NAME: &'static str = "CountryName";
    const ADDRESS_TIMEZONE: &'static str = "TimeZone";
    const ADDRESS_ADDRESS: &'static str = "Address";
    const ADDRESS_ZIP_CODE: &'static str = "ZipCode";
    const ADDRESS_DOMESTIC_PHONE_NUMBER: &'static str = "Phone";
    const ADDRESS_LATITUDE: &'static str = "Latitude";
    const ADDRESS_LONGITUDE: &'static str = "Longitude";
    const DATE_TIME_TIME: &'static str = "Time";
    const DATE_TIME_DATE: &'static str = "Date";
    const DATE_TIME_DATE_TIME: &'static str = "DateTime";
    const FILE_SYSTEM_FILE_NAME: &'static str = "FileName";
    const FILE_SYSTEM_EXTENSION: &'static str = "Extension";
    pub fn readable_options(category: Category) -> Vec<String> {
        use Category::*;
        match category {
            Fixed => Self::readable_fixed_options(),
            Select => Self::readable_select_options(),
            Lorem => Self::readable_lorem_options(),
            Name => Self::readable_name_options(),
            Primitive => Self::readable_primitive_options(),
            Internet => Self::readable_internet_options(),
            Company => Self::readable_company_options(),
            Address => Self::readable_address_options(),
            DateTime => Self::readable_datetime_options(),
            FileSystem => Self::readable_filesystem_options(),
        }
    }

    pub fn readable_fixed_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format(
            Category::Fixed,
            Self::FIXED_STRING,
            Self::STRING_VAR,
        ));
        stack.push(Self::option_format(
            Category::Fixed,
            Self::FIXED_NOT_STRING,
            Self::STRING_VAR,
        ));
        return stack;
    }

    pub fn readable_select_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format(
            Category::Select,
            Self::SELECT_STRING,
            Self::STRING_LIST_VAR,
        ));
        stack.push(Self::option_format(
            Category::Select,
            Self::SELECT_NOT_STRING,
            Self::STRING_LIST_VAR,
        ));
        return stack;
    }

    pub fn readable_lorem_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Lorem,
            Self::LOREM_WORD,
            Self::UNSIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Lorem,
            Self::LOREM_SENTENCE,
            Self::UNSIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Lorem,
            Self::LOREM_PARAGRAPH,
            Self::UNSIGNED_INTEGER_RANGE_VAR,
        ));
        return stack;
    }

    pub fn readable_name_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_FIRST_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_FIRST_NAME_FURIGANA,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_LAST_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_LAST_NAME_FURIGANA,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_FULL_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Name,
            Self::NAME_FULL_NAME_FURIGANA,
        ));
        return stack;
    }

    pub fn readable_primitive_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Primitive,
            Self::PRIMITIVE_INTEGER,
            Self::SIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Primitive,
            Self::PRIMITIVE_FLOAT,
            Self::SIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Primitive,
            Self::PRIMITIVE_ASCII,
            Self::SIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Primitive,
            Self::PRIMITIVE_BOOL,
        ));
        return stack;
    }

    pub fn readable_internet_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_EMAIL,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_USER_NAME,
        ));
        stack.push(Self::option_format(
            Category::Internet,
            Self::INTERNET_PASSWORD,
            Self::UNSIGNED_INTEGER_RANGE_VAR,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_CREDIT_CARD,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_URL,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_IPV4,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_IPV6,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_RGB,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_RGBA,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_USER_AGENT,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Internet,
            Self::INTERNET_STATUS_CODE,
        ));
        return stack;
    }

    pub fn readable_company_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_arg(
            Category::Company,
            Self::COMPANY_SUFFIX,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Company,
            Self::COMPANY_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Company,
            Self::COMPANY_INDUSTRY,
        ));
        return stack;
    }

    pub fn readable_address_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_BUILDING,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_STREET_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_CITY_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_STATE_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_COUNTRY_CODE,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_COUNTRY_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_TIMEZONE,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_ADDRESS,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Address,
            Self::ADDRESS_ZIP_CODE,
            Self::BOOL_VAR,
        ));
        stack.push(Self::option_format_has_no_necessary_arg(
            Category::Address,
            Self::ADDRESS_DOMESTIC_PHONE_NUMBER,
            Self::BOOL_VAR,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_LATITUDE,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::Address,
            Self::ADDRESS_LONGITUDE,
        ));
        return stack;
    }

    pub fn readable_datetime_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format(
            Category::DateTime,
            Self::DATE_TIME_TIME,
            Self::FORMAT_STRING_VAR,
        ));
        stack.push(Self::option_format(
            Category::DateTime,
            Self::DATE_TIME_DATE,
            Self::FORMAT_STRING_VAR,
        ));
        stack.push(Self::option_format(
            Category::DateTime,
            Self::DATE_TIME_DATE_TIME,
            Self::FORMAT_STRING_VAR,
        ));
        return stack;
    }

    pub fn readable_filesystem_options() -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(Self::option_format_has_no_arg(
            Category::FileSystem,
            Self::FILE_SYSTEM_FILE_NAME,
        ));
        stack.push(Self::option_format_has_no_arg(
            Category::FileSystem,
            Self::FILE_SYSTEM_EXTENSION,
        ));
        return stack;
    }

    // variable
    const OPTION_VAR: &'static str = "<option>";
    const CATEGORY_VAR: &'static str = "<category>";
    const OPTION_NAME_VAR: &'static str = "<option_name>";
    const COLUMN_NAME_VAR: &'static str = "<column_name>";
    const SUB_OPTION_VAR: &'static str = "<sub_option>";
    const STRING_VAR: &'static str = "<string>";
    const STRING_LIST_VAR: &'static str = "<string_list>";
    const UNSIGNED_INTEGER_RANGE_VAR: &'static str = "<unsigned_integer_range>";
    const SIGNED_INTEGER_RANGE_VAR: &'static str = "<signed_integer_range>";
    const UNSIGNED_INTEGER_VAR: &'static str = "<unsigned_integer>";
    const BOOL_VAR: &'static str = "<bool>";
    const FORMAT_STRING_VAR: &'static str = "<format_string>";

    // value
    const OPTION_FORMAT: &'static str =
        "<category>.<option_name>\\(<column_name>(#<sub_option>)?\\)";
    const CATEGORY_FORMAT: &'static str = "[A-Z][0-9a-zA-Z]*";
    const OPTION_NAME_FORMAT: &'static str = "[A-Z][0-9a-zA-Z]*";
    const SUB_OPTION_FORMAT: &'static str =
        "<string>|<string_list>|<unsigned_min_max>|<signed_min_max>|<boolean>|<format_string>";
    const STRING_FORMAT: &'static str = "((\".*\")|[.[^\\ ]]*)";
    const STRING_LIST_FORMAT: &'static str = "\\[<string>(#<string>)*\\]";
    const UNSIGNED_INTEGER_RANGE_FORMAT: &'static str = "<unsigned_integer>#<unsigned_integer>";
    const SIGNED_INTEGER_RANGE_FORMAT: &'static str = "-?<unsigned_integer>#-?<unsigned_integer>";
    const UNSIGNED_INTEGER_FORMAT: &'static str = "[0-9][1-9]*";
    const BOOL_FORMAT: &'static str = "(true)|(false)";

    // format pair
    const OPTION: (&'static str, &'static str) = (Scanner::OPTION_VAR, Scanner::OPTION_FORMAT);
    const CATEGORY: (&'static str, &'static str) =
        (Scanner::CATEGORY_VAR, Scanner::CATEGORY_FORMAT);
    const OPTION_NAME: (&'static str, &'static str) =
        (Scanner::OPTION_NAME_VAR, Scanner::OPTION_NAME_FORMAT);
    const COLUMN_NAME: (&'static str, &'static str) =
        (Scanner::COLUMN_NAME_VAR, Scanner::STRING_VAR);
    const SUB_OPTION: (&'static str, &'static str) =
        (Scanner::SUB_OPTION_VAR, Scanner::SUB_OPTION_FORMAT);
    const STRING: (&'static str, &'static str) = (Scanner::STRING_VAR, Scanner::STRING_FORMAT);
    const STRING_LIST: (&'static str, &'static str) =
        (Scanner::STRING_LIST_VAR, Scanner::STRING_LIST_FORMAT);
    const UNSIGNED_MIN_MAX: (&'static str, &'static str) = (
        Scanner::UNSIGNED_INTEGER_RANGE_VAR,
        Scanner::UNSIGNED_INTEGER_RANGE_FORMAT,
    );
    const SIGNED_MIN_MAX: (&'static str, &'static str) = (
        Scanner::SIGNED_INTEGER_RANGE_VAR,
        Scanner::SIGNED_INTEGER_RANGE_FORMAT,
    );
    const UNSIGNED_INT: (&'static str, &'static str) = (
        Scanner::UNSIGNED_INTEGER_VAR,
        Scanner::UNSIGNED_INTEGER_FORMAT,
    );
    const BOOL: (&'static str, &'static str) = (Scanner::BOOL_VAR, Scanner::BOOL_FORMAT);
    const FORMAT_STRING: (&'static str, &'static str) =
        (Scanner::FORMAT_STRING_VAR, Scanner::STRING_VAR);

    fn all_format_pair() -> Vec<(&'static str, &'static str)> {
        [
            Self::OPTION,
            Self::CATEGORY,
            Self::OPTION_NAME,
            Self::COLUMN_NAME,
            Self::SUB_OPTION,
            Self::STRING,
            Self::STRING_LIST,
            Self::UNSIGNED_MIN_MAX,
            Self::SIGNED_MIN_MAX,
            Self::UNSIGNED_INT,
            Self::BOOL,
            Self::FORMAT_STRING,
        ]
        .to_vec()
    }

    // format pair
    pub fn all_format_bnf() -> Vec<String> {
        Self::all_format_pair()
            .iter()
            .map(|(var, format)| format!("{} := {}", var, format))
            .collect()
    }

    fn option_format(category: Category, option_name: &str, arg_format: &str) -> String {
        format!(
            "{}.{}\\({}#{}\\)",
            category.to_string(),
            option_name,
            Self::COLUMN_NAME_VAR,
            arg_format
        )
    }

    fn option_format_has_no_necessary_arg(
        category: Category,
        option_name: &str,
        arg_format: &str,
    ) -> String {
        format!(
            "{}.{}\\({}(#{})?\\)",
            category.to_string(),
            option_name,
            Self::COLUMN_NAME_VAR,
            arg_format
        )
    }

    fn option_format_has_no_arg(category: Category, option_name: &str) -> String {
        format!(
            "{}.{}\\({}\\)",
            category.to_string(),
            option_name,
            Self::COLUMN_NAME_VAR
        )
    }

    pub fn new(target: &str) -> Self {
        Scanner {
            input: target.to_string(),
        }
    }

    // ---
    // helper
    // ---
    fn split(target: &str) -> Vec<String> {
        let s_list: Vec<String> = target
            .split("#")
            .filter_map(|s| if s == "" { None } else { Some(s.to_string()) })
            .collect();
        return s_list;
    }

    // ---
    // primitive parser
    // ---
    fn parse_none(subs: &[String]) -> Result<(), ScannerError> {
        if subs.is_empty() {
            return Ok(());
        } else {
            return Err(ScannerError::UnknownCharacters(subs.to_vec()));
        }
    }

    fn parse_string(subs: &[String]) -> Result<String, ScannerError> {
        if subs.len() != 1 {
            return Err(ScannerError::UnknownStringFormat(subs.to_vec()));
        } else {
            return Ok(subs[0].to_string());
        }
    }

    fn parse_string_list(subs: &[String]) -> Result<Vec<String>, ScannerError> {
        return Ok(subs.to_vec());
    }

    fn parse_int_range<T: FromStr + Ord + ToString>(
        subs: &[String],
    ) -> Result<(T, T), ScannerError> {
        if subs.len() != 2 {
            return Err(ScannerError::UnknownIntegerListFormat(subs.to_vec()));
        }
        let from = T::from_str(&subs[0]);
        let to = T::from_str(&subs[1]);
        if from.is_err() || to.is_err() {
            return Err(ScannerError::UnknownIntegerListFormat(subs.to_vec()));
        }
        let from = from.ok().unwrap();
        let to = to.ok().unwrap();
        if !(from <= to) {
            return Err(ScannerError::RangeErr(from.to_string(), to.to_string()));
        }
        return Ok((from, to));
    }

    fn parse_bool(subs: &[String]) -> Result<bool, ScannerError> {
        let s = Self::parse_string(subs)?;
        let b = bool::from_str(&s);
        if b.is_err() {
            return Err(ScannerError::UnknownBooleanFormat(subs.to_vec()));
        } else {
            return Ok(b.unwrap());
        }
    }

    // ---
    // combination parser
    // ---
    fn parse_category(&self, target: &str) -> Result<Category, ScannerError> {
        let target_string = target.to_string();
        if target_string == Category::Fixed.to_string() {
            return Ok(Category::Fixed);
        }
        if target_string == Category::Select.to_string() {
            return Ok(Category::Select);
        }
        if target_string == Category::Lorem.to_string() {
            return Ok(Category::Lorem);
        }
        if target_string == Category::Name.to_string() {
            return Ok(Category::Name);
        }
        if target_string == Category::Primitive.to_string() {
            return Ok(Category::Primitive);
        }
        if target_string == Category::Internet.to_string() {
            return Ok(Category::Internet);
        }
        if target_string == Category::Company.to_string() {
            return Ok(Category::Company);
        }
        if target_string == Category::Address.to_string() {
            return Ok(Category::Address);
        }
        if target_string == Category::DateTime.to_string() {
            return Ok(Category::DateTime);
        }
        if target_string == Category::FileSystem.to_string() {
            return Ok(Category::FileSystem);
        }
        return Err(ScannerError::UnknownCategory(target_string));
    }

    fn parse_fixed(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::FIXED_STRING {
            return Ok(FakeOption::FixedString(Self::parse_string(&Self::split(
                sub_option_str,
            ))?));
        }
        if option_name == Self::FIXED_NOT_STRING {
            return Ok(FakeOption::FixedNotString(Self::parse_string(
                &Self::split(sub_option_str),
            )?));
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Fixed,
        ));
    }

    fn parse_select(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::SELECT_STRING {
            return Ok(FakeOption::SelectString(Self::parse_string_list(
                &Self::split(sub_option_str),
            )?));
        }
        if option_name == Self::SELECT_NOT_STRING {
            return Ok(FakeOption::SelectNotString(Self::parse_string_list(
                &Self::split(sub_option_str),
            )?));
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Select,
        ));
    }

    fn parse_lorem(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::LOREM_WORD {
            if sub_option_str == "" {
                return Ok(FakeOption::Word);
            } else {
                let ints = Self::parse_int_range::<usize>(&Self::split(sub_option_str))?;
                return Ok(FakeOption::Words(ints.0, ints.1));
            }
        }
        if option_name == Self::LOREM_SENTENCE {
            if sub_option_str == "" {
                return Ok(FakeOption::Sentence);
            } else {
                let ints = Self::parse_int_range::<usize>(&Self::split(sub_option_str))?;
                return Ok(FakeOption::Sentences(ints.0, ints.1));
            }
        }
        if option_name == Self::LOREM_PARAGRAPH {
            if sub_option_str == "" {
                return Ok(FakeOption::Paragraph);
            } else {
                let ints = Self::parse_int_range::<usize>(&Self::split(sub_option_str))?;
                return Ok(FakeOption::Paragraphs(ints.0, ints.1));
            }
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Lorem,
        ));
    }

    fn parse_name(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::NAME_FIRST_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FirstName(false));
        }
        if option_name == Self::NAME_FIRST_NAME_FURIGANA {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FirstName(false));
        }
        if option_name == Self::NAME_LAST_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::LastName(false));
        }
        if option_name == Self::NAME_LAST_NAME_FURIGANA {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FirstName(false));
        }
        if option_name == Self::NAME_FULL_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FullName(false));
        }
        if option_name == Self::NAME_FULL_NAME_FURIGANA {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FullName(false));
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Name,
        ));
    }

    fn parse_primitive(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::PRIMITIVE_INTEGER {
            if sub_option_str == "" {
                return Ok(FakeOption::Integer);
            } else {
                let ints = Self::parse_int_range::<isize>(&Self::split(sub_option_str))?;
                return Ok(FakeOption::IntegerRange(ints.0, ints.1));
            }
        }
        if option_name == Self::PRIMITIVE_FLOAT {
            if sub_option_str == "" {
                return Ok(FakeOption::Float);
            } else {
                let ints = Self::parse_int_range::<isize>(&Self::split(sub_option_str))?;
                return Ok(FakeOption::FloatRange(ints.0, ints.1));
            }
        }
        if option_name == Self::PRIMITIVE_ASCII {
            let ints = Self::parse_int_range::<usize>(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Ascii(ints.0, ints.1));
        }
        if option_name == Self::PRIMITIVE_BOOL {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Boolean);
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Primitive,
        ));
    }

    fn parse_internet(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::INTERNET_EMAIL {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Email);
        }
        if option_name == Self::INTERNET_USER_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::UserName);
        }
        if option_name == Self::INTERNET_PASSWORD {
            let ints = Self::parse_int_range::<usize>(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Password(ints.0, ints.1));
        }
        if option_name == Self::INTERNET_CREDIT_CARD {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CreditCard);
        }
        if option_name == Self::INTERNET_URL {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::URL);
        }
        if option_name == Self::INTERNET_IPV4 {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::IPv4);
        }
        if option_name == Self::INTERNET_IPV6 {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::IPv6);
        }
        if option_name == Self::INTERNET_RGB {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::RGB);
        }
        if option_name == Self::INTERNET_RGBA {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::RGBA);
        }
        if option_name == Self::INTERNET_USER_AGENT {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::UserAgent);
        }
        if option_name == Self::INTERNET_STATUS_CODE {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::UserName);
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Internet,
        ));
    }

    fn parse_company(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::COMPANY_SUFFIX {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CompanySuffix);
        }
        if option_name == Self::COMPANY_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CompanyName);
        }
        if option_name == Self::COMPANY_INDUSTRY {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Industry);
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Company,
        ));
    }

    fn parse_address(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::ADDRESS_BUILDING {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Building);
        }
        if option_name == Self::ADDRESS_STREET_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::StreetName);
        }
        if option_name == Self::ADDRESS_CITY_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CityName);
        }
        if option_name == Self::ADDRESS_STATE_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::StateName);
        }
        if option_name == Self::ADDRESS_COUNTRY_CODE {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CountryCode);
        }
        if option_name == Self::ADDRESS_COUNTRY_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::CountryName);
        }
        if option_name == Self::ADDRESS_TIMEZONE {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::TimeZone);
        }
        if option_name == Self::ADDRESS_ADDRESS {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Address);
        }
        if option_name == Self::ADDRESS_ZIP_CODE {
            if sub_option_str == "" {
                return Ok(FakeOption::ZipCode(false));
            }
            return Ok(FakeOption::ZipCode(Self::parse_bool(&Self::split(
                sub_option_str,
            ))?));
        }
        if option_name == Self::ADDRESS_DOMESTIC_PHONE_NUMBER {
            if sub_option_str == "" {
                return Ok(FakeOption::DomesticPhoneNumber(false));
            }
            return Ok(FakeOption::DomesticPhoneNumber(Self::parse_bool(
                &Self::split(sub_option_str),
            )?));
        }
        if option_name == Self::ADDRESS_LATITUDE {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Latitude);
        }
        if option_name == Self::ADDRESS_LONGITUDE {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Longitude);
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::Address,
        ));
    }

    fn parse_datetime(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::DATE_TIME_TIME {
            if sub_option_str == "" {
                return Ok(FakeOption::Time(DEFAULT_TIME_FORMAT.to_string()));
            } else {
                return Ok(FakeOption::Time(Self::parse_string(&Self::split(
                    sub_option_str,
                ))?));
            }
        }
        if option_name == Self::DATE_TIME_DATE {
            if sub_option_str == "" {
                return Ok(FakeOption::Date(DEFAULT_TIME_FORMAT.to_string()));
            } else {
                return Ok(FakeOption::Date(Self::parse_string(&Self::split(
                    sub_option_str,
                ))?));
            }
        }
        if option_name == Self::DATE_TIME_DATE_TIME {
            if sub_option_str == "" {
                return Ok(FakeOption::DateTime(DEFAULT_DATE_TIME_FORMAT.to_string()));
            } else {
                return Ok(FakeOption::DateTime(Self::parse_string(&Self::split(
                    sub_option_str,
                ))?));
            }
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::DateTime,
        ));
    }

    fn parse_filesystem(
        &self,
        option_name: &str,
        sub_option_str: &str,
    ) -> Result<FakeOption, ScannerError> {
        if option_name == Self::FILE_SYSTEM_FILE_NAME {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::FileName);
        }
        if option_name == Self::FILE_SYSTEM_EXTENSION {
            Self::parse_none(&Self::split(sub_option_str))?;
            return Ok(FakeOption::Extension);
        }
        return Err(ScannerError::UnknownOption(
            option_name.to_string(),
            Category::FileSystem,
        ));
    }

    pub fn scan(&self) -> Result<(String, FakeOption), ScannerError> {
        let regex: Regex = Regex::new(r"^(?P<Category>[A-Z][[:alnum:]]*?)\.(?P<OptionName>[A-Z][[:alnum:]]*?)\((?P<ColumnName>.+?)(?:#(?P<SubOption>.*?))?\)$").unwrap();
        let capture = regex.captures(&self.input);
        if capture.is_none() {
            return Err(ScannerError::UnknownOptionFormat(self.input.to_string()));
        }
        let capture: Captures = capture.unwrap();
        let category = self.parse_category(capture.name("Category").unwrap().as_str())?;
        let option_name = capture.name("OptionName").unwrap().as_str();
        let column_name = capture.name("ColumnName").unwrap().as_str();
        let sub_option_str = capture.name("SubOption").map_or("", |s| s.as_str());
        let option: FakeOption = match category {
            Category::Fixed => self.parse_fixed(option_name, sub_option_str)?,
            Category::Select => self.parse_select(option_name, sub_option_str)?,
            Category::Lorem => self.parse_lorem(option_name, sub_option_str)?,
            Category::Name => self.parse_name(option_name, sub_option_str)?,
            Category::Primitive => self.parse_primitive(option_name, sub_option_str)?,
            Category::Internet => self.parse_internet(option_name, sub_option_str)?,
            Category::Company => self.parse_company(option_name, sub_option_str)?,
            Category::Address => self.parse_address(option_name, sub_option_str)?,
            Category::DateTime => self.parse_datetime(option_name, sub_option_str)?,
            Category::FileSystem => self.parse_filesystem(option_name, sub_option_str)?,
        };

        return Ok((column_name.to_string(), option));
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Fail)]
pub enum ScannerError {
    UnknownCategory(String),
    UnknownOptionFormat(String),
    UnknownOption(String, Category),
    UnknownCharacters(Vec<String>),
    UnknownStringFormat(Vec<String>),
    UnknownBooleanFormat(Vec<String>),
    UnknownStringListFormat(Vec<String>),
    UnknownIntegerListFormat(Vec<String>),
    RangeErr(String, String),
}

impl ScannerError {
    fn write_messages(f: &mut Formatter<'_>, title: &str, options: &[(&str, &str)]) {
        write!(f, "{}:", title);
        for option in options {
            write!(f, "\n\t{} := {}", option.0, option.1);
        }
    }

    fn write_unknown_format(f: &mut Formatter<'_>, target: &str) {
        writeln!(f, "Unknown format is \"{}\"", target);
    }

    fn write_unknown_format_of_vec(f: &mut Formatter<'_>, subs: &[String]) {
        writeln!(f, "Unknown format is {}", vec_to_str(subs));
    }

    pub fn write_option_format(f: &mut Formatter<'_>) {
        Self::write_messages(f, "Usable Option format", &Scanner::all_format_pair());
    }
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use ScannerError::*;
        match self {
            UnknownCategory(s) => {
                writeln!(f, "Unknown Category is {}", s);
                write!(
                    f,
                    "Usable Category is {}",
                    vec_to_str(
                        &Category::all_list()
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                    )
                );
                Ok(())
            }
            UnknownOptionFormat(s) => {
                Self::write_unknown_format(f, s);
                Self::write_option_format(f);
                Ok(())
            }
            UnknownOption(s, c) => {
                writeln!(f, "Unknown Option is \"{}\"", s);
                writeln!(f, "Usable Option's format is {}", Scanner::OPTION_FORMAT);
                write!(
                    f,
                    "And usable Option is {}",
                    vec_to_str(&Scanner::readable_options(*c))
                );
                Ok(())
            }
            UnknownCharacters(s_list) => {
                // because of s_list is characters of splitting # before.
                write!(f, "Unknown characters \"{}\"", s_list.join("#"));
                Ok(())
            }
            UnknownStringFormat(s_list) => {
                Self::write_unknown_format_of_vec(f, s_list);
                Self::write_messages(f, "Usable String format", &[Scanner::STRING]);
                Ok(())
            }
            UnknownBooleanFormat(s_list) => {
                Self::write_unknown_format_of_vec(f, s_list);
                Self::write_messages(f, "Usable Boolean format", &[Scanner::BOOL]);
                Ok(())
            }
            UnknownStringListFormat(s_list) => {
                Self::write_unknown_format_of_vec(f, s_list);
                Self::write_messages(
                    f,
                    "Usable String list format",
                    &[Scanner::STRING_LIST, Scanner::STRING],
                );
                Ok(())
            }
            UnknownIntegerListFormat(s_list) => {
                Self::write_unknown_format_of_vec(f, s_list);
                Self::write_messages(
                    f,
                    "Usable Integer list format",
                    &[
                        Scanner::UNSIGNED_MIN_MAX,
                        Scanner::SIGNED_MIN_MAX,
                        Scanner::UNSIGNED_INT,
                    ],
                );
                Ok(())
            }
            RangeErr(from, to) => {
                write!(f, "Range Err: {} is not larger than {}", from, to);
                Ok(())
            }
        }
    }
}
