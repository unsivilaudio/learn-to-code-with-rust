// use std::fs;
// use std::io::{self, stdin};
// use std::process;

// ERROR HANDLING INTRO ===================================
// fn main() {
//     println!("The program is running");
//     // process::exit(1);

//     eprintln!("Whoops we had an issue! Exiting...");
// }

// OPENING A FILE =========================
// fn main() {
//     let story = match File::open("story.txt") {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Something went wrong reading the file. The error was {error}");
//             process::exit(1);
//         }
//     };

//     println!("{:?}", story);
// }

// OPENING A USER DEFINED FILE =========================
// fn main() {
//     println!("Please enter a name of a file you'd like to read:");
//     let mut input = String::new();

//     let user_requsted_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requsted_file {
//         eprintln!("Something went wrong collecting user input, the error was {error:?}");
//         process::exit(1)
//     }

//     let mut file = match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Something went wrong opening the file. The error was {error}");
//             process::exit(1);
//         }
//     };

//     let mut file_content = String::new();
//     let read_operation = file.read_to_string(&mut file_content);

//     if let Err(error) = read_operation {
//         eprintln!("Something went wrong reading the file as a string, the error was {error}");
//         process::exit(1);
//     }

//     println!("{}", file_content);
// }

// PROPEGATING ERRORS / ? operator =================================================
// fn main() {
//     let file_result = read_file();

//     match file_result {
//         Ok(content) => println!("{content}"),
//         Err(error) => {
//             eprintln!("{error}");
//             process::exit(1);
//         }
//     }
// }

// fn read_file() -> Result<String, io::Error> {
//     println!("Please enter a name of a file you'd like to read:");

//     let mut input = String::new();
//     stdin().read_line(&mut input)?;

//     fs::read_to_string(input.trim())
// }

// OPTION with ? ==================================================================
fn main() {
    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}
