// CLOSURES INTRO ==================================
// fn main() {
//     let multiplier = 5;

//     fn multiply_by(value: i32) -> i32 {
//         value * multiplier // NOPE -- no parent scope access
//     }

//     println!("{}", multiply_by(3));
// }

// CLOSURES INTRO-2 ==================================
// fn main() {
//     let multiplier = 5;

//     // fn multiply_by(value: i32) -> i32 {
//     //     value * multiplier
//     // }

//     let multiply_by = |val: i32| -> i32 { val * multiplier };
//     println!("{}", multiply_by(5));

//     let product = |a: i32, b: i32| -> i32 { a * b };
//     println!("{}", product(4, 11));
// }

// CLOSURE SHORTCUTS ================================
// fn main() {
//     let multiplier = 5;
//     let multiply_by = |value| value * multiplier;
//     println!("{}", multiply_by(5 as u8));
//     // println!("{}", multiply_by(3.14)); // ERROR - 'u8' set above

//     let mirror = |value| value;
//     println!("{}", mirror("Why"));
//     // println!("{}", mirror(10)); // ERROR - "&str" set above
// }

// CLOSURE IMMUTABLE REFS ===================================
// fn main() {
//     let multiplier = 5;

//     let multiply_by = |value: u8| value * multiplier;
//     println!("{}", multiply_by(3));

//     let numbers = vec![4, 8, 15, 16, 23, 42];
//     println!("{:?}", numbers);

//     let print_numbers = || {
//         println!("{:?}", numbers); // DONT COLLAPSE
//     };
//     print_numbers();
//     print_numbers();
//     print_numbers();
//     println!("{:?}", numbers);
// }

// CLOSURE MUTABLE REFS ====================================
// fn main() {
//     let mut numbers = vec![4, 8, 15, 16, 23, 42];
//     let mut add_number = || numbers.push(100);
//     add_number();
//     add_number();
//     println!("{:?}", numbers);
// }

// CLOSURE WITH OWNERSHIP ================================
// fn main() {
//     let number = 13;
//     let capture_number = || number;
//     let a = capture_number();
//     let b = capture_number();
//     println!("{a} {b} {number}");

//     let first_name = String::from("Alice");
//     let capture_string = || first_name;
//     println!("{}", capture_string());
// }

// MOVE KEYWORD ================================
// fn main() {
//     let first_name = String::from("Alice");
//     let last_name = String::from("Wonder");
//     let capture_string = move || {
//         println!("{first_name} {last_name}");
//     };
//     capture_string();
//     capture_string();
//     capture_string();
//     // println!("{first_name}");
//     // println!("{last_name}");
// }

// unwrap_or_else METHOD =========================
// fn main() {
//     let option = Option::Some("Salami");
//     let closure = || "Pizza";
//     let food = option.unwrap_or_else(closure);
//     println!("{food}");

//     let option = Option::None;
//     let pizza_fan = false;
//     let closure = || if pizza_fan { "Pizza" } else { "Hot Pocket" };
//     let food = option.unwrap_or_else(closure);
//     println!("{food}");
// }

// FNONCE TRAIT ==================================
// use std::io::stdin;

// #[derive(Debug)]
// struct Vault {
//     password: String,
//     treasure: String,
// }

// impl Vault {
//     fn unlock<F>(self, procedure: F) -> Option<String>
//     where
//         F: FnOnce() -> String,
//     {
//         let user_password = procedure();
//         if user_password == self.password {
//             Some(self.treasure)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let vault = Vault {
//         password: String::from("topsecret"),
//         treasure: String::from("Gold"),
//     };
//     let hack = || {
//         let mut user_input = String::new();
//         println!("Please provide a password to crack the vault");
//         _ = stdin().read_line(&mut user_input);
//         user_input.trim().to_string()
//     };
//     let extraction = vault.unlock(hack);
//     println!("{extraction:?}");
// }

// STRING .retain =====================================
// fn main() {
//     let mut game_console = String::from("Playstation");
//     let mut deleted_chars = String::new();

//     let closure = |character| {
//         let is_not_a = character != 'a';
//         if is_not_a {
//             true
//         } else {
//             deleted_chars.push(character);
//             false
//         }
//     };
//     game_console.retain(closure);
//     println!("{game_console}");
//     println!("{deleted_chars}");
// }

// CLOSURE II CUSTOM METHODS ========================
// #[derive(Debug)]
// struct Location {
//     name: String,
//     treasure: u32,
// }

// #[derive(Debug)]
// struct Map<'a> {
//     locations: &'a [Location],
// }

// impl<'a> Map<'a> {
//     fn explore<F>(&self, mut action: F)
//     where
//         F: FnMut(&Location),
//     {
//         let final_idx = self.locations.len() - 1;
//         let mut current_index = 0;
//         while current_index <= final_idx {
//             let current_location = &self.locations[current_index];
//             action(current_location);
//             current_index += 1;
//         }
//     }
// }

// fn main() {
//     let locations = [
//         Location {
//             name: "Enchanted Forest".to_string(),
//             treasure: 5,
//         },
//         Location {
//             name: "Mystic Mountain".to_string(),
//             treasure: 10,
//         },
//     ];

//     let map = Map {
//         locations: &locations,
//     };
//     let mut total_treasures = 0;
//     map.explore(|location| {
//         total_treasures += location.treasure;
//     });

//     println!("Total treasures collected: {total_treasures}");

//     let mut location_names = Vec::new();

//     map.explore(|location| {
//         location_names.push(location.name.clone());
//     });

//     println!("{location_names:?}");
// }

// FN TRAIT REVIEW =========================================
// strictest FN type
// fn execute_thrice<F>(mut procedure: F)
// where
//     F: FnMut(),
// {
//     procedure();
//     procedure();
//     procedure();
// }

// fn main() {
//     let mut bosses = vec!["Boris"];
//     let closure = || {
//         bosses.push("Alexandria");
//     };
//     execute_thrice(closure);

//     println!("{:?}", bosses);
// }

// PASS FN IN FN TRAIT FUNCTION ==========================
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate");
}
fn main() {
    // execute_thrice(bake_cake);

    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);
}
