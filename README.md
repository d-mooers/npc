# 🚀 Node Copilot 🤖

Node Copilot is a fun and interactive CLI tool that helps you get programming-related answers using the power of GPT-4 API! Just type your question, and let the magic happen! ✨💻🧠

## Features

- Interactive prompt for asking programming-related questions
- Save the responses to a specified output file
- Optionally include code from files as context

## Prerequisites

- Rust (latest stable version)
- API key for GPT-4 (replace `your_gpt_api_key_here` in `src/gpt.rs`)

## Setup

1. Clone the repository:

```sh
git clone https://github.com/yourusername/node-copilot.git
```

2. Move into the project directory:

```sh
cd node-copilot
```

3. Build the project:

```sh
cargo build --release
```

4. Add the `target/release` directory to your `PATH` or create a symlink to the executable.

## Usage

```sh
node-copilot --out <output_file> [--include <regex for file paths>]
```

- `<output_file>`: The output file path where the responses will be saved.
- `<regex for file paths>`: (Optional) A regex pattern to include specific files as context.

## Example:

```sh
node-copilot --out results.txt --include "src/*.rs"
```

This will start an interactive experience where you can ask programming-related questions. The responses will be displayed on the screen and saved to the specified output file.

To exit the interactive session, type 'exit' and press enter.
