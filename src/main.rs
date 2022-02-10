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
use pbkdf2::password_hash::{
    SaltString,
    rand_core::OsRng
} ;
use rpassword::read_password_from_tty ;
use home::home_dir ;

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
            let salt = SaltString::generate(&mut OsRng) ;
            let path = match home_dir() {
                None => return Err(PRError::HomeDirectoryError("Can not find home dir".to_string())),
                Some(p) => p.join(".passbaker")
            } ;
            let mut pass = PassRecord::init(
                &path,
                &salt,
                &asker,
                &notifier,
                name
            )? ;
            pass.seedCyccle() ;
            Ok(())
        },
        Cli::Repeat {..} => {
            println!("haha");
            Ok(())
        }
    }
}
