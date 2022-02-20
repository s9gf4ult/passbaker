#![deny(bindings_with_variant_name)]

mod pass_record ;
mod cli ;
mod options ;
mod err ;
mod header ;
mod aux ;
mod attempts ;

use cli::{
    Cli, Parser
} ;
use pass_record::* ;
use err::PRError ;
use pbkdf2::password_hash::{
    SaltString,
    rand_core::OsRng
} ;
use rpassword::read_password_from_tty ;
use home::home_dir ;

struct ConsoleInter ;

impl Interactor for ConsoleInter {
    fn show_message(&mut self, s: &str) {
        print!("{}\n", s) ;
    }
    fn read_password(&mut self) -> String {
        read_password_from_tty(Some("Password: ")).unwrap()
    }
}

fn main() -> Result<(), PRError> {
    let args = Cli::parse() ;
    match args {
        Cli::New {name} => {
            let mut i = ConsoleInter;
            let salt = SaltString::generate(&mut OsRng) ;
            let home = match home_dir() {
                None => return Err(PRError::HomeDirectoryError("Can not find home dir".to_string())),
                Some(p) => p.join(".passbaker")
            } ;
            let mut pass = PassRecord::init(
                &home,
                &salt,
                &mut i,
                name,
            )? ;
            pass.seed_cycle(&home, &mut i)? ;
            Ok(())
        },
        Cli::Repeat {..} => {
            println!("haha");
            Ok(())
        }
    }
}
