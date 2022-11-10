use std::io::{self, ErrorKind};

use clap::Parser;
use phf::phf_map;

// use ansi_term::Color::RGB;
use ansi_term::{
    ANSIString, ANSIStrings, Color::{self, RGB}, Style
};

use rust_htslib::{bam, bam::Read, faidx};

// Set up base pair and quality score coloring

static BASE_COLORS: phf::Map<char, Color> = phf_map! {
    'A' => RGB(255, 0, 0),
    'C' => RGB(0, 255, 0),
    'G' => RGB(80, 80, 255),
    'T' => RGB(255, 255, 0),
    'N' => RGB(100, 100, 100),
};

fn calc_color_gradient(qual_val: f32) -> Color {
    let ratio = 255.0 / 60.0;
    let red: u8 = (255.0 - ratio * qual_val) as u8;
    let green: u8 = (ratio * qual_val) as u8;
    
    RGB(red, green, 80)
}

// Set up command line argument parsing

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Name of BAM file
    #[arg(short, long)]
    bam_file: String,


    /// Reference genome
    #[arg(short, long)]
    fasta: Option<String>,

    /// Ploidy level
    #[arg(short, long, default_value_t = 2)]
    ploidy: u8,
}

fn open_ref_genome(ref_genome: Option<&String>) -> Option<faidx::Reader> {
    ref_genome.map(|path| faidx::Reader::from_path(path).unwrap())
}

// Main 

fn run() -> io::Result<i32> {
    let args = Args::parse();

    let ref_genome = open_ref_genome(args.fasta.as_ref());

    let mut bam = bam::Reader::from_path(&args.bam_file).unwrap_or_else(
        |err| panic!("Could not open BAM file {}\n {}", args.bam_file, err)
    );
    let header = bam::Header::from_template(bam.header());
    let mut chromosomes = Vec::<String>::new();

    for (key, records) in header.to_hashmap() {
        for record in records {
            if key == "SQ" {
                chromosomes.push(record["SN"].clone())
            }
        }
    }
    
    let mut pileup_vec = Vec::<ANSIString>::new();
    let mut qual_vec = Vec::<f32>::new();
    let mut qual_ansi_vec = Vec::<ANSIString>::new();
    for p in bam.pileup() {
        let pileup = p.unwrap();
        let chrom = &chromosomes[pileup.tid() as usize];
        let ref_base = match &ref_genome {
            Some(seq) => seq.fetch_seq_string(
                chrom,
                pileup.pos() as usize,
                pileup.pos() as usize
            ).unwrap_or_else(|_| "N".to_string()),
            None => "N".to_string(),
        };
        
        for alignment in pileup.alignments() {
            if !alignment.is_del() && !alignment.is_refskip() {
                let base = alignment.record().seq()[
                    alignment.qpos().unwrap()
                ] as char;
                let qual_char = (
                    alignment.record().qual()[
                        alignment.qpos().unwrap()
                    ] + 33
                ) as char;
                let qual_val = alignment.record().qual()[
                    alignment.qpos().unwrap()
                ] as f32;
                pileup_vec.push(
                    BASE_COLORS.get(&base)
                    .unwrap()
                    .paint(std::string::String::from(base))
                );
                qual_ansi_vec.push(
                    Style::new().on(calc_color_gradient(qual_val))
                        .fg(RGB(20, 20, 20))
                        .paint(std::string::String::from(qual_char))
                );
            }
        }
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            chrom,
            pileup.pos() + 1,
            BASE_COLORS
                .get(&ref_base.chars().next().unwrap())
                .unwrap()
                .paint(ref_base),
            pileup.depth(),
            ANSIStrings(&pileup_vec),
            ANSIStrings(&qual_ansi_vec),
        );

        // for ch in &pileup_vec {
        //     print!("{0: <20}",
        //         BASE_COLORS
        //             .get(ch)
        //             .unwrap()
        //             .paint(std::string::String::from(*ch))
        //     );
        // }
        // print!("\t");

        // for ch in &qual_ansi_vec {
        //     print!("{0: <20}",
        //     Style::new().on(RGB(255, 100, 100))
        //         .fg(RGB(200, 200, 200))
        //         .paint(std::string::String::from(*ch))
        // );
        // }
        // println!();
        pileup_vec.clear();
        qual_vec.clear();
        qual_ansi_vec.clear();

    }
    Ok(exits::SUCCESS)
}

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
            exit(exits::SUCCESS)
        },
        Err(err) => {
            eprintln!("{}", err);
            exit(exits::RUNTIME_ERROR)
        },
        Ok(exit_status) => exit(exit_status),
    }
}

/* Modeled after exa */
mod exits {
    /// Exit code for when pretty-pileup runs OK.
    pub const SUCCESS: i32 = 0;

    /// Exit code for when there was at least one I/O error during execution.
    pub const RUNTIME_ERROR: i32 = 1;

    /// Exit code for when the command-line options are invalid.
    pub const OPTIONS_ERROR: i32 = 3;
}

/*
use rust_htslib::{bam, bam::Read};

let mut bam = bam::Reader::from_path(&"test/test.bam").unwrap();

// pileup over all covered sites
for p in bam.pileup() {
    let pileup = p.unwrap();
    println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());

    for alignment in pileup.alignments() {
        if !alignment.is_del() && !alignment.is_refskip() {
            println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()]);
        }
        // mark indel start
        match alignment.indel() {
            bam::pileup::Indel::Ins(len) => println!("Insertion of length {} between this and next position.", len),
            bam::pileup::Indel::Del(len) => println!("Deletion of length {} between this and next position.", len),
            bam::pileup::Indel::None => ()
        }
    }
}
*/