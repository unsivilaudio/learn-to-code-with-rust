// INTEGERS =====================================
// fn main() {
//     let eight_bit_signed: i16 = -32500;
//     let sixteen_bit_unassigned: u16 = 64000;

//     let thirty_two_bit_signed: i32 = -2147483648;
//     let thirty_two_bit_unsigned: u32 = 4294967295;

//     let some_value = 20u16;
// }

// _SEPARATORS =====================================
// fn main() {
//     let sixteen_bit_signed: i16 = 32_500;
// }

// USIZE and ISIZE =====================================
// fn main() {
//     // usize
//     // isize
//     let days: usize = 55;
//     let years: isize = -15_000;
// }

// STRINGS AND RAW STRINGS =====================================
// fn main() {
//     println!("Dear Emily,\nHow have you been?");
//     println!("\tOnce upon a time...");
//     println!("Juliet said \"I love you Romeo!\"");

//     let filepath = r"C:\My Documents\new\videos";
//     println!("{filepath}")
// }

// FUNCTIONS-METHODS =====================================
// fn main() {
//     let value: i32 = -15;
//     println!("{}", value.abs());

//     let empty_space = "      my content     ";
//     println!("{}", empty_space.trim());

//     println!("{}", value.pow(2));
//     println!("{}", value.pow(3));
// }

// FLOATING_POINTS =====================================
// fn main() {
//     let pi: f64 = 3.1415926535897932384;
//     println!("The current value of pi is {pi}");

//     println!("{}", pi.floor());
//     println!("{}", pi.ceil());
//     println!("{}", pi.round());
// }

// FLOATING_POINTS =====================================
// fn main() {
//     let pi: f64 = 3.1415926535897932384;
//     println!("The current value of pi is {:.4}", pi);
// }

// CASTING =====================================
// fn main() {
//     let miles_away = 50;
//     let miles_away_i8 = miles_away as i8;
//     let miles_away_u8 = miles_away as u8;

//     let miles_away = 100.329032;
//     let miles_away_f32 = miles_away as f32;
//     let miles_away_int = miles_away as i32;
//     println!("{miles_away_int}");
// }

// MATH =====================================
// fn main() {
//     let addition = 5 + 4;
//     let subtraction = 10 - 6;
//     let multiplication = 3 * 4;
//     println!("Addition: {addition}, Subtraction: {subtraction}, Multiplication: {multiplication}");

//     let floor_division = 5 / 3;
//     println!("{floor_division}");

//     let decimal_division = 5.0 / 3.0;
//     println!("{decimal_division}");

//     let remainder = 7 % 2;
//     println!("{remainder}");
// }

// AUGMENTED ASSIGNMENT OPERATOR =====================================
// fn main() {
//     let mut year = 2025;
//     year += 1;
//     println!("The new year is {year}");

//     year -= 5;
//     println!("The new year is {year}");

//     year *= 2;
//     println!("The new year is {year}");

//     year /= 4;
//     println!("The new year is {year}");
// }

// BOOLEAN =====================================
// fn main() {
//     let is_handsome = true;
//     let is_silly = false;

//     println!("Handsome: {is_handsome}, Silly: {is_silly}");

//     let age: i32 = 21;
//     let is_young = age < 35;
//     println!("{is_young}");
//     println!("{} {}", age.is_positive(), age.is_negative());
// }

// BOOLEAN INVERSION =====================================
// fn main() {
//     let age = 13;
//     let can_see_rated_r_movie = age >= 17;
//     let cannot_see_rated_r_movie = !can_see_rated_r_movie;
//     println!("I am {age} years old. Can I see this scary movie? {can_see_rated_r_movie}");
// }

// EQUALITY OPERATORS =====================================
// fn main() {
//     println!("{}", "Coke" == "Pepsi");
//     println!("{}", "Coke" != "Pepsi");
//     println!("{}", "Coke" == "coke");
//     println!("{}", "Coke" == "Coke ");
//     println!("{}", "Coke" == "Coke");

//     println!("{}", 13 == 13);
//     println!("{}", 13 != 13);

//     println!("{}", 26.1 == 26.1);
//     println!("{}", 26.14 == 26.1);

//     println!("{}", 13 == 13.0 as i32);

//     println!("{}", true == true);
//     println!("{}", false == false);
//     println!("{}", true != false);
// }

// && LOGIC =====================================
// fn main() {
//     let purchased_ticket = false;
//     let plane_on_time = false;
//     let making_event = purchased_ticket && plane_on_time;
//     println!("It is {} that I will arrive as expected", making_event);
// }

// || LOGIC =====================================
// fn main() {
//     let user_has_paid_for_subscription = true;
//     let user_is_admin = true;
//     let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
//     println!(
//         "Can this user see my site? {}",
//         user_can_see_premium_experience
//     );
// }

// CHARS TYPE =====================================
// fn main() {
//     let first_initial = 'J';
//     let emoji: char = '😂';

//     println!(
//         "{} {}",
//         first_initial.is_alphabetic(),
//         emoji.is_alphabetic()
//     );

//     println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
//     println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());
// }

// ARRAY TYPE =====================================
// fn main() {
//     let members: [i32; 6] = [4, 8, 15, 16, 23, 42];

//     let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
//     println!("length: {}", apples.len());

//     let currency_rates: [f64; 0] = [];
// }

// ARRAY TYPE (CONT.) =====================================
// fn main() {
//     let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

//     println!("{}", seasons[2]);
//     seasons[2] = "Autumn";
//     println!("{}", seasons[2]);
// }

// TRAITS & DBG =====================================
// fn main() {
//     let seasons = ["Spring", "Summer", "Fall", "Winter"];

//     println!("{:#?}", seasons);

//     dbg!(2 + 2);
// }

// TUPLES =====================================
// fn main() {
//     let employee = ("Molly", 32, "Marketing");

//     // let name = employee.0;
//     // let age = employee.1;
//     // let department = employee.2;

//     let (name, age, department) = employee;

//     println!("Name: {name}, Age: {age}, Department: {department}");
// }

// RANGES =====================================
// fn main() {
//     let month_days = 1..31;
//     println!("{:#?}", month_days);
//     let month_days = 1..=31;
//     println!("{:#?}", month_days);

//     // for number in month_days {
//     //     println!("{number}");
//     // }

//     let letters = 'b'..'f';

//     for letter in letters {
//         println!("{letter}");
//     }

//     let colors = ["Red", "Green", "Blue"];

//     for color in colors {
//         println!("{color} is a great color!");
//     }
// }

// GENERICS =====================================
fn main() {
    let month_days: std::ops::Range<i8> = 1..31;
    let letters: std::ops::Range<char> = 'b'..'f';
    // Value -- 5
    // Type -- i32
}
