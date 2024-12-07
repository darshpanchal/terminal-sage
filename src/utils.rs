// src/utils.rs
use std::env;
use anyhow::{Result, Context};

pub fn check_openai_api_key() -> Result<()> {
    env::var("OPENAI_API_KEY").context("Please set the OPENAI_API_KEY environment variable")?;
    Ok(())
}

pub fn model_selector(model_param: Option<&String>) -> Result<String> {
    // Check if the --model parameter is provided
    if let Some(model) = model_param {
        return Ok(model.clone()); // Use the provided model parameter
    }

    // Check the environment variable
    let model_name = env::var("MODEL_NAME").context("Neither --model parameter nor MODEL_NAME environment variable is set.")?;

    Ok(model_name) // Return the model name from the environment variable
}
