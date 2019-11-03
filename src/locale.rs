#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Locale {
    Japan,
}

impl Default for Locale {
    fn default() -> Self {
        Locale::Japan
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Locale::*;
        let s: String = match self {
            Japan => "Japan",
        }
        .to_string();

        write!(f, "{}", s)
    }
}
