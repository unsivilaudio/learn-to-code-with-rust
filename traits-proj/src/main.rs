use std::fmt::{Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data())
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: &str, percentage: u32) -> Self {
        Self {
            calories,
            price,
            percentage,
            flavor: flavor.to_string(),
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }
    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Soda [ {},{:.2},{} ]",
            self.calories, self.price, self.flavor
        )
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T: Debug> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }
    fn get_data(&self) -> String {
        format!("A delicious {} ounce {:?}", self.ounces, self.kind)
    }
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("**Coffee --")
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

fn main() {
    let mut latte = Coffee::new("Latte", Milk::Almond, 20);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new("Cappuccino".to_string(), Milk::Whole, 16);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(100, 3.99, "Cherry", 100);
    println!("{}", pepsi);
    let mut coke = pepsi.clone();
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{:?}", coke);
}
