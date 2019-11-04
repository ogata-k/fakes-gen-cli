use crate::converter::file_type::FileType;
use crate::date_time_format::DEFAULT_DATE_TIME_FORMAT;
use chrono::Local;

fn map_string_formatted(data: &[String]) -> Vec<String> {
    data.iter().map(|d| format!("\"{}\"", d)).collect()
}

/// one record
pub fn to_record(header: &[String], record: &[String], file_type: FileType) -> Option<String> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::default();
            if converter.validate(header, record).is_none() {
                return None;
            }
            return Some(converter.to_record(header, record));
        }
        FileType::TSV => {
            let converter = TsvConverter::default();
            if converter.validate(header, record).is_none() {
                return None;
            }
            return Some(converter.to_record(header, record));
        }
        FileType::JSON => {
            let converter = JsonConverter::default();
            if converter.validate(header, record).is_none() {
                return None;
            }
            return Some(converter.to_record(header, record));
        }
    }
}

/// many record
pub fn to_data_set(
    header: &[String],
    data_set: &[Vec<String>],
    file_type: FileType,
) -> Option<String> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_data_set(header, data_set));
        }
        FileType::TSV => {
            let converter = TsvConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_data_set(header, data_set));
        }
        FileType::JSON => {
            let converter = JsonConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_data_set(header, data_set));
        }
    }
}

/// full formed many record
pub fn to_full_form(
    header: &[String],
    data_set: &[Vec<String>],
    file_type: FileType,
) -> Option<String> {
    match file_type {
        FileType::CSV => {
            let converter = CsvConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_full_form(header, data_set));
        }
        FileType::TSV => {
            let converter = TsvConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_full_form(header, data_set));
        }
        FileType::JSON => {
            let converter = JsonConverter::default();
            if !data_set
                .iter()
                .all(|d| converter.validate(header, &d).is_some())
            {
                return None;
            }
            return Some(converter.to_full_form(header, data_set));
        }
    }
}

/// Converter
trait Converter: Default {
    // assertion
    fn validate(&self, header: &[String], record: &[String]) -> Option<()>;

    // do not assertion
    fn to_record(&self, header: &[String], record: &[String]) -> String;
    fn to_data_set(&self, header: &[String], data_set: &[Vec<String>]) -> String;
    fn to_full_form(&self, header: &[String], data_set: &[Vec<String>]) -> String;
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct CsvConverter {}

impl Converter for CsvConverter {
    fn validate(&self, header: &[String], record: &[String]) -> Option<()> {
        if header.len() == record.len() {
            Some(())
        } else {
            None
        }
    }

    fn to_record(&self, _header: &[String], record: &[String]) -> String {
        record.join(",")
    }

    fn to_data_set(&self, _header: &[String], data_set: &[Vec<String>]) -> String {
        let mut lines: Vec<String> = Vec::new();
        for data in data_set {
            lines.push(self.to_record(_header, data));
        }
        return lines.join("\n");
    }

    fn to_full_form(&self, header: &[String], data_set: &[Vec<String>]) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(self.to_record(header, &map_string_formatted(header)));
        for data in data_set {
            lines.push(self.to_record(header, data));
        }
        return lines.join("\n");
    }
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct TsvConverter {}

impl Converter for TsvConverter {
    fn validate(&self, header: &[String], record: &[String]) -> Option<()> {
        if header.len() == record.len() {
            Some(())
        } else {
            None
        }
    }

    fn to_record(&self, _header: &[String], record: &[String]) -> String {
        record.join("\t")
    }

    fn to_data_set(&self, _header: &[String], data_set: &[Vec<String>]) -> String {
        let mut lines: Vec<String> = Vec::new();
        for data in data_set {
            lines.push(self.to_record(_header, data));
        }
        return lines.join("\n");
    }

    fn to_full_form(&self, header: &[String], data_set: &[Vec<String>]) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(self.to_record(header, &map_string_formatted(header)));
        for data in data_set {
            lines.push(self.to_record(header, data));
        }
        return lines.join("\n");
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct JsonConverter {
    indent: u8,
    indent_space_count: u8,
}

impl Default for JsonConverter {
    fn default() -> Self {
        JsonConverter {
            indent: 0,
            indent_space_count: 2,
        }
    }
}

impl Converter for JsonConverter {
    fn validate(&self, header: &[String], record: &[String]) -> Option<()> {
        if header.len() == record.len() {
            Some(())
        } else {
            None
        }
    }

    fn to_record(&self, header: &[String], record: &[String]) -> String {
        let one_indent: String = " ".repeat(self.indent_space_count as usize);
        let indent: String = " ".repeat((self.indent_space_count * self.indent) as usize);
        let mut s_vec: Vec<String> = vec![format!("{}{{", indent)];
        let items: Vec<String> = header
            .iter()
            .zip(record)
            .map(|(h, r)| format!("{}{}\"{}\": {}", one_indent, indent, h, r))
            .collect();
        s_vec.push(items.join(",\n"));
        s_vec.push(format!("{}}}", indent));
        return s_vec.join("\n");
    }

    fn to_data_set(&self, header: &[String], data_set: &[Vec<String>]) -> String {
        let indent: String = " ".repeat((self.indent_space_count * self.indent) as usize);
        let mut lines: Vec<String> = Vec::new();
        let mut items: Vec<String> = Vec::new();

        lines.push(format!("{}[", indent));
        let indented_converter: Self = JsonConverter {
            indent: self.indent + 1,
            indent_space_count: self.indent_space_count,
        };
        for data in data_set {
            items.push(indented_converter.to_record(header, data));
        }
        lines.push(items.join(",\n"));
        lines.push(format!("{}]", indent));
        return lines.join("\n");
    }

    fn to_full_form(&self, header: &[String], data_set: &[Vec<String>]) -> String {
        let one_indent = " ".repeat(self.indent_space_count as usize);
        let indent: String = " ".repeat((self.indent_space_count * self.indent) as usize);
        let mut lines: Vec<String> = vec![format!("{}{{", indent)];
        let mut items: Vec<String> = Vec::new();

        lines.push(format!(
            "{}{}\"{}\": [",
            one_indent,
            indent,
            Local::now().format(DEFAULT_DATE_TIME_FORMAT)
        ));
        let indented_converter: Self = JsonConverter {
            indent: self.indent + 2,
            indent_space_count: self.indent_space_count,
        };
        for data in data_set {
            items.push(indented_converter.to_record(header, data));
        }
        lines.push(items.join(",\n"));
        lines.push(format!("{}{}]", one_indent, indent));
        lines.push(format!("{}}}", indent));
        return lines.join("\n");
    }
}
