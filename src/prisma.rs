use crate::types;
use serde_json::Result;

pub fn convert_container_scan(json_content: &String, extra_links_raw: &String) -> Result<()> {
    let scan: types::PrismaScan = serde_json::from_str(json_content)?;

    if scan.results.len() < 1 {
        eprintln!("No scan results found in the input file");
        std::process::exit(10);
    }

    let scan_result = &scan.results[0];
    let vulnerabilities = &scan.results[0].vulnerabilities;
    let vulnerability_distribution = &scan.results[0].vulnerability_distribution;
    let compliances = &scan.results[0].compliances;
    let compliance_distribution = &scan.results[0].compliance_distribution;

    let mut vulnerabilities_table = String::from("| Id | Package | Version | Description | Severity | Grace Days |\n| --- | --- | --- | --- | --- | --- |\n");
    
    if vulnerabilities.len() == 0 {
        vulnerabilities_table = String::from("No vulnerabilities found!");
    }
    
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

    let vulnerability_distribution_stats = format!(
        "Critical: {} High: {} Medium: {} Low: {} Total: {}\n\n",
        vulnerability_distribution.critical,
        vulnerability_distribution.high,
        vulnerability_distribution.medium,
        vulnerability_distribution.low,
        vulnerability_distribution.total,
    );

    let mut compliances_table =
        String::from("| Title | Severity | Description |\n| --- | --- | --- |\n");

    if compliances.len() == 0 {
        compliances_table = String::from("No compliance violations found!");
    }
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

    let compliance_distribution_stats = format!(
        "Critical: {} High: {} Medium: {} Low: {} Total: {}\n\n",
        compliance_distribution.critical,
        compliance_distribution.high,
        compliance_distribution.medium,
        compliance_distribution.low,
        compliance_distribution.total,
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
        "# Scan result for {}\n\n{}\n\n## Vulnerabilities\n\n{}Status: {}\n\n{}\n\n## Compliance\n\n{}Status: {}\n\n{}\n\n",
        scan_result.name,
        prisma_logo,
        vulnerability_distribution_stats,
        vulnerability_scan_results,
        vulnerabilities_table_expandable,
        compliance_distribution_stats,
        compliance_scan_results,
        compliances_table_expandable
    );

    print!("{}## Links\n\n- [View scan result in Prisma Cloud console]({})\n{}\n\n", scan_result_body, scan.console_url, extra_links_raw);

    Ok(())
}
