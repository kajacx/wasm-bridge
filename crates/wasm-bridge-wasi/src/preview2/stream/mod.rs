// TODO: Proper trait signatures

mod output_stream;
pub use output_stream::*;

mod input_stream;
pub use input_stream::*;

pub enum StreamError {
    Closed,
    LastOperationFailed(anyhow::Error),
}

pub enum StreamStatus {
    Open,
    Ended,
}
impl StreamStatus {
    pub(crate) fn to_variant(&self) -> u8 {
        match self {
            Self::Open => 0,
            Self::Ended => 1,
        }
    }

    pub(crate) fn from_variant(variant: u8) -> Self {
        match variant {
            0 => Self::Open,
            1 => Self::Ended,
            _ => unreachable!("invalid stream status variant: {variant}"),
        }
    }
}
