mod cli;
mod gpt;
mod file_reader;

use cli::parse_args;
use gpt::gpt_request;
use file_reader::read_files;

use console::{Term, style};
use std::fs::File;
use std::io::Write;
use async_std::task;

async fn run_interactive_loop(output_file: &str, include_pattern: Option<&str>) {
    let term = Term::stdout();
    let mut output = File::create(output_file).expect("Failed to create output file");

    loop {
        let term_success = term.write_line("Enter your programming-related question or type 'exit' to quit:");

        match term_success {
            Ok(_) => {}
            Err(_) => {
                term.write_line("Failed to write to terminal");
                continue;
            }
        }


        let user_input_result = term.read_line();

        let user_input = match user_input_result {
            Ok(input) => input,
            Err(_) => {
                term.write_line("Failed to read input");
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
        term.write_line(&styled_response);
        output.write_all(styled_response.as_bytes());
    }
}

fn main() {
    let (output_file, include_pattern) = parse_args();
    task::block_on(run_interactive_loop(&output_file, include_pattern.as_deref()))
}
