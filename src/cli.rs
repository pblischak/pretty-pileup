use clap::Parser;

// Set up command line argument parsing

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Name of BAM file
    #[arg(short, long)]
    pub bam_file: String,

    /// Reference genome
    #[arg(short, long)]
    pub fasta: Option<String>,

    /// Ploidy level
    #[arg(short, long, default_value_t = 2)]
    pub ploidy: u8,
}
