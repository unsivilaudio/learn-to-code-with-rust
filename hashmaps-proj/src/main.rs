use std::{collections::HashMap, vec};

fn main() {
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::new();
    sauces_to_meals.insert("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]);
    sauces_to_meals.insert("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]);
    sauces_to_meals.insert("Mustard", vec!["Hot Dogs", "Burgers", "Pretzels"]);
    println!("{:?}", sauces_to_meals);

    sauces_to_meals.remove("Mayonnaise");
    match sauces_to_meals.get("Mustard") {
        Some(meals) => println!("The meals were {:?}", meals),
        None => println!("That sauce was not found."),
    };

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);
    println!("{:?}", sauces_to_meals);
}
