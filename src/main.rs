#![deny(bindings_with_variant_name)]

mod attempts;
mod aux;
mod cli;
mod err;
mod header;
mod options;
mod pass_record;
use home::home_dir;
use pbkdf2::password_hash::{rand_core::OsRng, SaltString};
use rpassword::read_password_from_tty;

use crate::{
    cli::{Cli, Parser},
    err::PRError,
    pass_record::*,
};

struct ConsoleInter;

impl Interactor for ConsoleInter {
    fn show_message(&mut self, s: &str) {
        print!("{}\n", s);
    }
    fn read_password(&mut self) -> String {
        read_password_from_tty(Some("Password: ")).unwrap()
    }
}

fn main() -> Result<(), PRError> {
    let args = Cli::parse();
    match args {
        Cli::New { name } => {
            let mut i = ConsoleInter;
            let salt = SaltString::generate(&mut OsRng);
            let home = match home_dir() {
                None => {
                    return Err(PRError::HomeDirectoryError(
                        "Can not find home dir".to_string(),
                    ))
                }
                Some(p) => p.join(".passbaker"),
            };
            let mut pass = PassRecord::init(&home, &salt, &mut i, name)?;
            pass.seed_cycle(&home, &mut i)?;
            Ok(())
        }
        Cli::Repeat { .. } => {
            println!("haha");
            Ok(())
        }
    }
}
