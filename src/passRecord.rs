use pbkdf2 ::{
    Pbkdf2,
    password_hash::{
        rand_core::OsRng,
        SaltString, PasswordHasher, PasswordVerifier, PasswordHash
    },
};
use std::io ;
use chrono::prelude::* ;

struct PassRecord<'a> {
    pub hash: PasswordHash<'a>,
    pub records: Vec<PassEnter>
}

struct PassEnter {
    date: DateTime<Utc>,
    result: PassResult
}

enum PassResult {
    Success,
    Miss
}
