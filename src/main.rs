use std::env;
use std::fs;

mod prisma;
mod types;
mod defaults;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide input file");
        std::process::exit(1);
    }

    // TODO implement logic to add extra links to markdown
    let extra_links_raw = String::from("");

    let scan_file_name = &args[1];
    let scan_file_content =
        fs::read_to_string(scan_file_name).expect("Something went wrong reading the scan file");
    prisma::convert_container_scan(&scan_file_content, &extra_links_raw)
        .expect("Something went wrong deserializing data");
}
