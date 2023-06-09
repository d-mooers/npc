use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, time::Duration};

const GPT_API_BASE: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";
const SYSTEM_PROMPT: &str = "You are an engineer, named 'Node Copilot'. 
You have vast knowledge of all fields of software engineering. 
You are able to answer programming questions";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Assistant,
    System,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

type Prompt = Vec<Message>;

pub fn build_prompt(base_prompt: &str, context: &str) -> Prompt {
    let mut prompt = vec![
        Message {
            role: Role::System,
            content: SYSTEM_PROMPT.to_string(),
        },
        Message {
            role: Role::Assistant,
            content: "Acknowledged, I am Node Copilot. Please provide some context code so I know what technologies to reference for future answers.".to_string(),
        }
    ];

    if context.len() > 0 {
        prompt.push(Message {
            role: Role::User,
            content: format!("Here is some code from my codebase\n{}", context),
        });
    }

    prompt.push(Message {
        role: Role::User,
        content: format!("{}", base_prompt),
    });

    prompt
}

#[tokio::main]
pub async fn gpt_request(prompt: &str, context: &str) -> Result<String, Error> {
    let api_key = env::var("GPT_API_KEY").expect("GPT_API_KEY must be set");
    let max_tokens = env::var("MAX_TOKENS")
        .unwrap_or("100".to_string())
        .parse::<u32>()
        .unwrap_or_default();
    let client = Client::builder().timeout(Duration::from_secs(60)).build()?;

    let prompt_with_context = build_prompt(prompt, context);

    let data = json!({
        "model": MODEL,
        "messages": prompt_with_context,
        "max_tokens": max_tokens,
        "temperature": 0.5,
    });

    let response: Response = client
        .post(GPT_API_BASE)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&data)
        .send()
        .await?;

    let response_data = response.json::<serde_json::Value>().await?;
    let text = response_data.to_string();
    let generated_text = response_data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or(text.as_str());

    Ok(generated_text.trim().to_string())
}
