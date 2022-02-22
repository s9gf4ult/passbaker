use chrono::prelude::*;
use password_hash::{PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use std::{
    default::Default,
    fs::*,
    io,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use crate::{attempts::*, err::*, header::*, options::*};

pub trait Interactor {
    fn read_password(&mut self) -> String;
    fn show_message(&mut self, s: &str);
    fn pay_attention(&mut self, s: &str) {
        self.show_message(s);
    }
}

pub struct PassRecord<'a> {
    header: PassHeader<'a>,
    attempts: PasswordAttempts,
}

impl<'a> PassRecord<'a> {
    /// Initiates the new record by asking the password twice and creating all
    /// files for further operation.
    pub fn init<'b>(
        dir: &PathBuf,
        salt: &'a SaltString,
        inter: &mut impl Interactor,
        name: String,
    ) -> Result<PassRecord<'a>, PRError> {
        let hash = {
            let pass = inter.read_password();
            Pbkdf2.hash_password(pass.as_bytes(), salt)?
        };
        inter.show_message("Repeat the password");
        let pass2 = inter.read_password();
        Pbkdf2.verify_password(pass2.as_bytes(), &hash)?;

        PassRecord::check_work_dir(dir)?;
        let header = PassHeader {
            created: Utc::now(),
            hash: hash,
            name: name,
            options: Default::default(),
        };

        header.create_configs(dir)?;

        let result = PassRecord {
            header: header,
            attempts: PasswordAttempts(vec![]),
        };
        Ok(result)
    }

    pub fn seed_cycle(
        &mut self,
        home: &PathBuf,
        inter: &mut impl Interactor,
    ) -> Result<(), PRError> {
        while let (Stage::Seed, next) = self
            .attempts
            .next_attempt(&self.header.created, &self.header.options)?
        {
            let mut now = Utc::now();
            while next > now {
                sleep(Duration::from_secs(1));
                now = Utc::now();
                let msg = format!("There is {} seconds left", (next - now).num_seconds());
                inter.show_message(&msg);
            }
            let dir = self.header.attempts_dir_name(home);
            inter.pay_attention("Enter password:");
            let mut s = inter.read_password();
            while !self.header.check_pass(&s)? {
                let attempt = Box::new(PassAttempt {
                    timestamp: Utc::now(),
                    result: AttemptResult::Miss,
                });
                self.attempts.register_attempt(&dir, attempt)?;
                inter.show_message("Wrong password, try again");
                s = inter.read_password();
            }
            let attempt = Box::new(PassAttempt {
                timestamp: Utc::now(),
                result: AttemptResult::Success,
            });
            self.attempts.register_attempt(&dir, attempt)?;
        }
        Ok(())
    }

    fn check_work_dir(dir: &Path) -> Result<(), PRError> {
        match dir.metadata() {
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    // Ok just create the directory
                    create_dir(dir)?;
                }
                _ => return Err(PRError::from(e)),
            },
            Ok(meta) => {
                if !meta.file_type().is_dir() {
                    return Err(PRError::HomeDirectoryError(
                        "File name exists but this is not a directory".to_string(),
                    ));
                }
            }
        };
        Ok(())
    }
}
