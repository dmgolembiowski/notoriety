#![feature(generic_arg_infer)]
#![allow(unused_variables, unused_assignments)]
use {
    anyhow::{bail, anyhow, Result},
    std::cell::RefCell,
    std::collections::BTreeMap,
    std::sync::Mutex,
};


// [`Note`]() is a programming interface providing ways to read and
// edit the note "file" this library creates.
#[derive(Clone)]
pub(crate) struct RawNote {
    cryptkind: Encryption,
    name: String,
    content: Option<Vec<u8>>,
    cipher: Option<Vec<u8>>,
}

impl RawNote {
    pub(crate) fn new<Content: AsRef<[u8]> + Sized>(
        cryptkind: Encryption,
        name: impl AsRef<str>, 
        content: Option<Content>, 
    ) -> Self {
        
        const STARTER_NOTE_SIZE: usize = 4096;
        let cipher: Option<Vec<u8>> = None;
        let cryptkind = Encryption::Unspecified;    
        let content: Option<Vec<u8>> = if let Some(inner) = content {
            Some(Vec::from(inner.as_ref()))
        } else {
            Some(Vec::with_capacity(STARTER_NOTE_SIZE))
        };

        let name = name.as_ref().to_string();

        Self { cryptkind, name, content, cipher }
    }
}

#[derive(Clone, Debug)]
pub enum Encryption {
    Unencrypted,
    Encrypted,
    Unspecified,
}

#[derive(Clone)]
pub struct Note {
    raw: Option<RefCell<RawNote>>,
}

impl Default for Note {
    fn default() -> Self {
        let raw: Option<RefCell<RawNote>> = None;
        Note { raw }
    }
}

// First-edition releases on [`Note`]() implementations will provide 
// simple methods to store and retrieve the document. Future releases 
// will support exporting this logic to a trait so that different editors,
// alternative data storage strategies, etc. can be used.
impl Note {
    // Creates blank in-memory copies of the note
    pub(crate) fn new(raw: Option<RawNote>) -> Self {
        let raw = if let Some(raw) = raw { Some(RefCell::new(raw)) } else { None };
        Note { raw }
    }
    // Library-facing constructor
    pub(crate) fn init<Content: AsRef<[u8]> + Sized>(
        name: impl Into<String>, content: Option<Content>, cryptkind: Encryption) -> Self 
    {            
        let name = name.into();
        let raw = Some(RefCell::new(RawNote::new(cryptkind, &name, content)));
        Self { raw }
    }
}
// Queries the provider to create a new document with `name` and `cipher`, returning
// successfully when the note did not exist. Duplicate names are not supported however
// duplicate ciphers may be used.
//
// Note the lifetime bound 'a implies the reference to the cipher key
// has to live as long as the Note<'a> it is indirectly bound to.
pub fn create_note<'a, T: AsRef<[u8]> + Sized>(
    name: impl Into<String>, 
    cipher: Option<T>
) -> Result<Note> {
    let cryptkind = Encryption::Unspecified;
    let name: String = name.into().clone();
    let content: Option<&[u8]> = None;
    Ok(Note::init(name.clone(), content, cryptkind))
}

// Queries the provider to collect the note with the title [`name`](). If one does not exist,
// we transparently report that there was an error. If it does, then we optionally return a 
// [`NoteRefMut`]() provided it does not require decryption and a cipher was not provided, or
// that it did need to be decrypted and the correct cipher key was supplied. If decryption fails
// on a known document, the return type is Ok<None>.
pub fn retrieve_note<'a>(name: &str, cipher: Option<&[u8]>) -> Option<Result<Note>> {
    todo!("Get a note that exists under a given name");
}

// Update the note with latest changes, returning errors for an invalid note reference.
pub fn update_note<'a>(note: &mut Note, data: &[u8]) -> Result<()> {
    todo!("Call Note.save with supplied buffer via `new`");
}

