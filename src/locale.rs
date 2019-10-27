#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Locale {
    Japan
}

impl Default for Locale{
    fn default() -> Self {
        Locale::Japan
    }
}