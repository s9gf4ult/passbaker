use core::fmt ;
use pbkdf2::Pbkdf2 ;
use std:: {
    io,
    path::{Path, PathBuf},
    default::Default,
    todo,
    fs::*,
} ;
use chrono::{
    prelude::*, Duration
} ;
use serde:: {
    *, de, de::Visitor
} ;
use password_hash:: {
    rand_core::OsRng,
    SaltString, PasswordHasher, PasswordVerifier, PasswordHash,
    errors as password_hash_errors
} ;
use toml::ser::Error as TomlError ;
use csv::Writer;

use crate::{
    options::*,
    err::*
};

pub trait Interactor {
    fn showMessage(&self, s: &str) ;
    fn readPassword(&self) -> String ;
}

pub struct PasswordAttempts (Vec<Box<PassAttempt>>) ;

pub struct PassRecord<'a> {
    header: PassHeader<'a>,
    attempts: PasswordAttempts,
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
    options: Options,
}

#[derive(Serialize, Deserialize)]
pub struct PassAttempt {
    date: DateTime<Utc>,
    result: AttemptResult,
}

#[derive(Serialize, Deserialize)]
pub enum AttemptResult {
    Success,
    Miss
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

fn dirExists(dir: &PathBuf) -> Result<(), PRError> {
    match dir.metadata() {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                create_dir(dir) ;
            },
            _ => return Err(PRError::from(e)),
        },
        Ok(meta) => if ! meta.file_type().is_dir() {
            return Err(PRError::HomeDirectoryError("File exists but this is not a directory".to_string())) ;
        }
    };
    Ok(())
}

impl PasswordAttempts {
    fn registerAttempt(&mut self, dir: &PathBuf, item: Box<PassAttempt>) -> Result<(), PRError> {
        let filename: Result<PathBuf, PRError> = {
            dirExists(dir)? ;
            let dateStr = item.date.date().to_string() ;
            let f = dateStr + ".csv" ;
            Ok(dir.clone().join(f))
        };
        let filename = filename? ;
        let mut writer = Writer::from_path(&filename)? ;
        writer.serialize(&item)? ;
        writer.flush()? ;
        self.0.push(item) ;
        Ok(())
    }

    fn nextAttempt(&self, created: DateTime<Utc>, opts: &Options) -> Result<DateTime<Utc>, PRError> {
        todo!()
    }
}

impl <'a> PassHeader<'a> {
    fn configFileName(&self, dir: &PathBuf) -> PathBuf {
        dir.join( &(self.name.clone() + ".toml") )
    }

    fn attemptsDirName(&self, dir: &PathBuf) -> PathBuf {
        dir.join( &self.name )
    }

    fn createConfigs(&self, dir: &PathBuf) -> Result<(), PRError> {
        let s = toml::to_string_pretty(self)? ;
        let dirPath = self.attemptsDirName(dir) ;
        let path = self.configFileName(dir) ;
        match path.metadata() {
            Err(e) => match e.kind() {
                // If file does not exists then this is because we are creating new one
                io::ErrorKind::NotFound => (),
                _ => return Err(PRError::from(e)),
            },
            Ok(_meta) => return Err(PRError::HomeDirectoryError("File already exists".to_string())),
        }
        dirExists(&dirPath)? ;
        write(path, s.as_bytes());
        Ok(())
    }
}
