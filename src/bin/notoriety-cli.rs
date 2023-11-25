use {
    anyhow::{anyhow, bail, Result},    
    argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString
        },
        Argon2
    },
    notoriety_cli::{
        create_note, retrieve_note, update_note,
        Note,
    },
};

fn main() -> Result<(), ()> {
    Ok(())
}
