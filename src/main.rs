mod cli;
mod file_reader;
mod gpt;

use cli::{draw_ascii_text, parse_args, type_to_terminal};
use file_reader::read_files;
use gpt::gpt_request;

use async_std::task;
use console::{style, Term};
use dotenv::dotenv;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

async fn run_interactive_loop(output_file: &str, include_pattern: Option<&str>) {
    let term = Term::stdout();
    let mut output = File::create(output_file).expect("Failed to create output file");

    loop {
        type_to_terminal(
            "Enter your programming-related question or type 'exit' to quit:\n",
            Duration::from_millis(10),
        );

        let user_input_result = term.read_line();

        let user_input = match user_input_result {
            Ok(input) => input,
            Err(_) => {
                type_to_terminal("Failed to read input", Duration::from_millis(50));
                continue;
            }
        };

        if user_input.trim().eq_ignore_ascii_case("exit") {
            type_to_terminal(
                &format!("{}", style("Goodbye").green()),
                Duration::from_millis(50),
            );
            break;
        }

        let context = match include_pattern {
            Some(pattern) => read_files(pattern),
            None => String::new(),
        };

        type_to_terminal(
            &format!(
                "\n{}\n\n{}\n",
                style("Copilot:").bold(),
                style("Let me think...").green()
            ),
            Duration::from_millis(10),
        );
        let response = gpt_request(&user_input, &context)
            .expect("Failed to get a response from the GPT-4 API");
        let response_raw = response.as_str();

        let styled_response = format!(
            "{}\n\n{}\n\n",
            style("Copilot:").bold(),
            style(response_raw).green()
        );
        type_to_terminal(&styled_response, Duration::from_millis(10));

        let success = output.write_all(response_raw.as_bytes());
        assert!(success.is_ok());
    }
}

fn main() {
    dotenv().ok();
    draw_ascii_text("Node Copilot");
    let (output_file, include_pattern) = parse_args();
    task::block_on(run_interactive_loop(
        &output_file,
        include_pattern.as_deref(),
    ))
}
