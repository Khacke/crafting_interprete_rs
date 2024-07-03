pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
    FileNotUtf8(String),
    UnexpectedCharacter(usize),
    UnterminatedString(usize),
}

// region:    - Error impl
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (Error::FileNotFound(s1), Error::FileNotFound(s2)) => s1 == s2,
            (Error::FileNotUtf8(s1), Error::FileNotUtf8(s2)) => s1 == s2,
            (Error::UnexpectedCharacter(l1), Error::UnexpectedCharacter(l2)) => l1 == l2,
            (Error::UnterminatedString(l1), Error::UnterminatedString(l2)) => l1 == l2,
            _ => false
        }
    }
}
// endregion:  - Error impl