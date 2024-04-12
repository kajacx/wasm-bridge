use rand_core::RngCore;

use super::*;

#[derive(Default)]
pub struct WasiCtxBuilder {
    stdin: Option<Box<dyn StdinStream>>,
    stdout: Option<Box<dyn StdoutStream>>,
    stderr: Option<Box<dyn StdoutStream>>,

    random: Option<SecureRandom>,
    env_variables: Vec<(String, String)>,

    wall_clock: Option<Box<dyn HostWallClock>>,
    monotonic_clock: Option<Box<dyn HostMonotonicClock>>,
}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> WasiCtx {
        WasiCtx::new(
            self.stdin,
            self.stdout,
            self.stderr,
            self.random,
            self.env_variables,
            self.wall_clock,
            self.monotonic_clock,
        )
    }

    pub fn stdin(self, in_stream: impl StdinStream + 'static) -> Self {
        Self {
            stdin: Some(Box::new(in_stream)),
            ..self
        }
    }

    pub fn stdout(self, out: impl StdoutStream + 'static) -> Self {
        Self {
            stdout: Some(Box::new(out)),
            ..self
        }
    }

    pub fn stderr(self, err: impl StdoutStream + 'static) -> Self {
        Self {
            stderr: Some(Box::new(err)),
            ..self
        }
    }

    pub fn inherit_stdin(self) -> Self {
        // TODO: could be implemented at least on node, but readline is asynchronous
        self
    }

    pub fn inherit_stdout(self) -> Self {
        Self {
            stdout: Some(Box::new(console_log_stream())),
            ..self
        }
    }

    pub fn inherit_stderr(self) -> Self {
        Self {
            stderr: Some(Box::new(console_error_stream())),
            ..self
        }
    }

    pub fn inherit_stdio(self) -> Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }

    pub fn secure_random(self, random: impl RngCore + Send + Sync + 'static) -> Self {
        Self {
            random: Some(Box::new(random)),
            ..self
        }
    }

    pub fn env(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.env_variables
            .push((name.as_ref().to_string(), value.as_ref().to_string()));
        self
    }

    pub fn envs(mut self, envs: &[(impl AsRef<str>, impl AsRef<str>)]) -> Self {
        for (name, value) in envs {
            self = self.env(name, value);
        }
        self
    }

    pub fn wall_clock(self, wall_clock: impl HostWallClock + 'static) -> Self {
        Self {
            wall_clock: Some(Box::new(wall_clock)),
            ..self
        }
    }

    pub fn monotonic_clock(self, monotonic_clock: impl HostMonotonicClock + 'static) -> Self {
        Self {
            monotonic_clock: Some(Box::new(monotonic_clock)),
            ..self
        }
    }
}
