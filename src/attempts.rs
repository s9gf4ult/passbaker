use serde::{Serialize, Deserialize} ;
use chrono::{
    prelude::*, Duration,
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
    fn register_attempt(&mut self, dir: &PathBuf, item: Box<PassAttempt>) -> Result<(), PRError> {
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

    fn next_attempt(
        &self,
        created: DateTime<Utc>,
        opts: &Options
    ) -> Result<(Stage, DateTime<Utc>), PRError> {
        let mut duration = Duration::seconds(opts.initial.try_into()?) ;
        let mut res = created + duration ;
        let mut stage = Stage::Seed ;
        let mut successful: u64 = 0 ; // Successful attempts count
        let seedComplete = opts.seed.completion ;
        let consComplete = seedComplete.and_then(|seed| {
            opts.consolidation.completion.and_then(|cons| { Some(seed + cons) })
        }) ;
        let retentionComplete = consComplete.and_then(|cons| {
            opts.retention.completion.and_then(|ret| { Some(cons + ret)})
        }) ;

        for attempt in &self.0 {
            if let Success = &attempt.result {
                successful += 1 ;
            };
            match &stage {
                Seed => match seedComplete {
                    Some(complete) if successful >= complete => {
                        let stage = Stage::Consolidate ;
                    },
                    _ => (),
                },
                Consolidate => match consComplete {
                    Some(complete) if successful >= complete => {
                        let stage = Stage::Retent ;
                    },
                    _ => (),
                },
                _ => (),
            } ;
            let timings = opts.stage_timings(&stage) ;
            let factor = match &attempt.result {
                Success => timings.succFactor,
                Miss =>    timings.missFactor,
            } ;
            let prevSecs = duration.num_seconds() as f64 ;
            let newSecs: i64 = timings.maxInterval.min(
                (factor * prevSecs).round() as u64
            ).try_into()? ;
            duration = Duration::seconds(newSecs) ;
            res = attempt.timestamp + duration ;
        };
        Ok((stage, res))
    }
}
