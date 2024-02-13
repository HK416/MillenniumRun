pub mod error;
pub mod event;
pub mod shared;
pub mod timer;

pub const APPLICATION_INFORMATION: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/app.info"));
