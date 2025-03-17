// STRINGS RECAP ===========================================
// fn main() {
//     // &str - reference string slice
//     // String - heap allocated string

//     let pirate = "Bloodhook";

//     let sailor = String::from(pirate);

//     let bad_guy = pirate.to_string();

//     println!("{pirate} and {sailor} and {bad_guy}");

//     let first_initial = &pirate[0..1];
//     println!("{first_initial}");
// }

// STRING CONCAT =====================================
// fn main() {
//     let first_name = String::from("Sylvester");
//     let last_name = String::from("Stallone");

//     let full_name = first_name + &last_name;

//     // println!("{first_name}");
//     println!("{full_name}");
// }

// format! MACRO ===========================
// fn main() {
//     let first_name = String::from("Sylvester");
//     let last_name = String::from("Stallone");

//     let icon = format!("{first_name} {last_name}");

//     println!("{icon}");
//     println!("{first_name}");
//     println!("{last_name}");
// }

// COMMON METHODS ==========================
// fn main() {
//     let mut music_genres = "           Rock, Metal, Country, Rap    ";
//     // println!("{}", music_genres.trim());
//     // println!("{}", music_genres.trim_start());
//     // println!("{}", music_genres.trim_end());

//     music_genres = music_genres.trim();
//     println!("{}", music_genres.to_uppercase());
//     println!("{}", music_genres.to_lowercase());

//     println!("{}", music_genres.replace("a", "@"));
//     println!("{:?}", music_genres.split(", ").collect::<Vec<&str>>());
// }

// COLLECTING INPUT ==========================
use std::io;
fn main() {
    let mut name = String::new();
    print!("What is your name?:\n");
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(err) => println!("There was an error: {err}"),
    }
}
