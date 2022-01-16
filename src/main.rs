use pbkdf2 ::{
    Pbkdf2,
    password_hash::{
        rand_core::OsRng,
        SaltString, PasswordHasher, PasswordVerifier, PasswordHash
    },
};
use std::io ;


fn main() {
    let pwd = b"pass\n" ;
    let salt = SaltString::generate(&mut OsRng);
    let hash = Pbkdf2.hash_password(pwd, &salt).unwrap().to_string();
    println!("{}", hash);
    let parsed = PasswordHash::new(&hash).unwrap() ;
    println!("{:?}", &parsed);
    let mut pwd = String::new() ;
    let mut stdin = io::stdin() ;
    stdin.read_line(&mut pwd).unwrap() ;

    let res = Pbkdf2.verify_password(pwd.as_bytes(), &parsed) ;

    println!("{:?}", &res) ;
}
