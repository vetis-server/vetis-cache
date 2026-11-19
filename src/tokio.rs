use cel::Context;
use vetis::{
    Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase, HookPhases},
    matcher::Matcher,
};

use crate::CacheHookConfig;

/// Cache hook
pub struct CacheHook {
    matcher: Option<Box<dyn Matcher>>,
    config: CacheHookConfig,
}

unsafe impl Send for CacheHook {}
unsafe impl Sync for CacheHook {}

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

        CacheHook { matcher, config: config }
    }
}

impl Hook for CacheHook {
    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phases(&self) -> HookPhases {
        HookPhases::new(HookPhase::Rewrite)
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
