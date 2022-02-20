use pbkdf2::Pbkdf2 ;
use std:: {
    io,
    path::{Path, PathBuf},
    thread::sleep,
    default::Default,
    todo,
    fs::*,
    time::Duration,
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
    options::*,
};


pub trait Interactor {
    fn show_message(&self, s: &str) ;
    fn read_password(&self) -> String ;
}

pub struct PassRecord<'a> {
    header: PassHeader<'a>,
    attempts: PasswordAttempts,
}

impl <'a> PassRecord<'a> {
    /// Initiates the new record by asking the password twice and creating all
    /// files for further operation.
    pub fn init<'b> ( dir: &PathBuf,
                      salt: &'a SaltString,
                      inter: &impl Interactor,
                      name: String
    ) -> Result<PassRecord<'a> ,PRError> {
        let hash = {
            let pass = inter.read_password() ;
            Pbkdf2.hash_password(pass.as_bytes(), salt)?
        } ;
        inter.show_message("Repeat the password") ;
        let pass2 = inter.read_password() ;
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)? ;

        PassRecord::check_work_dir(dir)? ;
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

    pub fn seed_cycle(&mut self, home: &PathBuf, inter: &impl Interactor) -> Result<(), PRError> {
        while let (Stage::Seed, next) =
            self.attempts.next_attempt(&self.header.created, &self.header.options)? {
                let mut now = Utc::now() ;
                while next > now {
                    sleep(Duration::from_secs(1));
                    now = Utc::now() ;
                }
                inter.show_message("Enter password: ") ;
                let s = inter.read_password() ;
                let res = if self.header.check_pass(&s)? {
                    AttemptResult::Success
                } else {
                    AttemptResult::Miss
                };
                let attempt = Box::new(PassAttempt{
                    timestamp: Utc::now(),
                    result: res,
                });
                let dir = self.header.attemptsDirName(home);
                self.attempts.register_attempt(&dir, attempt)? ;
        };
        todo!() ;
    }

    fn check_work_dir(dir: &Path) -> Result<(), PRError> {
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
