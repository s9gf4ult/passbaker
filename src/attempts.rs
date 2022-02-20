use serde::{Serialize, Deserialize} ;
use chrono::{
    prelude::*, Duration,
} ;
use std:: {
    path::{PathBuf},
    fs::*,
} ;
use csv::{WriterBuilder} ;
use crate::{
    err::*,
    aux::*,
    options::*,
} ;


pub struct PasswordAttempts (pub Vec<Box<PassAttempt>>) ;

#[derive(Serialize, Deserialize)]
pub struct PassAttempt {
    pub timestamp: DateTime<Utc>,
    pub result: AttemptResult,
}

#[derive(Serialize, Deserialize)]
pub enum AttemptResult {
    Success,
    Miss
}

impl PasswordAttempts {
    pub fn register_attempt(&mut self, dir: &PathBuf, item: Box<PassAttempt>) -> Result<(), PRError> {
        let filename = {
            dir_exists(dir)? ;
            let date_str = item.timestamp.date().to_string() ;
            let f = date_str + ".csv" ;
            dir.join(f)
        } ;
        let file = OpenOptions::new()
            .read(false)
            .create(true)
            .write(true)
            .truncate(false)
            .append(true)
            .open(&filename)? ;
        let mut writer = WriterBuilder::new()
            .has_headers(false).from_writer(&file) ;
        writer.serialize(&item)? ;
        writer.flush()? ;
        self.0.push(item) ;
        Ok(())
    }

    pub fn next_attempt(
        &self,
        created: &DateTime<Utc>,
        opts: &Options
    ) -> Result<(Stage, DateTime<Utc>), PRError> {
        let mut duration = Duration::seconds(opts.initial.try_into()?) ;
        let mut res = *created + duration ;
        let mut stage = Stage::Seed ;
        let mut successful: u64 = 0 ; // Successful attempts count
        let seed_complete = opts.seed.completion ;
        let cons_complete = seed_complete.and_then(|seed| {
            opts.consolidation.completion.and_then(|cons| { Some(seed + cons) })
        }) ;

        for attempt in &self.0 {
            if let AttemptResult::Success = &attempt.result {
                successful += 1 ;
            };
            match &stage {
                Stage::Seed => match seed_complete {
                    Some(complete) if successful >= complete => {
                        stage = Stage::Consolidate ;
                    },
                    _ => (),
                },
                Stage::Consolidate => match cons_complete {
                    Some(complete) if successful >= complete => {
                        stage = Stage::Retent ;
                    },
                    _ => (),
                },
                _ => (),
            } ;
            let timings = opts.stage_timings(&stage) ;
            let factor = match &attempt.result {
                AttemptResult::Success => timings.succ_factor,
                AttemptResult::Miss =>    timings.miss_factor,
            } ;
            let prev_secs = duration.num_seconds() as f64 ;
            let new_secs: i64 = timings.max_interval.min(
                (factor * prev_secs).round() as u64
            ).try_into()? ;
            duration = Duration::seconds(new_secs) ;
            res = attempt.timestamp + duration ;
        };
        Ok((stage, res))
    }
}
