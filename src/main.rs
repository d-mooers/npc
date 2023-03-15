mod cli;
mod gpt;
mod file_reader;

use cli::{parse_args, type_to_terminal, draw_ascii_text};
use gpt::gpt_request;
use file_reader::read_files;

use console::{Term, style};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use async_std::task;

async fn run_interactive_loop(output_file: &str, include_pattern: Option<&str>) {
    let term = Term::stdout();
    let mut output = File::create(output_file).expect("Failed to create output file");

    loop {
        type_to_terminal("Enter your programming-related question or type 'exit' to quit:\n", Duration::from_millis(50));

        let user_input_result = term.read_line();

        let user_input = match user_input_result {
            Ok(input) => input,
            Err(_) => {
                type_to_terminal("Failed to read input", Duration::from_millis(50));
                continue;
            }
        };

        if user_input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let context = match include_pattern {
            Some(pattern) => read_files(pattern),
            None => String::new(),
        };

        let response = gpt_request(&user_input, &context).await.expect("Failed to get a response from the GPT-4 API");

        let styled_response = format!("{}\n{}\n\n", style("Response:").bold(), response);
        type_to_terminal(&styled_response, Duration::from_millis(50));
        
        output.write_all(styled_response.as_bytes());
    }
}

fn main() {
    draw_ascii_text("Node Copilot");
    let (output_file, include_pattern) = parse_args();
    task::block_on(run_interactive_loop(&output_file, include_pattern.as_deref()))
}
