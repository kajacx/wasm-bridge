use std::collections::HashMap;

use rand_core::RngCore;

use super::*;
use crate::Result;

pub enum IsATTY {
    Yes,
    No,
}

#[derive(Default)]
pub struct WasiCtxBuilder {
    stdin: Option<Box<dyn InputStream>>,
    stdout: Option<Box<dyn OutputStream>>,
    stderr: Option<Box<dyn OutputStream>>,

    random: Option<SecureRandom>,

    wall_clock: Option<Box<dyn HostWallClock>>,
    monotonic_clock: Option<Box<dyn HostMonotonicClock>>,
    env: HashMap<String, String>,
}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self, _table: &mut Table) -> Result<WasiCtx> {
        let v = std::mem::take(self);

        Ok(WasiCtx::new(
            v.stdin,
            v.stdout,
            v.stderr,
            v.random,
            v.wall_clock,
            v.monotonic_clock,
            v.env,
        ))
    }

    /// *NOTE*: is_atty is currently ignored
    pub fn stdin(&mut self, in_stream: impl InputStream + 'static, _is_atty: IsATTY) -> &mut Self {
        self.stdin = Some(Box::new(in_stream));
        self
    }

    /// *NOTE*: is_atty is currently ignored
    pub fn stdout(&mut self, out: impl OutputStream + 'static, _is_atty: IsATTY) -> &mut Self {
        self.stdout = Some(Box::new(out));
        self
    }

    /// *NOTE*: is_atty is currently ignored
    pub fn stderr(&mut self, err: impl OutputStream + 'static, _is_atty: IsATTY) -> &mut Self {
        self.stderr = Some(Box::new(err));
        self
    }

    pub fn inherit_stdin(&mut self) -> &mut Self {
        // TODO: could be implemented at least on node, but readline is asynchronous
        self
    }

    pub fn inherit_stdout(&mut self) -> &mut Self {
        self.stdout = Some(Box::new(console_log_stream()));
        self
    }

    pub fn inherit_stderr(&mut self) -> &mut Self {
        self.stderr = Some(Box::new(console_error_stream()));
        self
    }

    pub fn inherit_stdio(&mut self) -> &mut Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }

    pub fn set_secure_random(&mut self) -> &mut Self {
        self.random = None;
        self
    }

    pub fn set_secure_random_to_custom_generator(
        &mut self,
        random: impl RngCore + Send + Sync + 'static,
    ) -> &mut Self {
        self.random = Some(Box::new(random));
        self
    }

    pub fn set_wall_clock(&mut self, wall_clock: impl HostWallClock + 'static) -> &mut Self {
        self.wall_clock = Some(Box::new(wall_clock));
        self
    }

    pub fn set_monotonic_clock(
        &mut self,
        monotonic_clock: impl HostMonotonicClock + 'static,
    ) -> &mut Self {
        self.monotonic_clock = Some(Box::new(monotonic_clock));
        self
    }

    pub fn set_env(mut self, env: &[(impl AsRef<str>, impl AsRef<str>)]) -> Self {
        self.env = env
            .iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect();
        self
    }
}
