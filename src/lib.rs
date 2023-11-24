#![feature(generic_arg_infer)]
#![allow(unused_variables, unused_assignments)]
use {
    anyhow::Result,
};

// [`Note`]() is a programming interface providing ways to read and
// edit the note "file" this library creates.
pub struct Note {
}

// First-edition releases on [`Note`]() implementations will provide 
// simple methods to store and retrieve the document. Future releases 
// will support exporting this logic to a trait so that different editors,
// alternative data storage strategies, etc. can be used.
impl Note {

}

// Read-only handle to a document
pub struct NoteRef<'a>(&'a Note);

// Exclusive handle to a document
pub struct NoteRefMut<'a>(&'a mut Note);

// Queries the provider to create a new document with `name` and `cipher`, returning
// successfully when the note did not exist. Duplicate names are not supported however
// duplicate ciphers may be used.
//
// Note the lifetime bound 'a implies the reference to the cipher key
// has to live as long as the Note<'a> it is indirectly bound to.
pub fn create_note<'a, T: AsRef<[u8]> + Sized>(
    name: &str, 
    cipher: Option<T>
) -> Result<NoteRef<'a>> {
    todo!("Create a note titled `name` in an existing db");
}

// Queries the provider to collect the note with the title [`name`](). If one does not exist,
// we transparently report that there was an error. If it does, then we optionally return a 
// [`NoteRefMut`]() provided it does not require decryption and a cipher was not provided, or
// that it did need to be decrypted and the correct cipher key was supplied. If decryption fails
// on a known document, the return type is Ok<None>.
pub fn retrieve_note<'a>(name: &str, cipher: Option<&[u8]>) -> Option<Result<NoteRefMut<'a>>> {
    todo!("Get a note that exists under a given name");
}

// Update the note with latest changes, returning errors for an invalid note reference.
pub fn update_note<'a>(note: &mut NoteRefMut<'a>, data: &[u8]) -> Result<()> {
    todo!("Call Note.save with supplied buffer via `new`");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn crud_create() {
        let cipher: Option<&[u8]> = None;
        let note: Result<NoteRef> = create_note::<&[u8]>("test", None);
        match note {
            Ok(_) => (),
            Err(e) => panic!("{:?}", &e),
        }
    }

    #[test]
    fn crud_create_unnamed() {
        let note: Result<NoteRef> = create_note::<&[u8]>("", None);
        match note {
            Ok(note_ref) => (),
            Err(e) => panic!("{:?}", &e),
        }
    }


    #[test]
    fn crud_create_named_encrypted() {
        let cipher: Option<&[u8]> = Some(b"intentionally bad password");
        let note: Result<NoteRef> = create_note::<&[u8]>("encrypted-test", cipher);
        match note {
            Ok(note_ref) => (),
            Err(e) => panic!("{:?}", &e),
        }
    }


}
