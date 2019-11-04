use failure::Fail;
use failure::_core::fmt::{Display, Error, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Scanner {
    // TODO
}

#[derive(Debug, Eq, PartialEq, Clone, Fail)]
pub struct ScannerError {
    // TODO
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
