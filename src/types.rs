use serde::Deserialize;
use crate::defaults;

#[derive(Deserialize)]
pub struct PrismaScan {
    #[serde(rename = "consoleURL", default = "defaults::default_string")]
    pub console_url: String,
    #[serde(default = "defaults::default_scan_results_vector")]
    pub results: Vec<ScanResult>,
}

#[derive(Deserialize)]
pub struct ScanResult {
    #[serde(default = "defaults::default_string")]
    pub id: String,
    #[serde(default = "defaults::default_string")]
    pub name: String,
    #[serde(rename = "complianceScanPassed", default = "defaults::default_bool")]
    pub compliance_scan_passed: bool,
    #[serde(rename = "vulnerabilityScanPassed", default = "defaults::default_bool")]
    pub vulnerability_scan_passed: bool,
    #[serde(default = "defaults::default_compliances_vector")]
    pub compliances: Vec<Compliance>,
    #[serde(default = "defaults::default_vulnerabilities_vector")]
    pub vulnerabilities: Vec<Vulnerability>,
    #[serde(
        rename = "vulnerabilityDistribution",
        default = "defaults::default_scan_results_distribution"
    )]
    pub vulnerability_distribution: ScanResultDistribution,
    #[serde(
        rename = "complianceDistribution",
        default = "defaults::default_scan_results_distribution"
    )]
    pub compliance_distribution: ScanResultDistribution,
}

#[derive(Deserialize)]
pub struct Compliance {
    #[serde(default = "defaults::default_i32")]
    pub id: i32,
    #[serde(default = "defaults::default_string")]
    pub title: String,
    #[serde(default = "defaults::default_string")]
    pub severity: String,
    #[serde(default = "defaults::default_string")]
    pub description: String,
    #[serde(default = "defaults::default_string")]
    pub cause: String,
}

#[derive(Deserialize)]
pub struct Vulnerability {
    #[serde(default = "defaults::default_string")]
    pub id: String,
    #[serde(rename = "packageName", default = "defaults::default_string")]
    pub pacakage_name: String,
    #[serde(rename = "packageVersion", default = "defaults::default_string")]
    pub pacakage_version: String,
    #[serde(default = "defaults::default_string")]
    pub status: String,
    #[serde(default = "defaults::default_string")]
    pub description: String,
    #[serde(default = "defaults::default_string")]
    pub severity: String,
    #[serde(default = "defaults::default_string")]
    pub link: String,
    #[serde(rename = "graceDays", default = "defaults::default_i32")]
    pub grace_days: i32,
}

#[derive(Deserialize)]
pub struct ScanResultDistribution {
    #[serde(default = "defaults::default_i32")]
    pub critical: i32,
    #[serde(default = "defaults::default_i32")]
    pub high: i32,
    #[serde(default = "defaults::default_i32")]
    pub medium: i32,
    #[serde(default = "defaults::default_i32")]
    pub low: i32,
    #[serde(default = "defaults::default_i32")]
    pub total: i32,
}
