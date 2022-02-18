use serde:: {
    *,
} ;
use std:: {
    default::Default,
} ;


#[derive(Serialize, Deserialize)]
pub struct Options {
    pub initial: u64,
    pub seed: TimingOpts,
    pub consolidation: TimingOpts,
    pub retention: TimingOpts,
}

#[derive(Serialize, Deserialize)]
pub struct TimingOpts {
    // How many times the time intervals must increase after successful attempt
    pub succFactor: f64,
    // Same for fail
    pub missFactor: f64,
    // max interval in secs
    pub maxInterval: u64,
    // How mucs successful attempts we need to complete the stage
    pub completion: Option<u64>,
}

pub enum Stage {
    Seed,
    Consolidate,
    Retent,
}

impl Options {
    pub fn stage_timings<'a> (&'a self, stage: &Stage) -> &'a TimingOpts {
        match stage {
            Seed => &self.seed,
            Consolidate => &self.consolidation,
            Retent => &self.retention,
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            initial: 5,
            seed: TimingOpts
            { succFactor: 2.0,
              missFactor: 1.0,
              maxInterval: 3600, // Repeat at least every hour
              completion: Some(10)
            },
            consolidation: TimingOpts
            { succFactor: 2.0,
              missFactor: 1.5,
              maxInterval: 3600*24, // Repeat at least every day
              completion: Some(10),
            },
            retention: TimingOpts
            { succFactor: 3.0,
              missFactor: 2.0,
              maxInterval: 3600*24*7, // At least every week
              completion: None
            },
        }
    }
}
