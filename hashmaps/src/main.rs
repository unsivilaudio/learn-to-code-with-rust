use std::collections::{HashMap, HashSet};

// HASHMAPS INTRO ===================================
// fn main() {
//     let mut menu: HashMap<String, f64> = HashMap::new();
//     menu.insert(String::from("Steak"), 29.99);
//     menu.insert(String::from("Tuna"), 13.99);
//     menu.insert(String::from("Burger"), 14.99);

//     println!("{menu:?}");

//     // let mut country_capital: HashMap<&str, &str> = HashMap::new();
//     // let mut country_capital = HashMap::<HashMap<&str, &str>>::new();
//     let mut country_capital = HashMap::new();
//     country_capital.insert("France", "Paris");
//     country_capital.insert("Germany", "Berlin");
// }

// HASHMAP from ====================================
// fn main() {
//     let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];
//     let mut years_at_company = HashMap::from(data);
//     println!("{years_at_company:?}");

//     let ben = years_at_company.remove("Ben");
//     println!("{:?}", ben);
//     println!("{}", ben.unwrap());
//     println!("{years_at_company:?}");

//     let ben = years_at_company.remove("Ben");
//     println!("{:?}", ben);
// }

// HASHMAP OWNERSHIP =================================
// fn main() {
//     let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairing.insert(&drink, &milk);
//     coffee_pairing.insert("Flat White", "ALmond Milk");
//     println!("{:?}", coffee_pairing);
//     println!("{drink} {milk}");
// }

// HASHMAP OWNERSHIP =================================
// fn main() {
//     let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairing.insert(&drink, &milk);
//     coffee_pairing.insert("Flat White", "ALmond Milk");

//     let value = coffee_pairing
//         .get("Cappuccino")
//         .copied()
//         .unwrap_or("Unknown Milk");
//     println!("{value}");
// }

// HASHMAP -- EXISTING KEYS =================================
// fn main() {
//     let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairing.insert(&drink, &milk);
//     coffee_pairing.insert("Flat White", "ALmond Milk");
//     println!("{coffee_pairing:?}");
//     coffee_pairing.insert("Latte", "Pistachio Milk");
//     println!("{coffee_pairing:?}");
// }

// HASHMAP -- ENTRY METHOD =================================
// fn main() {
//     let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairing.insert(&drink, &milk);
//     coffee_pairing.insert("Flat White", "ALmond Milk");

//     coffee_pairing.entry("Latte").or_insert("Pistachio Milk");
//     println!("{coffee_pairing:?}");
//     coffee_pairing
//         .entry("Cappuccino")
//         .or_insert("Pistachio Milk");
//     println!("{coffee_pairing:?}");
// }

// HASHSET ============================================
// fn main() {
//     let mut concert_queue: HashSet<&str> = HashSet::new();
//     println!("{concert_queue:?}");

//     concert_queue.insert("Molly");
//     concert_queue.insert("Megan");
//     println!("{concert_queue:?}");

//     concert_queue.insert("Molly");
//     println!("{concert_queue:?}");

//     println!("{}", concert_queue.remove("Molly"));
//     println!("{}", concert_queue.remove("Franny"));
//     println!("{concert_queue:?}");

//     println!("{}", concert_queue.contains("Molly"));
//     println!("{}", concert_queue.contains("Fred"));

//     println!("{:?}", concert_queue.get("Megan"));
//     println!("{:?}", concert_queue.get("Joe"));
// }

// HASHSET OPERATIONS =======================================
fn main() {
    let mut concert_queue = HashSet::new();
    let mut movie_queue = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{:?}", attendees.is_subset(&movie_queue));
    println!("{:?}", concert_queue.is_superset(&attendees));
}
