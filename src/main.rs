use std::env;
use std::fs;

pub mod prisma;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide input file");
        std::process::exit(1);
    }

    let config_file_name = "scan2md.json";
    let extra_links_raw = String::from("");

    if (std::path::Path::new(config_file_name).exists()) {
        let config_file_content = fs::read_to_string(config_file_name)
            .expect("Something went wrong reading the scan file");
    }

    let scan_file_name = &args[1];
    let scan_file_content =
        fs::read_to_string(scan_file_name).expect("Something went wrong reading the scan file");
    prisma::convert_container_scan(&scan_file_content, &extra_links_raw)
        .expect("Something went wrong deserializing data");
}
