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
or get from crates.io

    cargo install fakes-gen

# As Library
## support fake option
### With other option
    // "sep" is separator for data generated as each "options" 
    Join(sep, options),

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

# As CLI
## usage

     fakes-gen [FLAGS] [OPTIONS] [option]...
     
## \[FLAGS\] and \[OPTIONS\]

    // help message from fakes-gen -h 
    FLAGS:
        -b, --bnf         show Backus-Naur Form and detail format for options
        -f, --fullform    flag for generating as fullform such as body with header
        -h, --help        Print this message
        -u, --usable      show list of all usable options for faker
        -V, --version     Prints version information
    
    OPTIONS:
        -c, --converter <converter>    converter for output [default: csv]  [possible values: csv, tsv, json]
        -l, --locale <locale>          3-char's country code. [default: jpn]  [possible values: jpn]
        -s, --size <size>              data size. If 1, generate as record. If over 1, generate as data_set. [default: 1]

## usable format of \[option\]
Usable format is form such as ```Xxxx.Yyyy(zzz)``` or ```Xxxx.Yyyy(zzz#sub)```.
```Xxxx``` is Category. Usable Category is "With", " "Fixed", "Select", "Lorem", "Name", "Primitive", "Internet", "Company", "Address", "DateTime" and "FileSystem".
```Yyyy``` is Option for each Category. Usable Option is theirs and ```zzz``` is ```<column_name>```.
 If you use header or fullform, fakes-gen use ```<column_name>```.
```
// modified usable message from fakes-gen -u
Category:
 With
Options:
// With.Join(h)oge#_dd_#Select.String(hoge#sss)#2#Select.NotString(1#2#3)) -> "sss_dd_1_dd_2"
・With\.Join\(<column_name>#<join_separator>(#<repeatable_option>)*\)

Category:
 Fixed  // fixed value of user-value. 
Options:
・Fixed.String\(<column_name>#<string>\)  // as String format "hoge".
・Fixed.NotString\(<column_name>#<string>\)  // as not String format hoge.
Example: 
fakes-gen Fixed.String(_#hoge)  // "hoge"

Category:
 Select  // select a value from user-values.
Options:
・Select.String\(<column_name>#<string_list>\)  // as String format "hoge".
・Select.NotString\(<column_name>#<string_list>\)  // as not String format hoge.
Example: 
fakes-gen Select.NotString(_#hoge#fuga)  // hoge

Category:
 Lorem  // dummy words 
Options:
・Lorem.Word\(<column_name>(#<unsigned_integer_range>)?\)  // one word or words of num in the range
・Lorem.Sentence\(<column_name>(#<unsigned_integer_range>)?\)  // one sentence or sentences of num in the range
・Lorem.Paragraph\(<column_name>(#<unsigned_integer_range>)?\)  // one paragraph or paragraphs of num in the range
Example: 
fakes-gen Lorem.Word(_)  // "氷山"
fakes-gen Lorem.Word(_#2#4)  // "有向グラフ Haskell Rust"

Category:
 Name  // name
Options:
・Name.FirstName\(<column_name>\)  // first name
・Name.FirstNameFurigana\(<column_name>\)  // furigana of first name if exist, first name if not exist
・Name.LastName\(<column_name>\)  // last name
・Name.LastNameFurigana\(<column_name>\)  // furigana of last name if exist, last name if not exist
・Name.FullName\(<column_name>\)  // full name
・Name.FullNameFurigana\(<column_name>\)  // furigana of full name if exist, full name if not exist
Example: 
fakes-gen Name.FullName(_) Name.LastNameFurigana(_) Name.FirstNameFurigana(_)  // "露木 静男","ツユキ","シズオ"

Category:
 Primitive  // primitive values
Options:
・Primitive.Int\(<column_name>(#<signed_integer_range>)?\)  // integer or integer with limit range
・Primitive.Float\(<column_name>(#<signed_integer_range>)?\)  // real num with limit range
・Primitive.Ascii\(<column_name>(#<signed_integer_range>)?\)  // one ascii char or ascii chars of num in the range
・Primitive.Bool\(<column_name>\)  // boolean
Example: 
fakes-gen Primitive.Float(_)  // -32544.35
fakes-gen Primitive.Float(_#-1#1)  // -0.87

Category:
 Internet  // internet
Options:
・Internet.Email\(<column_name>\)  // safe address of email
・Internet.UserName\(<column_name>\)  // name of user account
・Internet.Password\(<column_name>#<unsigned_integer_range>\)  // password as length one or num in the range
・Internet.CreditCard\(<column_name>\)  // safe number sequence of credit card
・Internet.URL\(<column_name>\)  // safe url
・Internet.IPv4\(<column_name>\)  // safe ipv4
・Internet.IPv6\(<column_name>\)  // safe ipv6
・Internet.RGB\(<column_name>\)  // rgb color such as #12480F
・Internet.RGBA\(<column_name>\)  // rgb with alpha such as #09AF50CB
・Internet.UserAgent\(<column_name>\)  // user agent
・Internet.StatusCode\(<column_name>\)  // status of http request
Example: 
fakes-gen Internet.URL(_) Internet.StatusCode(_)  // "http://example.com/B/lUVB","6IxT4VL92u"

Category:
 Company  // compnay
Options:
・Company.Suffix\(<column_name>\)  // suffix of company such as Corp.
・Company.Name\(<column_name>\)  // name of company
・Company.Industry\(<column_name>\)  // domain of industry
Example: 
fakes-gen Company.Name(_) Company.Industry(_)  // "フリーダム匿名組合","宿泊業"

Category:
 Address  // address in country
Options:
・Address.Building\(<column_name>\)  // name of building
・Address.Street\(<column_name>\)  // street in country
・Address.City\(<column_name>\)  // city in country
・Address.State\(<column_name>\)  // state in country
・Address.CountryCode\(<column_name>\)  // country code of three chars
・Address.CountryName\(<column_name>\)  // name of country
・Address.TimeZone\(<column_name>\)  // time zone
・Address.Address\(<column_name>\)  // address
・Address.ZipCode\(<column_name>(#<bool>)?\)  // zipcode with hyphen when not use <bool> or set true 
・Address.Phone\(<column_name>(#<bool>)?\)  // dummy phone number with hyphen when not use <bool> or set true 
・Address.Latitude\(<column_name>\)  // latitude such as +20.134875,-00.134875
・Address.Longitude\(<column_name>\)  // logitude +028.672211,-228.672211
Example: 
fakes-gen Address.Phone(_#true)  // "03-357-1407"

Category:
 DateTime  // date time with sub-format of "%Y-%m-%d %H:%I:%M" 
 // If you want to know format, please reference to https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers
Options:
・DateTime.Time\(<column_name>#<format_string>\)
・DateTime.Date\(<column_name>#<format_string>\)
・DateTime.DateTime\(<column_name>#<format_string>\)
Example: 
fakes-gen DateTime.Date(_#%m/%d) DateTime.Time(_)  // "08/17","18:06:18"

Category:
 FileSystem  // file
Options:
・FileSystem.FileName\(<column_name>\)  // name with extension of file
・FileSystem.Extension\(<column_name>\)  // extension
Example: 
fakes-gen FileSystem.FileName(_)  // "8yIY6C4Pl.csv"
```

And their option's format is theirs. \[option\] is \<option\>.
```
<option> := <normal_option>|<special_option>
<normal_option> := <category>\.<option_name>\(<column_name>(#<sub_option>)?\)
<special_option> := <with_join_option>
<with_join_option> := With\.Join\(<column_name>#<join_separator>(#<repeatable_option>)*\)
<join_separator> := [^#]*
<repeatable_option> := <unsigned_integer>?#<option_without_column_name>
<option_without_column_name> := <category>\.<option_name>\((<sub_option>)?\)
<category> := [A-Z][0-9a-zA-Z]*
<option_name> := [A-Z][0-9a-zA-Z]*
<column_name> := <string>
<sub_option> := <string>|<string_list>|<unsigned_integer_range>|<signed_integer_range>|<boolean>|<format_string>
<string> := ((".*")|[.[^\ ]]*)
<string_list> := \[<string>(#<string>)*\]
<unsigned_integer_range> := <unsigned_integer>#<unsigned_integer>
<signed_integer_range> := -?<unsigned_integer>#-?<unsigned_integer>
<unsigned_integer> := [0-9][1-9]*
<bool> := (true)|(false)
<format_string> := <string>
If you want to know <format_string>, please reference to https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers

```

# TODO
* ~~FakeOption's item is to convert to form Category(EachOption)~~
* ~~can use as cli tool~~
* add Primitive.Utf8 to FakeOption
* can specify character-code when generate
* ~~move helper's \[not_\]string_formatted to each Converter~~
