#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::tokio::CacheHook;
use serde::{Deserialize, Serialize};
use vetis::{
    Str, VetisResult,
    errors::{ConfigError, VetisError},
};

#[cfg(feature = "runtime-tokio")]
/// Tokio runtime support module
pub mod tokio;

#[cfg(test)]
mod tests;

/// Builder for creating `CacheHookConfig` instances.
pub struct CacheHookConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
}

impl CacheHookConfigBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + Send + Sync + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Build the `CacheHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<CacheHookConfig>` - The `CacheHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<CacheHookConfig> {
        Ok(CacheHookConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
        })
    }
}

/// Configuration for static file serving.
#[derive(Clone, Serialize, Deserialize)]
pub struct CacheHookConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
}

#[typetag::serde(name = "cache")]
impl HookConfig for CacheHookConfig {
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    fn boxed_clone(&self) -> Box<dyn HookConfig> {
        Box::new(self.clone())
    }

    fn boxed_hook(&self) -> Box<dyn VetisHook> {
        Box::new(CacheHook::new(self.clone()))
    }
}

impl CacheHookConfig {
    /// Allow create a new `CacheHookConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `CacheHookConfigBuilder` - The builder.
    pub fn builder() -> CacheHookConfigBuilder {
        CacheHookConfigBuilder { matcher: None }
    }
}
