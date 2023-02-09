use std::io;

use ansi_term::{ANSIString, ANSIStrings, Color::RGB, Style};
use clap::Parser;
use phf::phf_map;
use rust_htslib::{bam, bam::Read, faidx};

use crate::{cli::Args, colors::BasePair, config::parse_config, exits::SUCCESS};

// Set up base pair and quality score coloring

static BASE_PAIRS: phf::Map<char, BasePair> = phf_map! {
    'A' => BasePair::BaseA,
    'C' => BasePair::BaseC,
    'G' => BasePair::BaseG,
    'T' => BasePair::BaseT,
    'N' => BasePair::BaseN,
};

fn open_ref_genome(ref_genome: Option<&String>) -> Option<faidx::Reader> {
    ref_genome.map(|path| faidx::Reader::from_path(path).unwrap())
}

pub fn run() -> io::Result<i32> {
    let args = Args::parse();
    let color_theme = parse_config();

    let ref_genome = open_ref_genome(args.fasta.as_ref());

    let mut bam = bam::Reader::from_path(&args.bam_file)
        .unwrap_or_else(|err| panic!("Could not open BAM file {}\n {}", args.bam_file, err));
    let header = bam::Header::from_template(bam.header());
    let mut chromosomes = Vec::<String>::new();

    for (key, records) in header.to_hashmap() {
        for record in records {
            if key == "SQ" {
                chromosomes.push(record["SN"].clone())
            }
        }
    }

    let mut pileup_vec = Vec::<ANSIString>::with_capacity(100);
    let mut qual_vec = Vec::<f32>::with_capacity(100);
    let mut qual_ansi_vec = Vec::<ANSIString>::with_capacity(100);
    for p in bam.pileup() {
        let pileup = p.unwrap();
        let chrom = &chromosomes[pileup.tid() as usize];
        let ref_base = match &ref_genome {
            Some(seq) => seq
                .fetch_seq_string(chrom, pileup.pos() as usize, pileup.pos() as usize)
                .unwrap_or_else(|_| "N".to_string()),
            None => "N".to_string(),
        };

        for alignment in pileup.alignments() {
            if !alignment.is_del() && !alignment.is_refskip() {
                let raw_base = alignment.record().seq()[alignment.qpos().unwrap()] as char;
                let base = BASE_PAIRS.get(&raw_base).unwrap_or(&BasePair::BaseN);
                let qual_char = (alignment.record().qual()[alignment.qpos().unwrap()] + 33) as char;
                let qual_val = alignment.record().qual()[alignment.qpos().unwrap()] as f64;
                let base_color = color_theme.get_base_color(base);
                pileup_vec.push(base_color.paint(raw_base.to_string()));
                qual_ansi_vec.push(
                    Style::new()
                        .on(color_theme
                            .quality_gradient
                            .calc_color_on_gradient(qual_val))
                        .fg(RGB(20, 20, 20))
                        .paint(std::string::String::from(qual_char)),
                );
            }
        }

        let raw_ref_base = ref_base.chars().next().unwrap_or('N');
        let ref_base_color =
            color_theme.get_base_color(BASE_PAIRS.get(&raw_ref_base).unwrap_or(&BasePair::BaseN));
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            chrom,
            pileup.pos() + 1,
            ref_base_color.paint(ref_base),
            pileup.depth(),
            ANSIStrings(&pileup_vec),
            ANSIStrings(&qual_ansi_vec),
        );

        pileup_vec.clear();
        qual_vec.clear();
        qual_ansi_vec.clear();
    }
    Ok(SUCCESS)
}
