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
    pub succ_factor: f64,
    // Same for fail
    pub miss_factor: f64,
    // max interval in secs
    pub max_interval: u64,
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
            Stage::Seed => &self.seed,
            Stage::Consolidate => &self.consolidation,
            Stage::Retent => &self.retention,
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            initial: 5,
            seed: TimingOpts
            { succ_factor: 2.0,
              miss_factor: 1.0,
              max_interval: 3600, // Repeat at least every hour
              completion: Some(10)
            },
            consolidation: TimingOpts
            { succ_factor: 2.0,
              miss_factor: 1.5,
              max_interval: 3600*24, // Repeat at least every day
              completion: Some(10),
            },
            retention: TimingOpts
            { succ_factor: 3.0,
              miss_factor: 2.0,
              max_interval: 3600*24*7, // At least every week
              completion: None
            },
        }
    }
}
