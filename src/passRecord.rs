use pbkdf2 ::{
    Pbkdf2,
    password_hash::{
        rand_core::OsRng,
        SaltString, PasswordHasher, PasswordVerifier, PasswordHash,
        errors::Error

    },
};
use std::io ;
use std::path::{Path} ;
use chrono::prelude::* ;
use std::todo ;

pub struct PassRecord<'a> {
    hash: PasswordHash<'a>,
    records: Vec<PassEnter>
}

#[derive(Debug)]
pub enum PRError {
    PasswordHashError(Error)
}

impl From<Error> for PRError {
    fn from(pe: Error) -> PRError { PRError::PasswordHashError(pe) }
}

impl <'a> PassRecord<'a> {
    // Initiates the new record by asking the password twice and creating all
    // files for further operation.
    pub fn init(dir: &Path) -> Result<Self,PRError> {
        let stdin = io::stdin() ;
        let pass1 = {
            let mut s = String::new() ;
            stdin.read_line(&mut s) ;
            s
        } ;
        let salt = SaltString::generate(&mut OsRng) ;
        let hash = Pbkdf2.hash_password(pass1.as_bytes(), &salt)? ;
        let pass2 = {
            let mut s = String::new() ;
            stdin.read_line(&mut s) ;
            s
        } ;
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)?;
        // hash.verify_password(&pass2.as_bytes(), hash);

        todo!()
    }
    pub fn seedCyccle(&self) {
        todo!();
    }
}

pub struct PassEnter {
    date: DateTime<Utc>,
    result: PassResult
}

pub enum PassResult {
    Success,
    Miss
}
