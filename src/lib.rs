//! `pretty-pileup` command-line utility for pretty printing of SAMtools pileups.
//!
//! **Features**:
//!  - Easy inspection of pileups with colored output.
//!  - Fully configurable color themes.
//!  - Seemless integration with pagers such as `less`.
//!
pub mod cli;
pub mod colors;
pub mod config;
pub mod exits;
pub mod pileup;
