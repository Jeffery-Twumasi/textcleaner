use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(
    author = "Jeffery Twumasi",
    version = "0.1.0",
    about = "TextCleaner: A simple CLI tool to clean text files"
)]
struct Args {

    /// Input file to clean
    input: String,

    /// Remove empty lines
    #[arg(long)]
    remove_empty: bool,

    /// Extract only lines containing this keyword
    #[arg(long)]
    only: Option<String>,
}



fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input)
        .expect("Could not read the input file");

    let mut cleaned_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
    // Skip empty lines if --remove-empty is set
    if args.remove_empty && line.trim().is_empty() {
        continue;
    }

    // If --only is set, keep only lines containing the keyword
    if let Some(ref keyword) = args.only {
        if !line.contains(keyword) {
            continue;
        }
    }

    cleaned_lines.push(line);
}



// Create (or overwrite) output file
let mut output_file = File::create("cleaned.txt")
    .expect("Could not create output file");

// Write cleaned lines to the file
for line in &cleaned_lines {
    writeln!(output_file, "{}", line)
        .expect("Could not write to output file");
}

println!("Cleaning complete!");
println!("Output written to cleaned.txt");

   
}
