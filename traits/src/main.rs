use traits::{Accommodation, AirBnB, Description, Hotel, book_for_one_night, mix_and_match};
fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel.summarize());
    hotel.book("Dana", 5);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    book_for_one_night(&mut airbnb, "Dan");

    mix_and_match(&mut hotel, &mut airbnb, "Phil");
}
