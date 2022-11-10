# `pretty-pileup`: Pretty Printing of SAMtools Pileups



## Acknowledgements

The following projects were a huge help in writing `prett-pileup`:

 - [`rust_htslib`](https://github.com/rust-bio/rust-htslib.git): This is an
   amazing library that wraps `htslib` and provides an excellent, high-level
   Rust API. I wouldn't have been able to write `pretty-pileup` without it.
 - [`exa`](https://github.com/ogham/exa.git): `exa` is a command line replacement
   for the Unix command `ls` written in Rust. I used its error handling model to
   deal with broken pipes when calls to `pretty-pileup` are piped into `less`
   (or another pager) but are terminated before the full file is read. Rust typically
   panics in these cases unless you handle the broken pipe error correctly and the
   code in `exa` was instrumental in making sure that didn't happen unnecessarily.
