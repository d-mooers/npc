use reqwest::{Client, Response, Error};
use serde_json::json;
use std::time::Duration;

const GPT_API_BASE: &str = "https://api.openai.com/v1/engines/davinci-codex/completions";
const GPT_API_KEY: &str = "your_gpt_api_key_here";

pub async fn gpt_request(prompt: &str, context: &str) -> Result<String, Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let prompt_with_context = format!("{}{}{}", prompt, "\n\n", context);

    let data = json!({
        "prompt": prompt_with_context,
        "max_tokens": 100,
        "temperature": 0.5,
    });

    let response: Response = client
        .post(GPT_API_BASE)
        .header("Authorization", format!("Bearer {}", GPT_API_KEY))
        .json(&data)
        .send()
        .await?;

    let response_data = response.json::<serde_json::Value>().await?;
    let generated_text = response_data["choices"][0]["text"].as_str().unwrap_or("");

    Ok(generated_text.trim().to_string())
}
