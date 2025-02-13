use super::{KvsEngine, KvsResult};
use crate::KvsErrors;
use sled::Db;
use std::path::PathBuf;

// Create our sled-based implementation
#[derive(Clone)]
pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    pub fn new(path: impl Into<PathBuf>) -> KvsResult<Self> {
        let db = sled::open(path.into())?;
        Ok(SledKvsEngine { db })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> KvsResult<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> KvsResult<Option<String>> {
        Ok(self
            .db
            .get(key.as_bytes())?
            .map(|i| String::from_utf8(i.to_vec()).unwrap()))
    }

    fn remove(&mut self, key: String) -> KvsResult<()> {
        if self.db.remove(key.clone())?.is_some() {
            self.db.flush()?;
            Ok(())
        } else {
            Err(KvsErrors::KeyNotFound())
        }
    }
}
