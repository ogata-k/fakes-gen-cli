use crate::cli::file_type::FileType;

pub fn convert_from_record(file_type: FileType, header: &[String], record: &[String]) -> String {
    // assume recode's format is such as ["\"xxx\"", "\"Xsc\""]
    match file_type {
        FileType::CSV => to_csv(record),
        FileType::TSV => to_tsv(record),
        FileType::JSON => to_json(0, header, record),
    }
}

pub fn convert_from_data_set(file_type: FileType, header: &[String], data_set: &[Vec<String>]) -> String {
    let mut lines: Vec<String> = Vec::new();
    match file_type {
        FileType::CSV => {
            lines.push(to_csv(&map_string_formatted(header)));
            for data in data_set {
                lines.push(to_csv(data));
            }
        },
        FileType::TSV => {
            lines.push(to_tsv(&map_string_formatted(header)));
            for data in data_set {
                lines.push(to_tsv(data));
            }
        },
        FileType::JSON => {
            lines.push("{".to_string());
            let mut items: Vec<String> = Vec::new();
            lines.push(format!("{}\"fake\": [", " ".repeat(4*1)));
            for data in data_set {
                items.push(to_json(2, header, data));
            }
            lines.push(items.join(",\n"));
            lines.push(format!("{}]", " ".repeat(4*1)));
            lines.push("}".to_string());
        }
    }
    return lines.join("\n");
}

fn to_csv(record: &[String]) -> String {
    record.join(",")
}

fn to_tsv(record: &[String]) -> String {
    record.join("\t")
}

fn to_json(indent: u8, header: &[String], record: &[String]) -> String {
    let space: String =  " ".repeat(4 * indent as usize);
    let mut s_vec: Vec<String> = vec![format!("{}{{",space)];
    assert_eq!(header.len(), record.len());
    let items: Vec<String> = header.iter().zip(record)
        .map(|(h, r)| format!("    {}\"{}\": {}", space, h, r))
        .collect();
    s_vec.push(items.join(",\n"));
    s_vec.push(format!("{}}}", space));
    return s_vec.join("\n");
}