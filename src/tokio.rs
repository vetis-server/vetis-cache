use http::{HeaderMap, HeaderValue};
use hyper_body_utils::HttpBody;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use tokio::fs::File;
use vetis::{
    Request, Response, VetisFutureResult, VetisResult,
    errors::{ContentError, HostError, VetisError},
    host::{HostContext, path::Path},
};

use crate::CacheHookConfig;

/// Cache hook
pub struct CacheHook {
    config: CacheHookConfig,
}

impl CacheHook {
    /// Create a new cache hook with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the cache hook
    ///
    /// # Returns
    ///
    /// * `CacheHook` - The cache hook
    pub fn new(config: CacheHookConfig) -> CacheHook {
        let matcher = if let Some(matcher_config) = config.matcher() {
            Some(matcher_config.boxed_matcher())
        } else {
            None
        };

        EncodeHook { matcher, _config: config }
    }
}

impl Hook for CacheHook {
    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phase(&self) -> &HookPhase {
        &HookPhase::Rewrite
    }

    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>> {
        self.matcher
            .as_mut()
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        _response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)> {
        let future = async move { Ok((request, Some(Response::builder().empty()))) };
        Box::pin(future)
    }
}
