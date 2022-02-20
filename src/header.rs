use pbkdf2::Pbkdf2 ;
use serde:: {
    *, de, de::Visitor
} ;
use core::fmt ;
use password_hash:: {
    PasswordHash, PasswordVerifier, errors,
} ;
use chrono::{
    prelude::*,
} ;
use std:: {
    io,
    path::{PathBuf},
    fs::*,
} ;
use crate::{
    options::*,
    err::*,
    aux::*,
} ;


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

#[derive(Serialize, Deserialize)]
pub struct PassHeader<'a> {
    pub name: String,
    pub created: DateTime<Utc>,
    #[serde(serialize_with = "password_hash_serialize")]
    #[serde(deserialize_with = "password_hash_deserialize", borrow)]
    pub hash: PasswordHash<'a>,
    pub options: Options,
}

impl <'a> PassHeader<'a> {
    fn configFileName(&self, dir: &PathBuf) -> PathBuf {
        dir.join( &(self.name.clone() + ".toml") )
    }

    pub fn attemptsDirName(&self, dir: &PathBuf) -> PathBuf {
        dir.join( &self.name )
    }

    pub fn check_pass(&self, s: &str) -> Result<bool, PRError> {
        match Pbkdf2.verify_password(s.as_bytes(), &self.hash) {
            Ok(()) => Ok(true),
            Err(errors::Error::Password) => Ok(false),
            Err(e) => Err(PRError::from(e)),
        }
    }

    pub fn createConfigs(&self, dir: &PathBuf) -> Result<(), PRError> {
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
