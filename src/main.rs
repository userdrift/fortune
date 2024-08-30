use reqwest;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const SECRET_KEY: &str = "SECRET_KEY";

fn fetch_() -> Result<String, io::Error> {
    if Path::new(SECRET_KEY).exists() {
        fs::read_to_string(SECRET_KEY)
    } else {
        print!("API_KEY: ");
        io::stdout().flush()?;
        let mut secret_key = String::new();
        io::stdin().read_line(&mut secret_key)?;
        let secret_key = secret_key.trim().to_string();
        fs::write(SECRET_KEY, &secret_key)?;
        Ok(secret_key)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = "request.json";
    if !Path::new(json_file).exists() {
        println!("request.json file not found.");
        return Ok(());
    }

    let secret_key = fetch_()?;

    let content = fs::read_to_string(json_file)?;
    let value: Value = serde_json::from_str(&content)?;

    let client = reqwest::Client::new();
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
                if let Some(fortune_s) = content.find("\"value\":") {
                    let fortune_content = &content[fortune_s + "\"value\":".len()..];
                    if let Some(fortune_e) = fortune_content.find('}') {
                        let fortune = fortune_content[..fortune_e].trim();
                        println!("{}", fortune.trim_matches('"'));
                    }
                }
            }
        }
    }

    Ok(())
}
