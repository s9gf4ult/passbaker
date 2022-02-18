use serde:: {
    *,
} ;
use chrono::{
    prelude::*,
} ;
use std:: {
    path::{PathBuf},
    todo,
} ;
use csv::Writer;
use crate::{
    err::*,
    aux::*,
    options::*,
} ;


pub struct PasswordAttempts (pub Vec<Box<PassAttempt>>) ;

#[derive(Serialize, Deserialize)]
pub struct PassAttempt {
    timestamp: DateTime<Utc>,
    result: AttemptResult,
}

#[derive(Serialize, Deserialize)]
pub enum AttemptResult {
    Success,
    Miss
}

impl PasswordAttempts {
    fn registerAttempt(&mut self, dir: &PathBuf, item: Box<PassAttempt>) -> Result<(), PRError> {
        let filename: Result<PathBuf, PRError> = {
            dirExists(dir)? ;
            let dateStr = item.timestamp.date().to_string() ;
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

    fn nextAttempt(
        &self,
        created: DateTime<Utc>,
        opts: &Options
    ) -> Result<DateTime<Utc>, PRError> {
        todo!()
    }
}
