extern crate docopt;
extern crate rustc_serialize;
extern crate crypto;

use std::process;
use std::fs;
use docopt::Docopt;

mod duplicates;

const USAGE: &'static str = "
Usage:
    fdupes <dir>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dir: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let paths = fs::read_dir(&args.arg_dir)
        .map(|fs|
             fs.flat_map(|f| f.map(|e| e.path()))
             .filter(|p| p.is_file())
             .collect())
        .unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(-1);
        });

    for duplicates in duplicates::find(paths) {
        for duplicate in duplicates {
            println!("{:?}", duplicate);
        }

        println!("");
    }
}
