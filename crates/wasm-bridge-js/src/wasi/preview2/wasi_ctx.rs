use super::*;

pub struct WasiCtx {
    stdin: Box<dyn InputStream>,
    stdout: Box<dyn OutputStream>,
    stderr: Box<dyn OutputStream>,
}

impl WasiCtx {
    pub(crate) fn new(
        stdin: Option<Box<dyn InputStream>>,
        stdout: Option<Box<dyn OutputStream>>,
        stderr: Option<Box<dyn OutputStream>>,
    ) -> Self {
        Self {
            stdin: stdin.unwrap_or_else(|| Box::new(void_stream())),
            stdout: stdout.unwrap_or_else(|| Box::new(voiding_stream())),
            stderr: stderr.unwrap_or_else(|| Box::new(voiding_stream())),
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
}
