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
    // let pwd = b"pass" ;
    // let salt = SaltString::generate(&mut OsRng);
    // let hash = Pbkdf2.hash_password(pwd, &salt).unwrap().to_string();
    // println!("{}", hash);
    // let parsed = PasswordHash::new(&hash).unwrap() ;
    // println!("{:?}", &parsed);
    // let mut pwd = String::new() ;
    // let stdin = io::stdin() ;
    // stdin.read_line(&mut pwd).unwrap() ;

    // let res = Pbkdf2.verify_password(pwd.as_bytes(), &parsed) ;

    // println!("{:?}", &res) ;
fn main() -> Result<(), PRError> {
    let args = Cli::parse() ;
    match args {
        Cli::New {name} => {
            let mut pass = PassRecord::init(Path::new("~/.passbaker/"))? ;
            pass.seedCyccle() ;
            Ok(())
        },
        Cli::Repeat {..} => {
            println!("haha");
            Ok(())
        }
    }
}
