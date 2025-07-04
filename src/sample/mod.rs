use super::Attester;
use anyhow::*;
use base64::Engine;
use serde::{Deserialize, Serialize};

pub fn detect_platform() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug)]
struct SampleQuote {
    svn: String,
    report_data: String,
}

#[derive(Debug, Default)]
pub struct SampleAttester {}

#[async_trait::async_trait]
impl Attester for SampleAttester {
    async fn get_evidence(&self, report_data: Vec<u8>) -> Result<String> {
        let evidence = SampleQuote {
            svn: "1".to_string(),
            report_data: base64::engine::general_purpose::STANDARD.encode(report_data),
        };

        serde_json::to_string(&evidence).context("Serialize sample evidence failed")
    }
}
