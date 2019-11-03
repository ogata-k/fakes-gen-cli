#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Category {
    Fixed,
    Select,
    Lorem,
    Name,
    Primitive,
    Internet,
    Company,
    Address,
    DateTime,
    FileSystem,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Category::*;
        let s: String = match self {
            Fixed => "Fixed",
            Select => "Select",
            Lorem => "Lorem",
            Name => "Name",
            Primitive => "Primitive",
            Internet => "Internet",
            Company => "Company",
            Address => "Address",
            DateTime => "DateTime",
            FileSystem => "FileSystem",
        }
        .to_string();
        write!(f, "{}", s)
    }
}
