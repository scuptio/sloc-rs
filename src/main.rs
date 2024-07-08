mod sloc;

use std::path::Path;
use clap::Parser;
use crate::sloc::count_lines_in_directory;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// directory to search
    #[arg(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    let (n1, n2) = count_lines_in_directory(path);
    println!("total SLOC : {}", n1);
    println!("test SLOC : {}", n2);
}
