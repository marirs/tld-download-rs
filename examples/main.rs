use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliOpts {
    /// Include Private Domains (default: does not include private domain)
    #[clap(short = 'p', long)]
    include_private_domains: bool,

    /// Writes to a output file
    #[clap(short = 'f', long, value_name = "FILE")]
    output_file: PathBuf,
}

fn main() {
    let cli = CliOpts::parse();
    match tld_download::from_publicsuffix(cli.include_private_domains) {
        Ok(r) => {
            let mut file = File::create(&cli.output_file).expect("couldn't create file");
            let r = r.join("\n");
            file.write_all(r.as_bytes())
                .expect("couldn't write to file");
            println!(
                "File with all tld suffixes created, refer: {}",
                cli.output_file.display()
            );
        }
        Err(e) => {
            println!("{e:?}")
        }
    }
}
