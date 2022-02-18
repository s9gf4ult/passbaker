use password_hash:: {

    errors as password_hash_errors
} ;
use toml::ser::Error as TomlError ;
use csv;
use std:: {
    io, num::TryFromIntError,
} ;


#[derive(Debug)]
pub enum PRError {
    PasswordHashError(password_hash_errors::Error),
    IOError(io::Error),
    HomeDirectoryError(String),
    TomlError(TomlError),
    CsvError(csv::Error),
    IntError(TryFromIntError),
}

impl From<password_hash_errors::Error> for PRError {
    fn from(pe: password_hash_errors::Error) -> PRError { PRError::PasswordHashError(pe) }
}

impl From<io::Error> for PRError {
    fn from(pe: io::Error) -> PRError { PRError::IOError(pe) }
}

impl From<TomlError> for PRError {
    fn from(e: TomlError) -> PRError { PRError::TomlError(e) }
}

impl From<csv::Error> for PRError {
    fn from(e: csv::Error) -> PRError { PRError::CsvError(e) }
}

impl From<TryFromIntError> for PRError {
    fn from(e: TryFromIntError) -> PRError { PRError::IntError(e) }

}
