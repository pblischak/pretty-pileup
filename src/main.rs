use std::io::ErrorKind;

use pretty_pileup::exits::{RUNTIME_ERROR, SUCCESS};
use pretty_pileup::pileup::run;

fn main() {
    use std::process::exit;

    // Needed to deal with broken pipe error: ie ending a pipe
    // to `less` before the file is completely read
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    match run() {
        Err(err) if err.kind() == ErrorKind::BrokenPipe => {
            // This happens when the output is piped to a program like `less`
            exit(SUCCESS)
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(RUNTIME_ERROR)
        }
        Ok(exit_status) => exit(exit_status),
    }
}
