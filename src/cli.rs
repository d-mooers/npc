use clap::{App, Arg};

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
                .required(true)
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
