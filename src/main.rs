use anyhow::{Result};
use clap::{Parser, Subcommand};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rustyline::DefaultEditor;
use serde_json::{json, Value};
use std::env;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Chat {
        #[arg(short, long, default_value = "llama-3.2-1b-instruct")]
        model: String, //llama-3.1-8b-instruct //gemini-1.5-flash
    },
    Analyze {
        #[arg(short, long, default_value = "100")]
        n: usize,

        #[arg(short, long, default_value = "llama-3.2-1b-instruct")]
        model: String,
    },
}

async fn call_llm_api(model: &str, messages: Vec<Value>) -> Result<Value> {
    
    let api_key = env::var("OPENAI_API_KEY")?;
    let base_url = env::var("OPENAI_BASE_URL")?;
    let chat_completion_url = format!("{}/chat/completions", base_url);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    let request_body = json!({
        "model": model,
        "messages": messages,
        "temperature": 0.8
    });

    let response = client
        .post(chat_completion_url)
        .headers(headers.clone())
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_data: Value = response.json().await?;
        Ok(response_data)
    } else {
        let error_text = response.text().await?;
        Err(anyhow::anyhow!("Error: {}", error_text))
    }
}

async fn analyze_logs(input_text: &str, n: usize, model: &str) -> Result<()> {
    println!("Analyzing last {} lines of logs:\n {} \n", n, input_text);
    
    let mut messages = Vec::new();

    messages.push(json!({
        "role": "system",
        "content": "You are a computer science expert. Analyze the following logs and find the issues and potential bugs. Keep it concise and to the point.",
    }));

    messages.push(json!({
        "role": "user",
        "content": input_text,
    }));

    let response_data = call_llm_api(model, messages.clone()).await?;
    
    if let Some(message) = response_data["choices"][0]["message"]["content"].as_str() {
        println!("\nHere is what I think about the logs: {}\n", message);
        messages.push(json!({
            "role": "assistant",
            "content": message
        }));
    }

    Ok(())
}

async fn handle_chat(model: &str) -> Result<()> {
    // Ensure OPENAI_API_KEY is set
    if env::var("OPENAI_API_KEY").is_err() {
        println!("Please set the OPENAI_API_KEY environment variable");
        return Ok(());
    }

    let mut rl = DefaultEditor::new()?;
    let mut messages = Vec::new();

    messages.push(json!({
        "role": "system",
        "content": "You are a command line expert. Give me a precise CLI command for the query I ask you. Keep it concise, just give me the exact command to run (no explanations).",
    }));

    println!("Chat started with model: {}. Type 'exit' to quit.", model);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().eq_ignore_ascii_case("exit") {
                    break;
                }

                rl.add_history_entry(line.as_str())?;

                messages.push(json!({
                    "role": "user",
                    "content": line
                }));

                let response_data = call_llm_api(model, messages.clone()).await?;

                if let Some(message) = response_data["choices"][0]["message"]["content"].as_str() {
                    println!("\nCommand: {}\n", message);
                    messages.push(json!({
                        "role": "assistant",
                        "content": message
                    }));
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Chat { model }) => {
            handle_chat(model).await?;
        }
        Some(Commands::Analyze { n, model }) => {
            let mut buffer = Vec::new();
            let stdin = io::stdin();
            let handle = stdin.lock();
            let reader = io::BufReader::new(handle);

            for line in reader.lines() {
                if let Ok(line) = line {
                    buffer.push(line);
                    if buffer.len() > *n {
                        buffer.remove(0);
                    }
                }
            }
            analyze_logs(buffer.join("\n").trim(), *n, model).await?;
        },
        None => {
            if let Some(input) = cli.input {
                println!("Input: {}", input);
            } else {
                println!("No command provided. Use --help for usage information.");
            }
        }
    }

    Ok(())
}
