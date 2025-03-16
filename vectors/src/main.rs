// VECTORS INTRO ===============================================
// fn main() {
//     // let pizza_diameters = Vec::<i32>::new();
//     let pizza_diameters = vec![8, 10, 12, 14];
//     println!("{pizza_diameters:?}");

//     // let pastas: Vec<&str> = Vec::new();
//     let pastas = vec!["Rigatoni", "Angel Hair", "Fettucine"];
//     println!("{pastas:?}");
// }

// VECTORS ADD/REMOVE ELEMENTS ================================
// fn main() {
//     let mut pizza_diameters = vec![8, 10, 12, 14];
//     pizza_diameters.push(16);
//     pizza_diameters.push(18);

//     pizza_diameters.insert(0, 4);

//     let last_pizza_diameter = pizza_diameters.pop();
//     println!("{last_pizza_diameter:?}");

//     let third_diameter_from_start = pizza_diameters.remove(2);
//     println!("{third_diameter_from_start:?}");

//     println!("{pizza_diameters:?}");
// }

// READING VECTORS ================================
// fn main() {
//     let pizza_diameters = vec![8, 10, 12, 14];

//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Saussage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let pizza_slice = &pizza_toppings[1..3]; // OWNERSHIP WILL TRY TO MOVE -- String
//     println!("{pizza_slice:?}");
// }

// GET METHOD ================================
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Saussage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let option = pizza_toppings.get(2);

//     match option {
//         Some(topping) => println!("The topping is {topping}"),
//         None => println!("No value at that index"),
//     }
// }

// OWNERSHIP VECTORS ================================
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Saussage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];
//     let mut delicious_toppings = pizza_toppings;

//     let toppings_reference = &delicious_toppings[1];
//     println!("{toppings_reference}");

//     delicious_toppings.push(String::from("Olives"));
// }

// WRITING VECTOR ELEMENTS ================================
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Saussage");
//     let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

//     pizza_toppings[1] = String::from("Olives");
//     println!("{:#?}", pizza_toppings);

//     let target_topping = &mut pizza_toppings[2];
//     target_topping.push_str(" and Meatballs");
//     let another_topping = &pizza_toppings[2];
//     println!("{:#?}", pizza_toppings);
//     println!("{another_topping}");
// }

// VECTOR CAPACITY ====================================
fn main() {
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Spring");
    seasons.push("Winter");
    println!(
        "length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    println!(
        "length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
