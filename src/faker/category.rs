#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Category {
    With,
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

impl Category {
    pub fn all_list() -> Vec<Self> {
        use Category::*;
        vec![
            With, Fixed, Select, Lorem, Name, Primitive, Internet, Company, Address, DateTime,
            FileSystem,
        ]
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Category::*;
        let s: String = match self {
            With => "With",
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
