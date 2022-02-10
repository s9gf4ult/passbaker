use core::fmt ;
use pbkdf2::Pbkdf2 ;
use std::io ;
use std::path::{Path} ;
use chrono::prelude::* ;
use std::todo ;
use std::fs::* ;
use serde:: {
    *, de, de::Visitor
} ;
use password_hash:: {
    rand_core::OsRng,
    SaltString, PasswordHasher, PasswordVerifier, PasswordHash,
    errors as password_hash_errors
} ;
use toml::ser::Error as TomlError ;

pub struct PassRecord<'a> {
    header: PassHeader<'a>,
    records: Vec<PassAttempt>,
}

fn password_hash_serialize<'a, S>(p: &PasswordHash<'a>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_str(& p.to_string())
}

struct PHVisitor ;

impl<'de> Visitor<'de> for PHVisitor {
    type Value = PasswordHash<'de> ;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing password hash")
    }

    fn visit_borrowed_str<E>(self, s: &'de str) -> Result<PasswordHash<'de>, E>
    where
        E: de::Error,
    {
        PasswordHash::new(s).map_err(de::Error::custom)
    }
}

fn password_hash_deserialize<'de, D>(de: D) -> Result<PasswordHash<'de>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_str(PHVisitor)
}

#[derive(Serialize)]
pub struct PassHeader<'a> {
    name: String,
    created: DateTime<Utc>,
    #[serde(serialize_with = "password_hash_serialize")]
    #[serde(deserialize_with = "password_hash_deserialize", borrow)]
    hash: PasswordHash<'a>,
}

pub struct PassAttempt {
    date: DateTime<Utc>,
    result: PassResult
}

pub enum PassResult {
    Success,
    Miss
}

#[derive(Debug)]
pub enum PRError {
    PasswordHashError(password_hash_errors::Error),
    IOError(io::Error),
    HomeDirectoryError(String),
    TomlError(TomlError),
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

impl <'a> PassRecord<'a> {
    // Initiates the new record by asking the password twice and creating all
    // files for further operation.
    pub fn init ( dir: &Path,
                  salt: &'a SaltString,
                  asker: &(dyn Fn() -> String),
                  notifier: &(dyn Fn(&str) -> ()),
                  name: String
    ) -> Result<PassRecord<'a> ,PRError> {
        let hash = {
            let pass = asker() ; // Ask user for password
            Pbkdf2.hash_password(pass.as_bytes(), salt)?
        } ;
        notifier("Repeat the password") ;
        let pass2 = asker() ; // Repeat user password and recheck it
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)? ;

        PassRecord::checkWorkDir(dir)? ;
        let header = PassHeader {
            created: Utc::now(),
            hash: hash,
            name: name,
        } ;

        header.createConfigFile(dir)? ;

        let result = PassRecord {
            header: header,
            records: vec![],
        } ;
        Ok(result)
    }

    pub fn seedCyccle(&self) {
        todo!();
    }

    fn checkWorkDir(dir: &Path) -> Result<(), PRError> {
        match dir.metadata() {
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    // Ok just create the directory
                    create_dir(dir)?;
                },
                _ => return Err(PRError::from(e))
            },
            Ok(meta) => {
                if  !meta.file_type().is_dir() {
                    return Err(PRError::HomeDirectoryError("File name exists but this is not a directory".to_string()))
                }
            }
        } ;
        Ok(())
    }
}

impl <'a> PassHeader<'a> {
    fn createConfigFile(&self, dir: &Path) -> Result<(), PRError> {
        let s = toml::to_string_pretty(self)? ;
        let path = dir.join( &(self.name.clone() + ".toml") ) ;
        match path.metadata() {
            Err(e) => match e.kind() {
                // If file does not exists then this is because we are creating new one
                io::ErrorKind::NotFound => (),
                _ => return Err(PRError::from(e)),
            },
            Ok(_meta) => return Err(PRError::HomeDirectoryError("File already exists".to_string())),
        }
        write(path, s.as_bytes());
        Ok(())
    }
}
