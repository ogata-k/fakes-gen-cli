use failure::Fail;
use std::fmt::{Formatter, Error};

#[derive(Debug, Eq, PartialEq, Clone, Fail)]
pub struct OptionError {
// TODO
}

impl std::fmt::Display for OptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}