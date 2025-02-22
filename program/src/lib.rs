#[deny(missing_docs)]
pub mod entrypoint;
#[deny(missing_docs)]
pub mod error;
pub mod instruction;
#[deny(missing_docs)]
pub mod payload;
#[deny(missing_docs)]
pub mod pda;
#[deny(missing_docs)]
pub mod processor;
#[deny(missing_docs)]
pub mod state;
#[deny(missing_docs)]
pub mod utils;

pub use safecoin_program;

/// Max name length for any of the names used in this crate.
pub const MAX_NAME_LENGTH: usize = 32;

safecoin_program::declare_id!("authvoCCz9WxYk6ZQH1smw5xFT9zRGJa1QoAHBZ2HBy");
