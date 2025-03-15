fn main() {
    let mut trip = start_trip();
    visit_philly(&mut trip);
    trip.push_str(" and ");
    visit_ny(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push_str(".");
    show_itinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philly(trip: &mut String) {
    trip.push_str("Philadelphia");
}

fn visit_ny(trip: &mut String) {
    trip.push_str("New York");
}

fn visit_boston(trip: &mut String) {
    trip.push_str("Boston");
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}
