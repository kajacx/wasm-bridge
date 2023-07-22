use super::*;

pub struct WasiCtx {
    stdout: Box<dyn OutputStream>,
    stderr: Box<dyn OutputStream>,
}

impl WasiCtx {
    pub(crate) fn new(
        stdout: Option<Box<dyn OutputStream>>,
        stderr: Option<Box<dyn OutputStream>>,
    ) -> Self {
        Self {
            stdout: stdout.unwrap_or_else(|| Box::new(voiding_stream())),
            stderr: stderr.unwrap_or_else(|| Box::new(voiding_stream())),
        }
    }

    pub(crate) fn stdout(&mut self) -> &mut dyn OutputStream {
        &mut *self.stdout
    }

    pub(crate) fn stderr(&mut self) -> &mut dyn OutputStream {
        &mut *self.stderr
    }
}
