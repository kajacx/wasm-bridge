#[cfg(all(not(feature = "sys"), not(feature = "js")))]
compile_error!("Enable `sys` feature to run on the desktop, or `js` feature to run on the web.");
#[cfg(all(not(feature = "sys"), not(feature = "js")))]
mod invalid;
#[cfg(all(not(feature = "sys"), not(feature = "js")))]
pub use invalid::*;

#[cfg(all(feature = "sys", feature = "js"))]
compile_error!(
    "Cannot have both `sys` and `js` features enabled at the same time. Please, pick one."
);
#[cfg(all(feature = "sys", feature = "js"))]
mod invalid;
#[cfg(all(feature = "sys", feature = "js"))]
pub use invalid::*;

#[cfg(all(feature = "sys", not(feature = "js")))]
mod sys;
#[cfg(all(feature = "sys", not(feature = "js")))]
pub use sys::*;

#[cfg(all(feature = "js", not(feature = "sys")))]
mod js;
#[cfg(all(feature = "js", not(feature = "sys")))]
pub use js::*;
