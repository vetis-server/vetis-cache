use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MokaCacheBackendConfigBuilder
pub struct CacheBackendConfigBuilder {
    base_dir: PathBuf,
}

impl CacheBackendConfigBuilder {
    /// Allow set max items this cache can hold
    pub fn base_dir(mut self, base_dir: impl Into<PathBuf>) -> Self {
        self.base_dir = base_dir.into();
        self
    }

    /// Build a new cache
    pub fn build(self) -> CacheBackendConfig {
        CacheBackendConfig { base_dir: self.base_dir }
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// MokaCacheBackendConfig
pub struct CacheBackendConfig {
    base_dir: PathBuf,
}

impl Default for CacheBackendConfig {
    fn default() -> Self {
        CacheBackendConfig { base_dir: "/tmp".into() }
    }
}

impl CacheBackendConfig {
    /// Create a new cache backend config builder
    pub fn builder() -> CacheBackendConfigBuilder {
        CacheBackendConfigBuilder { base_dir: "/tmp".into() }
    }

    /// Return max item this cache can hold
    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }
}

#[typetag::serde(name = "filesystem")]
impl vetis::cache::CacheBackendConfig for CacheBackendConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::cache::CacheBackendConfig> {
        Box::new(self.clone())
    }
}
