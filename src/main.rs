use std::io;

use clap::Parser;
use rust_find::find_by_name;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Generates metadata for help/version
struct Args {
    /// Directory to start the search
    #[arg(long, value_name = "DIR")]
    base_dir: String,

    /// String to check for in the name of the file
    #[arg(long)]
    contains: String,

    /// Max depth from the base-dir
    #[arg(long, default_value_t = 20)]
    max_depth: i32,
}

fn main() -> io::Result<()> {
    let _ = log4rs::init_file("log4rs.yaml", Default::default())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let cli_args = Args::parse();

    let found_files = find_by_name(&cli_args.base_dir, &cli_args.contains, cli_args.max_depth)?;

    for ff in found_files.iter() {
        println!("{ff}");
    }
    Ok(())
}
