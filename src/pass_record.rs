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

pub trait Interactor {
    fn showMessage(&self, s: &str) ;
    fn readPassword(&self) -> String ;
}

impl Default for Options {
    fn default() -> Options {
        Options {
            seed: TimingOpts
            { timeFactor: 2.0,
              maxInterval: 3600,
              completion: Some(10)
            },
            consolidation: TimingOpts
            { timeFactor: 2.0 ,
              maxInterval: 3600*24, // Repeat at least every day
              completion: Some(10),
            },
            retention: TimingOpts
            { timeFactor: 3.0 ,
              maxInterval: 3600*24*7, // At least every week
              completion: None
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    seed: TimingOpts,
    consolidation: TimingOpts,
    retention: TimingOpts,
}

#[derive(Serialize, Deserialize)]
pub struct TimingOpts {
    // How many times the time intervals must increase after successful attempt
    timeFactor: f64,
    // Interval in secs
    maxInterval: u64,
    // How mucs successful attempts we need to complete the stage
    completion: Option<u64>,
}

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
    options: Options,
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
        write(path, s.as_bytes());
        match dirPath.metadata() {
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    create_dir(dirPath) ;
                },
                _ => return Err(PRError::from(e)),
            },
            Ok(meta) => if ! meta.file_type().is_dir() {
                return Err(PRError::HomeDirectoryError("File exists but this is not a directory".to_string())) ;
            }
        } ;
        Ok(())
    }
}
