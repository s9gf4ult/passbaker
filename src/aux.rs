use std:: {
    io,
    path::{PathBuf},
    fs::*,
} ;

use crate::{
    err::*,
} ;

pub fn dir_exists(dir: &PathBuf) -> Result<(), PRError> {
    match dir.metadata() {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                create_dir(dir)? ;
            },
            _ => return Err(PRError::from(e)),
        },
        Ok(meta) => if ! meta.file_type().is_dir() {
            return Err(PRError::HomeDirectoryError("File exists but this is not a directory".to_string())) ;
        }
    };
    Ok(())
}
