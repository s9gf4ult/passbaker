use pbkdf2 ::{
    Pbkdf2,
    password_hash::{
        rand_core::OsRng,
        SaltString, PasswordHasher, PasswordVerifier, PasswordHash,
        errors as password_hash_errors
    },
};
use std::io ;
use std::path::{Path} ;
use chrono::prelude::* ;
use std::todo ;
use std::fs::* ;

pub struct PassRecord<'a> {
    hash: PasswordHash<'a>,
    name: String,
    records: Vec<PassAttempt>
}

#[derive(Debug)]
pub enum PRError {
    PasswordHashError(password_hash_errors::Error),
    IOError(io::Error),
    HomeDirectoryError(String)
}

impl From<password_hash_errors::Error> for PRError {
    fn from(pe: password_hash_errors::Error) -> PRError { PRError::PasswordHashError(pe) }
}

impl From<io::Error> for PRError {
    fn from(pe: io::Error) -> PRError { PRError::IOError(pe) }
}

impl <'a> PassRecord<'a> {
    // Initiates the new record by asking the password twice and creating all
    // files for further operation.
    pub fn init ( dir: &Path,
                  asker: &(dyn Fn() -> String),
                  notifier: &(dyn Fn(&str) -> ()),
                  name: String
    ) -> Result<PassRecord<'a> ,PRError> {
        let salt = SaltString::generate(&mut OsRng) ;
        let hash = {
            let pass = asker() ; // Ask user for password
            Pbkdf2.hash_password(pass.as_bytes(), &salt.clone())?
        } ;
        notifier("Repeat the password") ;
        let pass2 = asker() ; // Repeat user password and recheck it
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)? ;

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

        let result = PassRecord {
            hash: hash,
            name: name,
            records: vec![]
        } ;

        Ok(result)
    }
    pub fn seedCyccle(&self) {
        todo!();
    }
}

pub struct PassAttempt {
    date: DateTime<Utc>,
    result: PassResult
}

pub enum PassResult {
    Success,
    Miss
}
