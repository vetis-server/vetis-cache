use serde::{Deserialize, Serialize};

/// MokaCacheBackendConfigBuilder
pub struct CacheBackendConfigBuilder {
    max_items: usize,
}

impl CacheBackendConfigBuilder {
    /// Allow set max items this cache can hold
    pub fn max_items(mut self, max_items: usize) -> Self {
        self.max_items = max_items;
        self
    }

    /// Build a new cache
    pub fn build(self) -> CacheBackendConfig {
        CacheBackendConfig { max_items: self.max_items }
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// MokaCacheBackendConfig
pub struct CacheBackendConfig {
    max_items: usize,
}

impl Default for CacheBackendConfig {
    fn default() -> Self {
        CacheBackendConfig { max_items: 10000 }
    }
}

impl CacheBackendConfig {
    /// Create a new cache backend config builder
    pub fn builder() -> CacheBackendConfigBuilder {
        CacheBackendConfigBuilder { max_items: 10000 }
    }

    /// Return max item this cache can hold
    pub fn max_items(&self) -> usize {
        self.max_items
    }
}

#[typetag::serde(name = "moka")]
impl vetis::cache::CacheBackendConfig for CacheBackendConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::cache::CacheBackendConfig> {
        Box::new(self.clone())
    }
}
