use std::path::PathBuf;

use bio::io::fasta;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the fasta file
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,

    /// Chunk size
    #[arg(short, long, default_value = "1000")]
    chunks: u32,

    /// Index name
    #[arg(short, long, default_value = "sequences")]
    index: String
}

fn main() {
    let cli = Cli::parse();

    let reader = fasta::Reader::from_file(cli.path).unwrap();

    for record in reader.records() {
        let rec = record.expect("Couldn't parse entry: {}");
        let mut offset = 0;
        rec.seq().chunks(1000).for_each(|chunk| {
            println!("{}",
                serde_json::json!({
                    "index": {
                        "_index": cli.index,
                    }
                })
            );
            println!(
                "{}",
                serde_json::json!({
                    "identifier": rec.id(),
                    "offset": offset,
                    "sequence": String::from_utf8(chunk.to_vec()).unwrap()
                })
            );
            offset += 1000;
        })
    }
}
