//! `pretty-pileup` is a command-line utility for pretty printing of
//! [SAMtools pileups](http://www.htslib.org/doc/samtools-mpileup.html).
//!
//! **Features**:
//!  - Easy inspection of pileups with colored output.
//!  - Fully configurable color themes.
//!  - Seemless integration with pagers such as `less`.
//!
//! ## Example
//!
//! ```bash
//! pretty-pileup --bam-file example.bam --fasta ref.fasta | less
//! ```

pub mod cli;
pub mod colors;
mod config;
pub mod exits;
pub mod pileup;
