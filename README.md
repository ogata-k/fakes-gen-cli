# fakes-gen
This program is faker by random-select and support multi locale. Now, support Japan only.
So, if you want to support other locale, please pull request data of the locale.
You can generate formatted dummy data as xxxx. 
When it is String, formatter is "xxxx". When it is not String, formatter is xxxx.

## License
MIT

## install
as Library
    
    // Cargo.toml
    fakes-gen = "version"
 
as CLI for latest version

    cargo install --git https://github.com/ogata-k/fakes-gen-cli --branch master
or get from releases 

## support fake option
### Fixed user data
    // s is user data quated by ".
    FixedString(s)
    FixedNotString(s)

### Select from user data
    // list is user data such the formed: ["s1", "s2", "s3"]
    SelectString(list)
    SelectNotString(list)

### Lorem
    // unsigned integer n satisfied with from<=n<=to, 
    // you can generate dummy as joined n-data by space. 
    Word
    Words(from, to)
    Sentence
    Sentences(from, to)
    Paragraph
    Paragraphs(from, to)

### Name
    // When use with_furigana, you set true to furigana parameter.
    FirstName(with_furigana)
    FirstNameFurigana
    LastName(with_furigana)
    LastNameFurigana
    FullName(with_furigana)
    FullNameFurigana

### Primitive
    // You can use from and to paramator such as Lorem.
    // They are all not-String whithout Ascii.
    Integer
    IntegerRange(from, to)
    Float
    FloatRange(from, to)
    // Ascii chars without space
    Ascii(from, to)
    Boolean

### Internet
    // You can use from and to paramator such as Lorem. 
    // Their data is sensitive, so I pay attention dummy data to be as possible as safety.
    Email
    UserName
    Password(from, to)
    CreditCard
    URL
    IPv4
    IPv6
    RGB
    RGBA
    UserAgent
    StatusCode

### Company
    CompanySuffix
    CompanyName
    Industry

### Address
    Building
    StreetName
    CityName
    StateName
    CountryCode
    CountryName
    TimeZone
    Address
    // When you use number formatted as xxx-xxxx-xxx, set true to hyphen parameter.
    ZipCode(hyphen)
    DomesticPhoneNumber(hyphen)
    Latitude
    Longitude

### DateTime
    // format-str: https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html
    // String is format. default is "%Y-%m-%d %H:%I:%M"'s sub-format.
    // But, When Time(Date, DateTime), use only Time(Date, Time/Date)-formatter.
    // ex. "219-11-02 21:09:31"
    Time(format)
    Date(format)
    DateTime(format)

### FileSystem
    FileName
    Extension

## examples
* **all_options**: show examples of all options.
* **csv**: show record, data_set and full_form formatted csv for dummy of FullName and DateTime FakeOption.
* **tsv**: show record, data_set and full_form formatted tsv for dummy of FullName and DateTime FakeOption.
* **json**: show record, data_set and full_form formatted json for dummy of FullName and DateTime FakeOption.

## TODO
* ~~FakeOption's item is to convert to form Category(EachOption)~~
* ~~can use as cli tool~~
* add Primitive.Utf8 to FakeOption
* move helper's [not_]string_formatted to each Converter
