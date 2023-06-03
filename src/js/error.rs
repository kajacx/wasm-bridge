use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Error;

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        todo!()
    }

    fn description(&self) -> &str {
        todo!()
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
