mod pass_record ;
mod cli ;

use cli::{
    Cli, Parser
} ;
use pass_record::{
    PassRecord, PRError
};
use std::{
    path::Path
} ;

use rpassword::read_password_from_tty ;

fn notifier(s: &str) {
    print!("{}\n", s);
}

fn main() -> Result<(), PRError> {
    let args = Cli::parse() ;
    match args {
        Cli::New {name} => {
            let asker = || {
                read_password_from_tty(Some("Password: ")).unwrap()
            } ;
            let mut pass = PassRecord::init(Path::new("/home/razor/.passbaker/"), &asker, &notifier, name)? ;
            pass.seedCyccle() ;
            Ok(())
        },
        Cli::Repeat {..} => {
            println!("haha");
            Ok(())
        }
    }
}
