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
    fn showMessage(&self, s: &str) {
        print!("{}\n", s) ;
    }
    fn readPassword(&self) -> String {
        read_password_from_tty(Some("Password: ")).unwrap()
    }
}

fn main() -> Result<(), PRError> {
    let args = Cli::parse() ;
    match args {
        Cli::New {name} => {
            let i = ConsoleInter;
            let salt = SaltString::generate(&mut OsRng) ;
            let home = match home_dir() {
                None => return Err(PRError::HomeDirectoryError("Can not find home dir".to_string())),
                Some(p) => p.join(".passbaker")
            } ;
            let mut pass = PassRecord::init(
                &home,
                &salt,
                &i,
                name,
            )? ;
            pass.seedCyccle(&home, &i)? ;
            Ok(())
        },
        Cli::Repeat {..} => {
            println!("haha");
            Ok(())
        }
    }
}
