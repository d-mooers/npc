<p align="center">
  <a href="http://nodefinance.org" target="_blank"><img src="https://storage.googleapis.com/node_protocol_images/nodefinance.png" width="200" alt="Node Finance Logo" /></a>
</p>

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

### How to Create a Symlink

You can create a symlink to the executable by running one of the following commands, depending on your operating system:

#### Linux and macOS:

```sh
sudo ln -s $(pwd)/target/release/npc /usr/local/bin/npc
```

#### Windows (using Command Prompt with administrative privileges):

```sh
mklink %SystemRoot%\System32\node-copilot.exe %CD%\target\release\node-copilot.exe
```

### How to add directly to PATH

Alternatively, you can add the target/release directory to your PATH:

#### Linus / Macos

For Linux and macOS, add the following line to your shell configuration file (e.g., ~/.bashrc, ~/.zshrc, or ~/.bash_profile):

```sh
export PATH="$PATH:/path/to/your/node-copilot/target/release"
```

Remember to replace /path/to/your/node-copilot with the actual path to your project directory.

#### Windows

For Windows, follow these steps:

1. Open the Start menu, search for "Environment Variables" and click on "Edit the system environment variables".
2. In the "System Properties" window, click on the "Environment Variables" button.
3. In the "Environment Variables" window, under "System variables", find the "Path" variable, select it, and click on "Edit".
4. In the "Edit environment variable" window, click on "New" and add the full path to the `target\release directory` (e.g., `C:\path\to\your\node-copilot\target\release`).

After making these changes, restart your shell or terminal for the new `PATH` settings to take effect.

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
