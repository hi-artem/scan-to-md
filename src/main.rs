use serde::Deserialize;
use serde_json::Result;
use std::fmt;
use std::fs;
use std::env;

#[derive(Deserialize)]
struct Scan {
    #[serde(rename = "consoleURL",default = "default_string")]
    console_url: String,
    #[serde(default = "default_scan_results_vector")]
    results: Vec<ScanResult>,
}

#[derive(Deserialize)]
struct ScanResult {
    #[serde(default = "default_string")]
    id: String,
    #[serde(default = "default_string")]
    name: String,
    #[serde(default = "default_compliances_vector")]
    compliances: Vec<Compliance>,
    #[serde(rename = "complianceScanPassed", default = "default_bool")]
    compliance_scan_passed: bool,
    #[serde(default = "default_vulnerabilities_vector")]
    vulnerabilities: Vec<Vulnerability>,
    #[serde(rename = "vulnerabilityScanPassed", default = "default_bool")]
    vulnerability_scan_passed: bool,
}

#[derive(Deserialize)]
struct Compliance {
    #[serde(default = "default_i32")]
    id: i32,
    #[serde(default = "default_string")]
    title: String,
    #[serde(default = "default_string")]
    severity: String,
    #[serde(default = "default_string")]
    description: String,
    #[serde(default = "default_string")]
    cause: String,
}

#[derive(Deserialize)]
struct Vulnerability {
    #[serde(default = "default_string")]
    id: String,
    #[serde(rename = "packageName", default = "default_string")]
    pacakage_name: String,
    #[serde(rename = "packageVersion", default = "default_string")]
    pacakage_version: String,
    #[serde(default = "default_string")]
    status: String,
    #[serde(default = "default_string")]
    description: String,
    #[serde(default = "default_string")]
    severity: String,
    #[serde(default = "default_string")]
    link: String,
    #[serde(rename = "graceDays", default = "default_i32")]
    grace_days: i32,
}

fn default_string() -> String {
    "".to_string()
}

fn default_i32() -> i32 {
    0
}

fn default_bool() -> bool {
    false
}

fn default_compliances_vector() -> Vec<Compliance> {
    let default_compliances =  Vec::new();
    default_compliances
}

fn default_vulnerabilities_vector() -> Vec<Vulnerability> {
    let default_vulnerabilities =  Vec::new();
    default_vulnerabilities
}

fn default_scan_results_vector() -> Vec<ScanResult> {
    let default_scan_results =  Vec::new();
    default_scan_results
}


fn main() {
    container_scan().expect("Something went wrong deserializing data");
}

fn container_scan() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide input file");
        std::process::exit(1);
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let scan: Scan = serde_json::from_str(&contents)?;

    if scan.results.len() < 1 {
        eprintln!("No scan results found in the input file");
        std::process::exit(10);
    }

    let scan_result = &scan.results[0];
    let vulnerabilities = &scan.results[0].vulnerabilities;
    let compliances = &scan.results[0].compliances;

    let mut vulnerabilities_table = String::from("| Id | Package | Version | Description | Severity | Grace Days |\n| --- | --- | --- | --- | --- | --- |\n");
    for v in vulnerabilities {
        vulnerabilities_table = format!(
            "{}| [{}]({}) | {} | {} | {} | {} | {} |\n",
            vulnerabilities_table,
            v.id,
            v.link,
            v.pacakage_name,
            v.pacakage_version,
            v.description.replace("\n", ""),
            v.severity,
            v.grace_days
        );
    }
    let vulnerabilities_table_expandable = format!(
        "<details>\n<summary>See details</summary>\n\n{}\n\n</details>",
        vulnerabilities_table
    );

    let mut compliances_table =
        String::from("| Title | Severity | Description |\n| --- | --- | --- |\n");
    for c in compliances {
        compliances_table = format!(
            "{}| {} | {} | {} {} |\n",
            compliances_table,
            c.title,
            c.severity,
            c.description.replace("\n", ""),
            c.cause.replace("\n", "")
        );
    }
    let compliances_table_expandable = format!(
        "<details>\n<summary>See details</summary>\n\n{}\n\n</details>",
        compliances_table
    );

    let vulnerability_scan_results = if scan_result.vulnerability_scan_passed {
        "Passed :white_check_mark:"
    } else {
        "Failed :x:"
    };
    let compliance_scan_results = if scan_result.compliance_scan_passed {
        "Passed :white_check_mark:"
    } else {
        "Failed :x:"
    };
    let prisma_logo = "![Prisma Cloud Logo](https://www.paloaltonetworks.com/content/dam/pan/en_US/images/logos/brand/prisma-primary-reversed/Prisma-logo-reversed.png)\n";

    let scan_result_body = format!(
        "# Scan result for {}\n\n{}\n\n## Vulnerabilities\n\nStatus: {}\n\n{}\n\n## Compliance\n\nStatus: {}\n\n{}\n\n",
        scan_result.name,
        prisma_logo,
        vulnerability_scan_results,
        vulnerabilities_table_expandable,
        compliance_scan_results,
        compliances_table_expandable
    );

    print!("{}## Links\n\n- [View scan result in Prisma Cloud console]({})\n- [Do other stuff](https://example.com)\n\n", scan_result_body, scan.console_url);

    Ok(())
}
