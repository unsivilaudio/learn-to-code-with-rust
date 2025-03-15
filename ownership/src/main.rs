// SCOPES ==============================
// fn main() {
//     let age = 32;
//     let is_handsome = true;

//     println!("{age}");
//     println!("{is_handsome}")

//     // age variable exists here
// } // age variable goes out of scope here

// COPY TRAIT =========================
// fn main() {
//     let time = 2025;
//     let year = time; // '2025' copied to stack -- again
// }

// STRING TYPE =========================
// fn main() {
//     let text = String::new();
//     let candy = String::from("KitKat");
// }

// push_str METHOD =========================
// fn main() {
//     let mut name = String::from("Boris");
//     println!("{name}");
//     name.push_str(" Pask");
//     println!("{name}");

//     name.push_str("haver");
//     println!("{name}");
// }

// MOVES AND OWNERSHIP =========================
// fn main() {
//     let person = String::from("Boris");
//     let genius = person;

//     println!("My name is {person}"); // ERROR -- 'person' not owner
// }

// DROP FUNCTION =========================
// fn main() {
//     let person = String::from("Boris");

//     drop(person);

//     let genius = person; // ERROR -- No heap value
// } // drop(person)

// clone METHOD =========================
// fn main() {
//     let person = String::from("Boris");
//     let genuis = person.clone(); // COPY ON HEAP

//     println!("This is {person}");
// }

// BORROWING (REFERENCE) =========================
// fn main() {
//     let my_stack_value = 2;
//     let my_integer_reference = &my_stack_value;

//     let my_heap_value = String::from("Toyota");
//     let my_heap_reference = &my_heap_value;
// }

// DEREFERENCE =========================
// fn main() {
//     let my_stack_value = 2;
//     let my_integer_reference = &my_stack_value;
//     println!("{}", *my_integer_reference);

//     let my_heap_value = String::from("Toyota");
//     let my_heap_reference = &my_heap_value;
//     println!("{}", my_heap_reference); // automatically de-referenced
// }

// STRING vs str =========================
// fn main() {
//     /*
//        String - A dynamic piece of text stored on the heap at runtime
//        &String ("ref String") - A reference to the heap String
//        str - a hardcoded, read-only piece of text encoded in the binary
//        &str ("ref str") - A reference to the text in the memory that has loaded the binary file
//     */
//     let ice_cream = "Cookies and Cream";
//     println!("{}", ice_cream);
// }

// COPY TRAIT REFERENCES =========================
// fn main() {
//     let ice_cream = "Cookies and Cream";
//     let desert = ice_cream;
//     println!("{ice_cream} {desert}");
// }

// OWNERSHIP WITH FUNCTION PARAMS =========================
// fn main() {
//     let oranges = String::from("Oranges");
//     print_my_value(oranges);
//     println!("{oranges} is still valid"); // GONE -- MOVED OWNERSHIP!!!!!
// }

// fn print_my_value(value: String) {
//     println!("Your value is {value}");
// }

// MUTABLE FUNCTION PARAMS =========================
// fn main() {
//     let burger = String::from("Burger");
//     add_fries(burger);
// }

// fn add_fries(mut meal: String) {
//     meal.push_str(" and Fries");
//     println!("{meal}");
// }

// RETURN VALUES I =========================
// fn main() {
//     let cake = bake_cake();
//     println!("I now have a {cake} cake")
// }

// fn bake_cake() -> String {
//     String::from("Chocolate Mousse")
// }

// RETURN VALUES II =========================
fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    println!("{current_meal}");
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}
