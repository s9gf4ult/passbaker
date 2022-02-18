use pbkdf2::Pbkdf2 ;
use std:: {
    io,
    path::{Path, PathBuf},
    default::Default,
    todo,
    fs::*,
} ;
use chrono::{
    prelude::*
} ;
use password_hash:: {
    SaltString, PasswordHasher, PasswordVerifier,
} ;
use crate::{
    err::*,
    header::*,
    attempts::*,
};


pub trait Interactor {
    fn showMessage(&self, s: &str) ;
    fn readPassword(&self) -> String ;
}

pub struct PassRecord<'a> {
    header: PassHeader<'a>,
    attempts: PasswordAttempts,
}

impl <'a> PassRecord<'a> {
    // Initiates the new record by asking the password twice and creating all
    // files for further operation.
    pub fn init<'b> ( dir: &PathBuf,
                      salt: &'a SaltString,
                      inter: impl Interactor,
                      name: String
    ) -> Result<PassRecord<'a> ,PRError> {
        let hash = {
            let pass = inter.readPassword() ;
            Pbkdf2.hash_password(pass.as_bytes(), salt)?
        } ;
        inter.showMessage("Repeat the password") ;
        let pass2 = inter.readPassword() ;
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)? ;

        PassRecord::checkWorkDir(dir)? ;
        let header = PassHeader {
            created: Utc::now(),
            hash: hash,
            name: name,
            options: Default::default()
        } ;

        header.createConfigs(dir)? ;

        let result = PassRecord {
            header: header,
            attempts: PasswordAttempts(vec![]),
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
