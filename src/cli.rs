use clap::{App, Arg};
use figlet_rs::FIGfont;
use std::time::Duration;
use console::{Term, style};

pub fn parse_args() -> (String, Option<String>) {
    let matches = App::new("node-copilot")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for interacting with GPT-4 API")
        .arg(
            Arg::with_name("output_file")
                .short('o')
                .long("out")
                .value_name("OUTPUT_FILE")
                .help("Sets the output file for storing the results")
                .required(false)
                .default_value("output.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .long("include")
                .value_name("REGEX")
                .help("A regex pattern to include specific files as context")
                .takes_value(true),
        )
        .get_matches();

    let output_file = matches.value_of("output_file").unwrap().to_string();
    let include = matches.value_of("include").map(|s| s.to_string());
    (output_file, include)
}

pub fn draw_ascii_text(text: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(text);
    assert!(figure.is_some());
    type_to_terminal(&format!("{}", style(figure.unwrap().to_string().as_str()).bold().bright().blue()), Duration::from_millis(2));
}

pub fn type_to_terminal(text: &str, delay: Duration) {
    let term = Term::stdout();
    for c in text.chars() {
        term.write_str(&c.to_string()).unwrap();
        std::thread::sleep(delay);
    }
    term.write_line("\n").unwrap();
}