use super::*;

pub struct WasiCtx {
    stdin: Box<dyn InputStream>,
    stdout: Box<dyn OutputStream>,
    stderr: Box<dyn OutputStream>,

    random: SecureRandom,

    wall_clock: Box<dyn HostWallClock>,
    monotonic_clock: Box<dyn HostMonotonicClock>,
}

impl WasiCtx {
    pub(crate) fn new(
        stdin: Option<Box<dyn InputStream>>,
        stdout: Option<Box<dyn OutputStream>>,
        stderr: Option<Box<dyn OutputStream>>,
        random: Option<SecureRandom>,
        wall_clock: Option<Box<dyn HostWallClock>>,
        monotonic_clock: Option<Box<dyn HostMonotonicClock>>,
    ) -> Self {
        Self {
            stdin: stdin.unwrap_or_else(|| Box::new(void_stream())),
            stdout: stdout.unwrap_or_else(|| Box::new(voiding_stream())),
            stderr: stderr.unwrap_or_else(|| Box::new(voiding_stream())),
            random: random.unwrap_or_else(js_rand),
            wall_clock: wall_clock.unwrap_or_else(|| Box::new(real_wall_clock())),
            monotonic_clock: monotonic_clock.unwrap_or_else(|| Box::new(default_monotonic_clock())),
        }
    }

    pub(crate) fn stdin(&mut self) -> &mut dyn InputStream {
        &mut *self.stdin
    }

    pub(crate) fn stdout(&mut self) -> &mut dyn OutputStream {
        &mut *self.stdout
    }

    pub(crate) fn stderr(&mut self) -> &mut dyn OutputStream {
        &mut *self.stderr
    }

    pub(crate) fn random(&mut self) -> &mut dyn rand_core::RngCore {
        &mut *self.random
    }

    pub(crate) fn wall_clock(&self) -> &dyn HostWallClock {
        &*self.wall_clock
    }

    pub(crate) fn monotonic_clock(&self) -> &dyn HostMonotonicClock {
        &*self.monotonic_clock
    }
}
