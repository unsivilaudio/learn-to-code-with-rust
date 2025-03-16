// GENERICS INTRO =======================================
// fn main() {
//     println!("{}", identity(5));
//     println!("{}", identity(13.14));
//     println!("{}", identity("Hello"));
//     println!("{}", identity(String::from("World")));
// }

// fn identity<T>(value: T) -> T {
//     value
// }

// TURBOFISH OPERATOR =======================================
// #[derive(Debug)]
// struct DeliSandwich {}

// fn main() {
//     println!("{}", identity::<i32>(5));
//     println!("{}", identity::<f64>(13.14));
//     println!("{}", identity::<&str>("Hello"));
//     println!("{}", identity::<String>(String::from("World")));
//     println!("{}", identity::<bool>(true));
//     println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
// }

// fn identity<T>(value: T) -> T {
//     value
// }

// MULTIPLE GENERICS =======================================
// fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
//     (first, second)
// }
// fn main() {
//     make_tuple(5, "world");
//     make_tuple(5, 13);
//     make_tuple(true, 3.14);
//     make_tuple(true, false);
// }

// GENERICS IN STRUCTS + IMPL BLOCKS =======================================
// #[derive(Debug)]
// struct TreasureChest<T> {
//     captain: String,
//     treasure: T,
// }

// impl TreasureChest<String> {
//     fn clean_treasure(&mut self) {
//         self.treasure = self.treasure.trim().to_string()
//     }
// }

// impl TreasureChest<[&str; 3]> {
//     fn amount_of_treasure(&self) -> usize {
//         self.treasure.len()
//     }
// }

// impl<T> TreasureChest<T> {
//     fn capital_captain(&self) -> String {
//         self.captain.to_uppercase()
//     }
// }

// fn main() {
//     let gold_chest = TreasureChest {
//         captain: String::from("Firebird"),
//         treasure: "Gold",
//     };
//     println!("{:?}", gold_chest);
//     println!("{}", &gold_chest.capital_captain());

//     let mut silver_chest = TreasureChest {
//         captain: String::from("Bloodsail"),
//         treasure: String::from("          Silver    "),
//     };

//     println!("{:?}", silver_chest);
//     silver_chest.clean_treasure();
//     println!("{:?}", silver_chest);

//     let special_chest = TreasureChest {
//         captain: String::from("Bootyplunder"),
//         treasure: ["Gold", "Silver", "Platinum"],
//     };
//     println!("{:?}", special_chest);
//     println!("{}", special_chest.amount_of_treasure());
// }

// GENERICS IN ENUMS ====================================
#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Topping(T),
}
fn main() {
    let mushroom = CheeseSteak::Topping("mushroom");
    let onion = CheeseSteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = CheeseSteak::Topping(&topping);

    let mut plain: CheeseSteak<String> = CheeseSteak::Plain;
    plain = CheeseSteak::Topping("green peppers".to_string());
}
