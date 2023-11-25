use {
    anyhow::{anyhow, bail, Result},
    bonsaidb_local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
    bonsaidb_files::{BonsaiFiles, FileConfig, FilesSchema},
    super::note_api,
    std::path::{Path, PathBuf},
};

pub(crate) struct Environment(pub PathBuf);

impl Environment {
    pub fn absolute<S: AsRef<Path>>(path: S) -> Self {
        let path = path.as_ref().to_owned();
        if !path.exists() {
            std::fs::create_dir(&path);
        }
        Self(path)
    }

    pub fn new<S: AsRef<Path>>(name: S) -> Self {
        let path = name.as_ref().to_path_buf();
        if !path.exists() {
            std::fs::create_dir(&path);
        }
        Self(path)
    }
}

impl AsRef<Path> for Environment {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

pub fn create_environment<P: AsRef<Path>>(path: P) -> Result<Environment> {
    Ok(Environment::new(path))
}

pub struct Backend {
    pub db: Database,
}

pub fn get_or_init<P: AsRef<Path>>(path: P) -> Result<Backend> {
    if let Ok(dir) = create_environment(path) {
        let db = Database::open::<FilesSchema>(StorageConfiguration::new(dir)).unwrap();
        Ok(Backend { db })
    } else {
        Err(anyhow!("Unrecoverable error"))
    }
}
