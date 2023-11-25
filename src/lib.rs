#![feature(generic_arg_infer)]
#![allow(unused_variables, unused_assignments)]
use {
    anyhow::{bail, anyhow, Result},
};

pub(crate) mod note_api;
pub use note_api::*;

pub(crate) mod editor;
pub(crate) mod backend;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn crud_create() {
        let cipher: Option<&[u8]> = None;
        let note: Result<Note> = create_note::<&[u8]>("test", None);
        match note {
            Ok(_) => assert!(true),
            Err(e) => panic!("{:?}", &e),
        }
    }

    #[test]
    fn crud_create_unnamed() {
        let note: Result<Note> = create_note::<&[u8]>("", None);
        match note {
            Ok(note_ref) => (),
            Err(e) => panic!("{:?}", &e),
        }
    }


    #[test]
    fn crud_create_named_encrypted() {
        let cipher: Option<&[u8]> = Some(b"intentionally bad password");
        let note: Result<Note> = create_note::<&[u8]>("encrypted-test", cipher);
        match note {
            Ok(note_ref) => (),
            Err(e) => panic!("{:?}", &e),
        }
    }


}
