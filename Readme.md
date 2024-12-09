<p align="center">
  <a href="/">
    <img src="assets/terminal-sage-bg.png" width="318px" alt="Terminal Sage logo" />
</a>
<h3 align="center">Open-source LLM Powered Terminal Assistant</h3>
<p align="center">
  <a href="/">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/darshpanchal/terminal-sage">
  </a>
  <a href="/">
    <img alt="GitHub Issues or Pull Requests" src="https://img.shields.io/github/issues/darshpanchal/terminal-sage">
  </a>
</p>

# Terminal Sage

Terminal Sage is a command-line interface (CLI) tool powered by large language models (LLMs) designed to assist users with command-line operations and log analysis. It leverages OpenAI compatible API to provide intelligent suggestions and insights directly inside the terminal.

## Features

- **Chat Mode**: Interact with the CLI to get precise command-line instructions for your queries.
- **Analyze Mode**: Analyze logs to identify issues and potential bugs with the help of an AI model.
- **Customizable Models**: Choose from different AI models to tailor the responses to your needs.

## Installation

To get started with Terminal Sage, ensure you have Rust and Cargo installed. Then, clone the repository and build the project:

```bash
git clone https://github.com/darshpanchal/terminal-sage.git
cd terminal-sage
cargo build --release
```

## Usage

Before using Terminal Sage, make sure to set the required environment variables:

```bash
export OPENAI_API_KEY="your_openai_api_key" # Your API key (keep it random if using local model)
export OPENAI_BASE_URL="https://api.openai.com/v1" # Optional, defaults to OpenAI's API
export MODEL_NAME="gpt-4o"
```

### Local Model Usage (Ollama/Llama.cpp server)

Before using Terminal Sage, make sure to set the required environment variables:

```bash
export OPENAI_API_KEY="sk-xxx" 
export OPENAI_BASE_URL="http://localhost:1234/v1"
export MODEL_NAME="llama-3.2-1b-instruct"
```
Use --model flag to specify the model to use. I have tested with `llama-3.2-1b-instruct` (locally) and `llama-3.1-8b-instruct` (locally and with perplexity API).

### Commands

- **Chat Mode**: Start an interactive session to get command-line suggestions.
  ```bash
  ./target/release/terminal-sage chat --model <model_name>
  ```

- **Analyze Mode**: Analyze the last `n` lines of logs.
  ```bash
  ./target/release/terminal-sage analyze --n <number_of_lines> --model <model_name>
  ```


## Configuration

You can configure the default model using `MODEL_NAME` env or pass `--model` param to use different model in command-line parameters. You can also pass `-n` param to set number of lines to analyze (default is last 100 lines).

## Dependencies

Terminal Sage relies on several Rust crates, including:

- `clap` for command-line argument parsing
- `anyhow` for error handling
- `reqwest` for HTTP requests
- `serde_json` for JSON handling
- `tokio` for asynchronous runtime

For more details, refer to the `Cargo.toml` file.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

