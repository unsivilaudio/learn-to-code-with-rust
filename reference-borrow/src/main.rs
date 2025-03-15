// IMMUTABLE vs MUTABLE ===============================
// fn main() {
//     let mut current_meal = String::new();
//     add_flour(&mut current_meal);
//     show_my_meal(&current_meal);
// }

// fn add_flour(meal: &mut String) {
//     meal.push_str("Add flour");
// }

// fn show_my_meal(meal: &String) {
//     println!("Meal steps: {meal}");
// }

// MULTI IMMUT REFS ===============================
// fn main() {
//     let mut car = String::from("Red");
//     let ref1 = &mut car;
//     ref1.push_str(" and Silver");
//     let ref2 = &car;
//     println!("{ref2}");
// }

// OWNERSHIP IMMUT/MUT REFS ===============================
// fn main() {
//     let mut coffee = String::from("Mocha");
//     let a = &mut coffee;
//     println!("{a}");
//     let b = a;
//     println!("{b}");
// }

// DANGLING REFS ===============================
// fn main() {
//     let city = create_city();
//     println!("{city}");
// }

// fn create_city() -> &String {
//     let city = String::from("New York");
//     &city
// } // 'city' de-allocated -- DANGLING REFERENCE

// OWNERSHIP - ARR/TUP ===============================
fn main() {
    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{first} and {registrations:?}");

    // let languages = [String::from("Rust"), String::from("Javascript")];
    // let first = &languages[0];
    // println!("{first} and {languages:?}");

    let languages = (String::from("Rust"), String::from("Javascript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");
}
