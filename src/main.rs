mod passRecord ;
mod cli ;
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
fn main() {
    use cli::{
        Cli, Parser
    } ;
    let args = {
        Cli::parse ()
    } ;

    let name = "Hopacha" ;
    match args {
        Cli::New {name: newname} => println!("yoba {}, {}", name, newname),
        Cli::Repeat {..} => println!("boba"),
    }
}
