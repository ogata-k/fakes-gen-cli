use failure::Fail;
use std::fmt::{Formatter, Error};

#[derive(Debug, Eq, PartialEq, Clone, Fail)]
pub struct OptionParseError {
// TODO
}

impl std::fmt::Display for OptionParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}