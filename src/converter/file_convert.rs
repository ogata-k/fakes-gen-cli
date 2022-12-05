use crate::converter::file_type::FileType;
use crate::faker::fake_options::FakeOption;
use crate::faker::Faker;
use rand::Rng;
use std::io;

/// one record
pub fn to_record<W: io::Write, R: Rng>(
    w: &mut W,
    faker: &mut Faker<R>,
    file_type: FileType,
    header_options: &[(String, FakeOption)],
) -> io::Result<()> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::new(header_options);
            converter.to_record(w, &faker.gen_record(converter.options()))
        }
        FileType::TSV => {
            let converter = TsvConverter::new(header_options);
            converter.to_record(w, &faker.gen_record(converter.options()))
        }
        FileType::JSON => {
            let converter = JsonConverter::new(header_options);
            converter.to_record(w, &faker.gen_record(converter.options()))
        }
    }
}

/// one record with header
pub fn to_record_with_header<W: io::Write, R: Rng>(
    w: &mut W,
    faker: &mut Faker<R>,
    file_type: FileType,
    header_options: &[(String, FakeOption)],
) -> io::Result<()> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::new(header_options);
            converter.to_record_with_header(w, &faker.gen_record(converter.options()))
        }
        FileType::TSV => {
            let converter = TsvConverter::new(header_options);
            converter.to_record_with_header(w, &faker.gen_record(converter.options()))
        }
        FileType::JSON => {
            let converter = JsonConverter::new(header_options);
            converter.to_record_with_header(w, &faker.gen_record(converter.options()))
        }
    }
}

/// many record
pub fn to_data_set<W: io::Write, R: Rng>(
    w: &mut W,
    faker: &mut Faker<R>,
    file_type: FileType,
    header_options: &[(String, FakeOption)],
    count: usize,
) -> io::Result<()> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::new(header_options);
            converter.to_data_set(w, &faker.gen_data_set(count, converter.options()))
        }
        FileType::TSV => {
            let converter = TsvConverter::new(header_options);
            converter.to_data_set(w, &faker.gen_data_set(count, converter.options()))
        }
        FileType::JSON => {
            let converter = JsonConverter::new(header_options);
            converter.to_data_set(w, &faker.gen_data_set(count, converter.options()))
        }
    }
}

/// full formed many record
pub fn to_full_form<W: io::Write, R: Rng>(
    w: &mut W,
    faker: &mut Faker<R>,
    file_type: FileType,
    header_options: &[(String, FakeOption)],
    count: usize,
) -> io::Result<()> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::new(header_options);
            converter.to_full_form(w, &faker.gen_data_set(count, converter.options()))
        }
        FileType::TSV => {
            let converter = TsvConverter::new(header_options);
            converter.to_full_form(w, &faker.gen_data_set(count, converter.options()))
        }
        FileType::JSON => {
            let converter = JsonConverter::new(header_options);
            converter.to_full_form(w, &faker.gen_data_set(count, converter.options()))
        }
    }
}

fn split_options(header_options: &[(String, FakeOption)]) -> (Vec<String>, Vec<FakeOption>) {
    let mut header: Vec<String> = Vec::new();
    let mut options: Vec<FakeOption> = Vec::new();

    for (h, opt) in header_options {
        header.push(h.clone());
        options.push(opt.clone());
    }

    return (header, options);
}

/// Converter
trait Converter {
    // create
    fn new(header_options: &[(String, FakeOption)]) -> Self;

    fn header(&self) -> &Vec<String>;

    fn options(&self) -> &Vec<FakeOption>;

    // formatter
    fn formatted_header(&self) -> Vec<String> {
        self.header().iter().map(|h| format!("\"{}\"", h)).collect()
    }

    fn formatted_record(&self, record: &[String]) -> Vec<String> {
        record
            .iter()
            .zip(self.options())
            .map(|(rec, opt)| opt.with_format(rec))
            .collect()
    }

    // do not assertion
    /// write a record with flush
    fn to_header<W: io::Write>(&self, w: &mut W) -> io::Result<()>;

    /// write a record with flush
    fn to_record<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()>;

    fn to_record_with_header<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()>;

    fn to_data_set<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()>;

    fn to_full_form<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()>;
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct CsvConverter {
    header: Vec<String>,
    options: Vec<FakeOption>,
}

impl Converter for CsvConverter {
    fn new(header_options: &[(String, FakeOption)]) -> Self {
        let (header, options): (Vec<String>, Vec<FakeOption>) = split_options(header_options);
        CsvConverter { header, options }
    }

    fn header(&self) -> &Vec<String> {
        &self.header
    }

    fn options(&self) -> &Vec<FakeOption> {
        &self.options
    }

    /// write a record with flush
    fn to_header<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self.formatted_header().join(","))?;
        w.flush()
    }

    /// write a record with flush
    fn to_record<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        write!(w, "{}", self.formatted_record(&record).join(","))?;
        w.flush()
    }

    fn to_record_with_header<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        self.to_header(w)?;
        write!(w, "\n")?;
        self.to_record(w, record)?;
        Ok(())
    }

    fn to_data_set<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        if data_set.is_empty() {
            return Ok(());
        }
        if let Some((fst, snd)) = data_set.split_first() {
            self.to_record(w, fst)?;
            for record in snd {
                write!(w, "\n")?;
                self.to_record(w, record)?;
            }
        }
        Ok(())
    }

    fn to_full_form<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        self.to_header(w)?;
        for record in data_set {
            write!(w, "\n")?;
            self.to_record(w, record)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct TsvConverter {
    header: Vec<String>,
    options: Vec<FakeOption>,
}

impl Converter for TsvConverter {
    fn new(header_options: &[(String, FakeOption)]) -> Self {
        let (header, options): (Vec<String>, Vec<FakeOption>) = split_options(header_options);
        TsvConverter { header, options }
    }

    fn header(&self) -> &Vec<String> {
        &self.header
    }

    fn options(&self) -> &Vec<FakeOption> {
        &self.options
    }

    /// write a record with flush
    fn to_header<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self.formatted_header().join("\t"))?;
        w.flush()
    }

    /// write a record with flush
    fn to_record<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        write!(w, "{}", self.formatted_record(&record).join("\t"))?;
        w.flush()
    }

    fn to_record_with_header<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        self.to_header(w)?;
        write!(w, "\n")?;
        self.to_record(w, record)?;
        Ok(())
    }

    fn to_data_set<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        if data_set.is_empty() {
            return Ok(());
        }
        if let Some((fst, snd)) = data_set.split_first() {
            self.to_record(w, fst)?;
            for record in snd {
                write!(w, "\n")?;
                self.to_record(w, record)?;
            }
        }
        Ok(())
    }

    fn to_full_form<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        self.to_header(w)?;
        for record in data_set {
            write!(w, "\n")?;
            self.to_record(w, record)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct JsonConverter {
    indent_count: usize,
    one_indent: &'static str,
    header: Vec<String>,
    options: Vec<FakeOption>,
}

impl JsonConverter {
    fn add_indent(&self, num: usize) -> JsonConverter {
        JsonConverter {
            indent_count: self.indent_count + num,
            one_indent: self.one_indent,
            header: self.header().clone(),
            options: self.options().clone(),
        }
    }

    fn get_indent(&self) -> String {
        self.one_indent.repeat(self.indent_count)
    }
}

impl Converter for JsonConverter {
    fn new(header_options: &[(String, FakeOption)]) -> Self {
        let (header, options) = split_options(header_options);
        JsonConverter {
            indent_count: 0,
            one_indent: "  ",
            header,
            options,
        }
    }

    fn header(&self) -> &Vec<String> {
        &self.header
    }

    fn options(&self) -> &Vec<FakeOption> {
        &self.options
    }

    fn formatted_header(&self) -> Vec<String> {
        unreachable!()
    }

    #[allow(unused_variables)]
    fn to_header<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        unreachable!()
    }

    /// write a record with flush
    fn to_record<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        let indent: String = self.get_indent();
        write!(w, "{}{{", indent)?;
        let record_items: Vec<String> = self
            .header()
            .iter()
            .zip(self.formatted_record(&record))
            .map(|(h, r)| format!("{}{}\"{}\": {}", self.one_indent, indent, h, r))
            .collect();
        if let Some((head, tails)) = record_items.split_first() {
            write!(w, "\n{}", head)?;
            for tail in tails {
                write!(w, ",\n{}", tail)?;
            }
        }
        write!(w, "\n{}}}", indent)?;
        w.flush()
    }

    fn to_record_with_header<W: io::Write>(&self, w: &mut W, record: &[String]) -> io::Result<()> {
        self.to_record(w, record)
    }

    /// array for json value
    fn to_data_set<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        let indent: String = self.get_indent();

        write!(w, "{}[", indent)?;
        let indented_converter: JsonConverter = self.add_indent(1);
        if let Some((head, tails)) = data_set.split_first() {
            write!(w, "\n")?;
            indented_converter.to_record(w, head)?;
            for tail in tails {
                write!(w, ",\n")?;
                indented_converter.to_record(w, tail)?;
            }
            write!(w, "\n{}", indent)?;
        }
        write!(w, "]")?;
        w.flush()
    }

    fn to_full_form<W: io::Write>(&self, w: &mut W, data_set: &[Vec<String>]) -> io::Result<()> {
        let indent: String = self.get_indent();

        write!(w, "{}{{", indent)?;
        write!(w, "\n{}{}\"dummy\": [", self.one_indent, indent,)?;
        let indented_converter: JsonConverter = self.add_indent(2);
        if let Some((head, tails)) = data_set.split_first() {
            write!(w, "\n")?;
            indented_converter.to_record(w, head)?;
            for tail in tails {
                write!(w, ",\n")?;
                indented_converter.to_record(w, tail)?;
            }
            write!(w, "\n{}{}", self.one_indent, indent)?;
        }
        write!(w, "]")?;
        write!(w, "\n{}}}", indent)?;
        w.flush()
    }
}
