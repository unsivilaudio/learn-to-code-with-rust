// INVALID LIFETIMES ===============================
// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];

//     let favorite_cities = &cities[0..2];
//     println!("{:?}", favorite_cities);
//     let places = cities;
// }

// FUNCTION REFERENCES =========================
// fn create() -> i32 {
//     let age = 100;
//     &age
// } // INVALID REFERENCE RETURN
// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items[0..2]
// } // SAME AS ABOVE
// fn create_number_reference(number: i32) -> &i32 {
//     &number
// } // SAME -- DANGLING REFERENCe

// REFERENCE FN PARAMS ======================
// fn select_first_two_elements(items: &[String]) -> &[String] {
//     &items[0..2]
// }
// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = select_first_two_elements(&cities);
//     drop(cities);
//     println!("{:?}", two_cities);

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{:?}", two_coffees);
//     }
// }

// GENERIC LIFETIMES ====================================
// fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
//     &items[0..2]
// }
// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = select_first_two_elements(&cities);
//     println!("{:?}", two_cities);

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{:?}", two_coffees);
//     }
// }

// LIFETIMES AND REFERENTS ====================================
// fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
//     &items[0..2]
// }
// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = {
//         let cities_reference = &cities;
//         select_first_two_elements(cities_reference)
//     };
//     println!("{:?}", two_cities); // cities => cities_reference => two_cities

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{:?}", two_coffees);
//     }
// }

// LIFETIME ELISION (turn on 'Elision hints') ====================================
// fn my_awesome_function(first: String, value: &i32) -> &i32 {
//     value
// }

// fn select_first_two_elements(items: &[String]) -> &[String] {
//     &items[0..2]
// }
// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = {
//         let cities_reference = &cities;
//         select_first_two_elements(cities_reference)
//     };
//     println!("{:?}", two_cities); // cities => cities_reference => two_cities

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{:?}", two_coffees);
//     }
// }

// MULTIPLE PARAMS =======================================================
// fn choose_favorite<'a>(first: &str, second: &'a str) -> &'a str {
//     println!("{first}");
//     second
// }

// fn main() {}

// MULTIPLE PARAMS II ========================================
// fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
//     println!("The second is {second}");
//     first
// }
// fn main() {
//     let orlando = String::from("Orlando");
//     let result = {
//         let san_fransisco = String::from("San Fransisco");
//         longest(&orlando, &san_fransisco)
//     };
//     println!("{result}");
// }

// LIFETIME ELISION RULES II ===================================
// struct DentistAppointment {
//     doctor: String,
// }

// impl DentistAppointment {
//     // DEFAULT LIFETIME ANNOTATION -- '0,'1,'2....
//     // fn book<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_time: &'c str) -> &'a str {
//     fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
//         println!(
//             "You are booked from {} to {} with doctor {}",
//             check_in_time, check_out_time, self.doctor
//         );
//         check_in_time
//     }
// }

// fn main() {
//     let appt = DentistAppointment {
//         doctor: String::from("David"),
//     };

//     let result = appt.book("08:30", "09:15");
//     drop(appt);
//     println!("{result}");
// }

// LIFETIME IN STRUCTS =============================================
// #[derive(Debug)]
// struct TrainSystem<'a> {
//     name: &'a str,
// }

// fn main() {
//     let name = String::from("NJ Transit");
//     let nj_transit = { TrainSystem { name: &name } };

//     println!("{nj_transit:?}");
// }

// MULTIPLE LIFETIMES =======================================
// #[derive(Debug)]
// struct TravelPlan<'a, 'b> {
//     from: &'a str,
//     to: &'b str,
// }
// fn main() {
//     let from = String::from("Portland");
//     let plan = figure_out_ending_point(&from);

//     println!("{plan}");
// }

// fn figure_out_ending_point(from: &str) -> &str {
//     let to = String::from("Bangor");

//     let travel_plan = TravelPlan {
//         from: &from,
//         to: &to,
//     };

//     travel_plan.from
// }

// STATIC LIFETIMES ===================================
const COUNT: i32 = 400;

fn say_hello() -> &'static str {
    "Hello"
}

fn value() -> &'static i32 {
    &COUNT
}

fn main() {
    let greeting = say_hello();
    println!("{greeting}");

    let count = value();
    println!("{count}");
}
