// TODO: Proper trait signatures

mod output_stream;
pub use output_stream::*;

mod input_stream;
pub use input_stream::*;

pub enum StreamStatus {
    Open,
    Ended,
}
impl StreamStatus {
    pub(crate) fn to_variant(&self) -> String {
        match self {
            Self::Open => "open".into(),
            Self::Ended => "ended".into(),
        }
    }
}
