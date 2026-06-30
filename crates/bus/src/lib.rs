//! ## Melon Bus
//!
//! The bus through which all components chat.

mod bus;
pub use bus::Bus;

pub mod event;
pub use event::Event;

// FIXME: Rationalize and move to correct component
mod score;
pub use score::Score;
