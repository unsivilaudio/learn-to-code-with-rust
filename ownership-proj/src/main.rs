// COPY TRAIT ==============================
// fn main() {
//     let is_concert = true;
//     let is_event = is_concert;
//     // boolean copied???!?! --- YESS!!!!!
//     println!("{is_concert}");

//     let sushi = "Salmon";
//     let dinner = sushi;
//     // str -- COPIED!!!
//     println!("{sushi}");
// }

// HEAP MOVES ===============================
// fn main() {
//     let sushi = String::from("Salmon");
//     let dinner = sushi;
//     // STRING -- MOVED!!!
//     // println!("{sushi}"); // NOPE -- OWNERSHIP MOVED
//     println!("{dinner}");
// }

// clear METHOD ==========================
fn main() {
    let mut sushi = String::from("Salmon");
    sushi = eat_meal(sushi); // OWNERSHIP MOVED TO FN
    println!("Meal is: {}", sushi);
}

fn eat_meal(mut meal: String) -> String {
    meal.clear(); // MEAL STRING EMPTIED
    meal // MEAL -- EMTPY STRING RETURNED
}
