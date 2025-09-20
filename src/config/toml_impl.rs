use std::path::Path;

use orion_conf::{
    TomlIO,
    error::{ConfIOReason, OrionConfResult},
};
use orion_error::StructError;

use crate::config::backup_clean;

use super::ConfigLifecycle;

impl<T> ConfigLifecycle for T
where
    T: TomlIO<T> + serde::Serialize + serde::de::DeserializeOwned,
{
    fn load(path: &str) -> OrionConfResult<Self>
    where
        Self: Sized,
    {
        T::load_toml(Path::new(path))
    }

    fn init(&self, path: &str) -> OrionConfResult<()>
    where
        Self: Sized,
    {
        Self::safe_clean(path)?;
        self.save(path)
    }

    fn safe_clean(path: &str) -> OrionConfResult<()> {
        backup_clean(path).map_err(|err| StructError::from(ConfIOReason::Other(err.to_string())))
    }

    fn save(&self, path: &str) -> OrionConfResult<()> {
        self.save_toml(Path::new(path))
    }
}
