use crate::types;

pub fn default_string() -> String {
    "".to_string()
}

pub fn default_i32() -> i32 {
    0
}

pub fn default_bool() -> bool {
    false
}

pub fn default_compliances_vector() -> Vec<types::Compliance> {
    let default_compliances = Vec::new();
    default_compliances
}

pub fn default_vulnerabilities_vector() -> Vec<types::Vulnerability> {
    let default_vulnerabilities = Vec::new();
    default_vulnerabilities
}

pub fn default_scan_results_vector() -> Vec<types::ScanResult> {
    let default_scan_results = Vec::new();
    default_scan_results
}

pub fn default_scan_results_distribution() -> types::ScanResultDistribution {
    let default_scan_results_distribution = types::ScanResultDistribution {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        total: 0,
    };
    default_scan_results_distribution
}
