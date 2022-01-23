pub use clap::Parser ;


#[derive(Debug)]
#[derive(Parser)]
pub enum Cli {
    New {
        #[clap(help = "Name for new record")]
        name: String
    },
    Repeat {
        #[clap(help = "Choose only one record to repeat if set")]
        name: Option<String>
    }
}
