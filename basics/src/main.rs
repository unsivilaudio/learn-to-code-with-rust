// VARIABLES =============================
// fn main() {
//     let apples = 50;
//     let oranges = 6 + 15;
//     let _fruits = apples + oranges;
//     println!(
//         "This year, my garden has {} apples and {} oranges.",
//         apples, oranges
//     );
//     // println!("I have {fruits} total fruits.")
// }

// MUTABLES =============================
// fn main() {
//     let mut gym_reps = 10;
//     println!("I plan to do {gym_reps} reps");

//     gym_reps = 15;
//     println!("I now plan to do {gym_reps} reps");
// }

// VARIABLE SHADOWING ===================
// fn main(){
//     let grams_of_protein = "100.345";
//     let grams_of_protein = 100.345;
//     let mut grams_of_protein = 100;
//     grams_of_protein = 105;
// }

// SCOPES ===================
// fn main() {
//     let coffee_price = 5.99;

//     {
//         println!("The price is {coffee_price}");
//         let cookie_price = 1.99;
//         println!("The cookie price is {:#?}", cookie_price);
//     }
// }

// CONSTANTS ===================
// const TAX_RATE: f64 = 7.25;
// fn main() {
//     let income: i32 = 100000;
//     println!("My income is {income} and the tax rate is {TAX_RATE}");
// }

// TYPE ALIAS ===================
// type Meters = i32;
// fn main() {
//     let mile_race_length: Meters = 1600;
//     let two_mile_race_length: Meters = 3200;
//     println!(
//         "A one mile is {mile_race_length} meters long, a two mile race is {two_mile_race_length} long."
//     );
// }

// COMPILER DIRECTIVE ===================
#![allow(unused_variables)]
type Meters = i32;

fn main() {
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
}
