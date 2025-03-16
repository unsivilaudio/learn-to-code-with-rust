#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }
}

impl Flight {
    fn change_destination(&mut self, destination: String) -> &mut Self {
        self.destination = destination;
        self
    }
    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.2;
        self
    }
    fn itinerary(&self) {
        println!("{} => {}", self.origin, self.destination)
    }
}

fn main() {
    let mut dublin_london = Flight::new(String::from("Dublin"), String::from("London"), 105.99, 32);
    dublin_london
        .change_destination(String::from("Whales"))
        .increase_price()
        .itinerary();

    let new_york_oslo = Flight {
        origin: String::from("New York"),
        destination: String::from("Oslo"),
        ..dublin_london
    };

    new_york_oslo.itinerary();

    println!("Flight 1: {dublin_london:#?}");
    println!("Flight 2: {new_york_oslo:#?}");
}
