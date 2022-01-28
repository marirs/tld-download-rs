use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliOpts {
    /// Writes to a output file
    #[clap(short = 'f', long, parse(from_os_str), value_name = "FILE")]
    output_file: PathBuf,
}

fn main() {
    let cli = CliOpts::parse();
    let db = tld_download::from_db();
    let mut file = File::create(&cli.output_file).expect("couldn't create file");
    let db = db.join("\n");
    file.write_all(db.as_bytes())
        .expect("couldn't write to file");
    println!(
        "File with all tld suffixes created, refer: {}",
        cli.output_file.display()
    );
}
