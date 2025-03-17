use std::fs::File;
use std::io::{self, Write};
use std::process;

fn main() {
    let my_file = write_to_file();

    match my_file {
        Ok(filename) => println!("Successfully wrote {filename} to disk."),
        Err(error) => {
            eprintln!("There was an error creating the file, {error}");
            process::exit(1);
        }
    }
}

fn get_user_input(prompt: &str) -> Result<String, std::io::Error> {
    let mut input = String::new();
    println!("{prompt}");
    io::stdin().read_line(&mut input)?;
    Ok(format!("{}", input.trim()))
}

fn write_to_file() -> Result<String, std::io::Error> {
    let filename = get_user_input("What file would you like to write to?")?;
    let content = get_user_input("What would you like to write to the file?")?;

    let mut raw_file = File::create(&filename)?;
    File::write(&mut raw_file, content.trim().as_bytes())?;
    Ok(filename)
}
