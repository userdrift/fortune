use reqwest;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jśon = "request.json";
    if !Path::new(jśon).exists() {
        return Ok(());
    }

    let content = fs::read_to_string(jśon)?;
    let value: Value = serde_json::from_str(&content)?;

    let client = reqwest::Client::new();
    let secret_key = "AIzaSyCh_C0LtMD_fOG-T_7qtjm2HhfFqCCGv2U";
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        secret_key
    );

    let response = client
        .post(&url)
        .json(&value)
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(candidates) = response["candidates"].as_array() {
        if let Some(first_candidate) = candidates.first() {
            if let Some(content) = first_candidate["content"]["parts"][0]["text"].as_str() {
                if let Some(fortune_start) = content.find("\"value\":") {
                    let fortune_content = &content[fortune_start + "\"value\":".len()..];
                    if let Some(fortune_end) = fortune_content.find('}') {
                        let fortune = fortune_content[..fortune_end].trim();
                        println!("{}", fortune.trim_matches('"'));
                    }
                }
            }
        }
    }

    Ok(())
}
