use failure::_core::fmt::Display;

pub fn vec_to_str<T: Display>(list: &[T]) -> String {
    format!(
        "[{}]",
        list.iter()
            .map(|l| format!("\"{}\"", l.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
