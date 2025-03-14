const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "fall";
    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;

    points_scored = 35;
    #[allow(unused_variables)]
    let event_time = "06:00";
    let event_time = 6;

    let num_touchdowns = points_scored / TOUCHDOWN_POINTS;

    println!(
        "The best season is {season}, with {points_scored} scored points and {num_touchdowns} touchdowns, at {event_time} o'clock."
    );

    let _favorite_beverage = "Ice cold Coke.";
}
