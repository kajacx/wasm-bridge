use super::*;

pub struct WasiCtx {
    stdin: Box<dyn StdinStream>,
    stdout: Box<dyn StdoutStream>,
    stderr: Box<dyn StdoutStream>,

    random: SecureRandom,
    env_variables: Vec<(String, String)>,

    wall_clock: Box<dyn HostWallClock>,
    monotonic_clock: Box<dyn HostMonotonicClock>,
}

impl WasiCtx {
    pub(crate) fn new(
        stdin: Option<Box<dyn StdinStream>>,
        stdout: Option<Box<dyn StdoutStream>>,
        stderr: Option<Box<dyn StdoutStream>>,
        random: Option<SecureRandom>,
        env_variables: Vec<(String, String)>,
        wall_clock: Option<Box<dyn HostWallClock>>,
        monotonic_clock: Option<Box<dyn HostMonotonicClock>>,
    ) -> Self {
        Self {
            stdin: stdin.unwrap_or_else(|| Box::new(void_stream())),
            stdout: stdout.unwrap_or_else(|| Box::new(voiding_stream())),
            stderr: stderr.unwrap_or_else(|| Box::new(voiding_stream())),
            random: random.unwrap_or_else(js_rand),
            env_variables,
            wall_clock: wall_clock.unwrap_or_else(|| Box::new(real_wall_clock())),
            monotonic_clock: monotonic_clock.unwrap_or_else(|| Box::new(default_monotonic_clock())),
        }
    }

    pub(crate) fn stdin(&self) -> &dyn StdinStream {
        &*self.stdin
    }

    pub(crate) fn stdout(&self) -> &dyn StdoutStream {
        &*self.stdout
    }

    pub(crate) fn stderr(&self) -> &dyn StdoutStream {
        &*self.stderr
    }

    pub(crate) fn random(&mut self) -> &mut dyn rand_core::RngCore {
        &mut *self.random
    }

    pub(crate) fn env_variables(&self) -> &[(String, String)] {
        &self.env_variables
    }

    pub(crate) fn wall_clock(&self) -> &dyn HostWallClock {
        &*self.wall_clock
    }

    pub(crate) fn monotonic_clock(&self) -> &dyn HostMonotonicClock {
        &*self.monotonic_clock
    }
}
