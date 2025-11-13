use std::fs;
use std::io::{self, Write};
use regex::Regex;

fn remove_comments(python_code: &str) -> String {
    // Regular expression to match single-line comments
    let single_line_re = Regex::new(r"#.*").unwrap();
    // Regular expression to match multi-line comments
    let multi_line_re = Regex::new(r"/\*.*?\*/").unwrap();

    // Remove single-line comments
    let code_without_single_line_comments = single_line_re.replace_all(python_code, "");

    // Remove multi-line comments
    let code_without_comments = multi_line_re.replace_all(&code_without_single_line_comments, "");

    code_without_comments.to_string()
}

fn main() -> io::Result<()> {
    // Prompt the user for the input file path
    print!("Enter the path to the Python file: ");
    io::stdout().flush()?;
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();

    // Read the content of the input file
    let python_code = fs::read_to_string(input_path)?;

    // Remove comments from the Python code
    let code_without_comments = remove_comments(&python_code);

    // Prompt the user for the output file path
    print!("Enter the path to save the output file: ");
    io::stdout().flush()?;
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();

    // Write the code without comments to the output file
    fs::write(output_path, code_without_comments)?;

    println!("Comments removed and saved successfully!");

    Ok(())
}