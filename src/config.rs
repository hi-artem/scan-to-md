use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
struct ConverterConfig {
    #[serde(rename = "outputFile", default = "default_output_file")]
    output_file: String,
    #[serde(rename = "extraLinksRaw", default = "default_string")]
    extra_links_raw: String,
}

fn default_output_file() -> String {
    "results.md".to_string()
}

fn default_string() -> String {
    "".to_string()
}
