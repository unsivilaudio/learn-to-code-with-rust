// OPTION ENUM INTRO ===================================
// fn main() {
//     let a = Option::Some(5);
//     let b = Option::Some("Hello");
//     let c = Option::Some(true);

//     let a: Option<i32> = Option::Some(32);
//     let a = Option::<i16>::Some(5);

//     let d: Option<&str> = Option::None;
// }

// OPTION ENUM (EXAMPLE) + UNWRAP =============================
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Bass"),
//     ];

//     let bass = musical_instruments.get(2);
//     println!("Found the {:?}", bass);
//     let valid_instrument = bass.expect("Unable to retrieve element");
//     println!("{valid_instrument}");

//     let invalid_instrument = musical_instruments.get(100);
//     invalid_instrument.expect("unable to retrieve instrument"); // panic with error message
// }

// OPTION MATCH =============================
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Bass"),
//     ];

//     let bass = musical_instruments.get(2);
//     play(bass);

//     let invalid_instrument = musical_instruments.get(100);
//     play(invalid_instrument);
// }

// fn play(instrument_option: Option<&String>) {
//     match instrument_option {
//         Option::Some(instrument) => println!("Playing the {instrument}"),
//         Option::None => println!("Singing with my voice"),
//     }
// }

// OPTION FROM FN ================================
// fn main() {
//     let availability = is_item_in_stock(true, false);

//     match availability {
//         // Option:: globally available
//         Some(true) => println!("Yes the item is available"),
//         Some(false) => println!("No the item is not available"),
//         None => println!("Your item doesn't exist in our system"),
//     }
// }

// fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
//     if item_is_in_system && item_is_in_stock {
//         Some(true)
//     } else if item_is_in_system {
//         Some(false)
//     } else {
//         None
//     }
// }

// OPTION - unwrap_or ============================
// fn main() {
//     let present_value = Some(13);
//     let missing_value: Option<i32> = None;

//     println!("{}", present_value.unwrap_or(0));
//     println!("{}", missing_value.unwrap_or(100));
// }

// OPTION -- from scratch =======================
// #[derive(Debug, Copy, Clone)]
// enum MyOption {
//     Some(i32),
//     None,
// }

// impl MyOption {
//     fn unwrap(self) -> i32 {
//         match self {
//             MyOption::Some(val) => val,
//             MyOption::None => panic!("Uh oh"),
//         }
//     }

//     fn unwrap_or(self, fallback: i32) -> i32 {
//         match self {
//             MyOption::Some(val) => val,
//             MyOption::None => fallback,
//         }
//     }
// }

// fn main() {
//     let some_option = MyOption::Some(100);
//     println!("{}", some_option.unwrap_or(26));

//     let none_option = MyOption::None;
//     println!("{}", none_option.unwrap_or(13));
// }

// RESULT ENUM -- Result:: ==============================
// fn main() {
//     let ok: Result<i32, &str> = Ok(5);
//     println!("{:?}", ok);
//     let disaster: Result<i32, &str> = Err("Something went wrong");
//     println!("{:?}", disaster);
// }

// RESULT (EXAMPLE) ==================================
// fn main() {
//     let text = "50";
//     let text_as_number = text.parse::<i32>();
//     println!("{:?}", text_as_number);

//     let text = "Alabama";
//     let text_as_number = text.parse::<i32>();
//     println!("{:?}", text_as_number);
// }

// RESULT FROM FN ============================
// fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Cannot divide by zero".to_string())
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result = divide(10.0, 0.0);

//     match result {
//         Ok(calculation) => println!("Result: {}", calculation),
//         Err(message) => println!("Error: {}", message),
//     }
// }

// RESULT METHODS ============================
// fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Cannot divide by zero".to_string())
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result = divide(10.0, 2.0);
//     println!("{}", result.is_ok());
//     println!("{}", result.is_err());
// }

// RESULT unwrap nuances =======================
// fn operation(great_success: bool) -> Result<&'static str, &'static str> {
//     if great_success {
//         Ok("Success")
//     } else {
//         Err("Error")
//     }
// }

// fn main() {
//     let my_result = operation(true);

//     let content = match my_result {
//         Ok(message) => message,
//         Err(error) => error,
//     };

//     println!("{}", my_result.unwrap());
// }

// WHILE LET CONSTRUCT ======================
fn main() {
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}")
    }
}
