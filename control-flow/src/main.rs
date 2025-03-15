// IF ======================================
// fn main() {
//     let some_condition = true;

//     if some_condition {
//         println!("This line will be output!");
//     }

//     if false {
//         println!("This line will NOT be output.");
//     }
// }

// IF ELSE =================================
// fn main() {
//     let season = "spring";

//     if season == "summer" {
//         println!("School's out!");
//     } else if season == "winter" {
//         println!("Brr, so cold!");
//     // } else if season == "fall" {
//     //     println!("Leaves falling!");
//     // } else if season == "spring" {
//     //     println!("Lots of rain!");
//     } else {
//         println!("Lots of rain!");
//     }
// }

// IF RESULTS =================================
// fn even_or_odd(number: i32) {
//     let result = if number % 2 == 0 { "even" } else { "odd" };
//     println!("The number is {result}");
// }
// fn main() {
//     even_or_odd(17);
//     even_or_odd(100);
// }

// MATCH =================================
// fn main() {
//     let evaluation = false;

//     let value = match evaluation {
//         true => 20,
//         false => 40,
//     };

//     println!("{value}");
// }

// UNDERSCORE MATCH ARM =================================
// fn main() {
//     let season = "spring";

//     match season {
//         "summer" => println!("School's out!"),
//         "winter" => println!("Brr, so cold!"),
//         _ => println!("Lots of rain!"),
//     }
// }

// MATCH MULTIPLE =================================
// fn main() {
//     let number = 1337;

//     match number {
//         value if value % 2 == 0 => println!("{value} is an even number"),
//         x if x % 2 != 0 => println!("{x} is an odd number"),
//         _ => unreachable!(),
//     }
// }

// LOOPS AND BREAKS =================================
// fn main() {
//     let mut seconds = 21;

//     loop {
//         if seconds <= 0 {
//             println!("BLAST OFF!!!!!");
//             break;
//         }

//         if seconds % 2 == 0 {
//             println!("{seconds} seconds (even number), skipping 3 seconds...");
//             seconds -= 3;
//             continue;
//         }

//         println!("{seconds} seconds to blastoff..");
//         seconds -= 1;
//     }
// }

// WHILE LOOP =================================
// fn main() {
//     let mut seconds = 21;

//     while seconds > 0 {
//         if seconds % 2 == 0 {
//             println!("{seconds} seconds (even number), skipping 3 seconds...");
//             seconds -= 3;
//             continue;
//         }

//         println!("{seconds} seconds to blastoff..");
//         seconds -= 1;
//     }

//     println!("BLAST OFF!!!!!");
// }

// RECURSION =================================
fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
        return;
    }

    println!("{seconds} seconds to blastoff...");
    countdown(seconds - 1);
}

fn main() {
    countdown(5);
}
