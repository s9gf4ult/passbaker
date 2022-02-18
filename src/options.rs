use serde:: {
    *,
} ;
use std:: {
    default::Default,
} ;


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
