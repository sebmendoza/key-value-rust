pub mod kv;
pub mod sled;

pub use self::kv::KvStore;
pub use self::sled::SledKvsEngine;
use crate::error::KvsResult;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> KvsResult<()>;

    fn get(&mut self, key: String) -> KvsResult<Option<String>>;

    fn remove(&mut self, key: String) -> KvsResult<()>;
}
