// MANUAL ITERATIONS =================================
// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     // let mut current_index = 0;
//     // let final_index = numbers.len() - 1;

//     for value in numbers {
//         println!("{value}");
//     }

//     // while current_index <= final_index {
//     //     println!("{}", numbers[current_index]);
//     //     current_index += 1
//     // }

//     // loop {
//     //     if current_index > final_index {
//     //         break;
//     //     }

//     //     println!("{}", numbers[current_index]);
//     //     current_index += 1
//     // }
// }

// INTOITERATOR TRAIT ==================================
// use std::collections::HashMap;
// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     let my_iterator = my_vector.into_iter();

//     let my_vector = vec![true, false, true];
//     let my_iterator = my_vector.into_iter();

//     let mut my_hashmap = HashMap::new();
//     my_hashmap.insert("CBS", 2);
//     let my_iterator = my_hashmap.into_iter();
// }

// EXHAUSTING THE ITERATOR ===========================
// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     let mut my_iterator = my_vector.into_iter();
//     println!("{:?}", my_iterator);

//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());

//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator);
// }

// FOR LOOP ITERATOR ==============================
// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     let my_iterator = my_vector.into_iter();

//     for number in my_iterator {
//         // OWNERSHIP MOVED
//         println!("{number}");
//     }
// }

// ITER METHOD =========================
// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     // let my_iterator = my_vector.iter();

//     for number in &my_vector {
//         println!("{number}");
//     }

//     let cities = vec![String::from("Phoenix"), String::from("Dallas")];

//     for city in &cities {
//         println!("{city}");
//     }

//     println!("{cities:?}");
// }

// ITER_MUT ==============================
// fn main() {
//     let mut flavors = [
//         String::from("Chocolate"),
//         String::from("Vanilla"),
//         String::from("Strawberry"),
//     ];

//     // let iterator = flavors.iter_mut();

//     // for flavor in flavors.iter_mut() {
//     for flavor in &mut flavors {
//         flavor.push_str(" Ice Cream");
//     }

//     println!("{flavors:?}");

//     let mut school_grades = [85, 90, 72, 92];
//     for grade in &mut school_grades {
//         *grade -= 2;
//     }

//     println!("{school_grades:?}");
// }

// HASHMAP ITER ============================
// use std::collections::HashMap;

// fn main() {
//     let mut todos = HashMap::new();
//     todos.insert("Pickup Groceries", false);
//     todos.insert("Study Rust", true);
//     todos.insert("Sleep", false);

//     for (_todo, done) in &mut todos {
//         *done = true;
//     }

//     println!("{todos:?}");
// }

// STRING ITER =========================
// fn main() {
//     let seafood = "Oyster 🦪";

//     // for byte in seafood.bytes() {
//     //     print!("{byte}/");
//     // }

//     // for character in seafood.chars() {
//     //     print!("{character}/");
//     // }

//     // println!("{seafood}");

//     println!("{}", seafood.bytes().len());
//     println!("{}", seafood.chars().count());
// }

// ITER PROBLEMS
// use std::collections::HashMap;

// fn count_characters(text: &str) -> HashMap<char, u32> {
//     let mut counts = HashMap::new();

//     for word in text.split_whitespace() {
//         for character in word.chars() {
//             let count = counts.entry(character).or_insert(0);
//             *count += 1;
//         }
//     }

//     counts
// }

// fn main() {
//     println!(
//         "{:?}",
//         count_characters("Sally sells sea shells by the sea shore")
//     );
// }

// for_each METHOD ====================================================
// use std::collections::HashMap;

// fn count_characters(text: &str) -> HashMap<char, u32> {
//     let words = text.split_whitespace();
//     let mut counts = HashMap::new();

//     words.for_each(|word| {
//         word.chars().for_each(|character| {
//             let count = counts.entry(character).or_insert(0);
//             *count += 1;
//         });
//     });

//     counts
// }

// fn main() {
//     println!(
//         "{:?}",
//         count_characters("Sally sells sea shells by the sea shore")
//     );
// }

// map METHOD ==============================================
// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
//         println!("Square: {number}");
//     }
// }

// COLLECT METHOD ========================================
// use std::collections::HashSet;

// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];
//     // let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
//     let squares = numbers
//         .iter()
//         .map(|number: &i32| number.pow(2))
//         .collect::<HashSet<i32>>();

//     println!("{numbers:?}");
//     println!("{squares:?}");
// }

// MAP METHOD II ==============================================
// fn main() {
//     let names = [
//         String::from("Jimmy"),
//         String::from("Cleveland"),
//         String::from("Boris"),
//     ];

//     let name_lengths = names
//         .iter()
//         .map(|name| name.to_lowercase())
//         .map(|name| name.replace('i', "@@"))
//         .map(|name| name.len())
//         .collect::<Vec<usize>>();

//     println!("{name_lengths:?}");
// }

// FILTER METHOD I =========================================
// fn main() {
//     let numbers = [10, 13, 23, 2, 8, 9, 6];

//     let evens: Vec<i32> = numbers
//         .iter()
//         .filter(|val| *val % 2 == 0)
//         .copied()
//         .collect();

//     println!("{:?}", evens);
//     println!("{:?}", numbers);

//     // .rfind -- reverse iterator find

//     let first_even = numbers.into_iter().find(|number| number % 2 == 0);
//     let first_odd = numbers.into_iter().find(|number| number % 2 == 1);

//     println!("{:?}", first_even);
//     println!("{:?}", first_odd);

//     let nothing = numbers.into_iter().find(|number| *number > 100);
//     println!("{:?}", nothing);
// }

// FILTER AND FIND II ===========================
// #[derive(Debug, PartialEq, Eq)]
// enum ChannelType {
//     Comedy,
//     News,
//     ProgrammingTutorials,
// }

// #[derive(Debug)]
// struct TVChannel {
//     name: String,
//     channel_type: ChannelType,
// }

// fn main() {
//     let channels = [
//         TVChannel {
//             name: String::from("CBS"),
//             channel_type: ChannelType::Comedy,
//         },
//         TVChannel {
//             name: String::from("RustLive"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//         TVChannel {
//             name: String::from("NBC"),
//             channel_type: ChannelType::News,
//         },
//         TVChannel {
//             name: String::from("RustTV"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//     ];

//     let good_channels: Vec<String> = channels
//         .iter()
//         .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
//         .map(|channel| channel.name.clone())
//         .collect();
//     println!("{:?}", good_channels);

//     let good_channel = channels
//         .iter()
//         .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
//     println!("{:?}", good_channel);
// }

// ANY METHOD =======================================================
// #[derive(Debug, PartialEq, Eq)]
// enum ChannelType {
//     Comedy,
//     News,
//     ProgrammingTutorials,
// }

// #[derive(Debug)]
// struct TVChannel {
//     name: String,
//     channel_type: ChannelType,
// }

// fn main() {
//     let channels = [
//         TVChannel {
//             name: String::from("CBS"),
//             channel_type: ChannelType::Comedy,
//         },
//         TVChannel {
//             name: String::from("RustLive"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//         TVChannel {
//             name: String::from("NBC"),
//             channel_type: ChannelType::News,
//         },
//         TVChannel {
//             name: String::from("RustTV"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//     ];

//     let all_are_rust = channels
//         .iter()
//         .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
//     println!("{all_are_rust}");

//     let any_are_rust = channels
//         .iter()
//         .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
//     println!("{any_are_rust}");
// }

// CLONE ITERATOR ===============================================================
// fn main() {
//     let teas = [
//         String::from("Hot Earl Grey"),
//         String::from("Iced Green"),
//         String::from("Hot Matcha"),
//     ];

//     let more_teas: Vec<_> = teas
//         .iter()
//         .filter(|tea| tea.contains("Hot"))
//         .cloned()
//         .collect();
//     println!("{:?}", more_teas);
// }

// FILTER_MAP ========================================
// fn main() {
//     let stocks = ["nvda", "", "appl", "msft", "goog"];

//     let capitalized_stocks: Vec<String> = stocks
//         .iter()
//         .filter(|stock| !stock.is_empty())
//         .map(|stock| stock.to_uppercase())
//         .collect();
//     println!("{:?}", capitalized_stocks);

//     let capitalized_stocks: Vec<String> = stocks
//         .iter()
//         .filter_map(|stock| {
//             if stock.is_empty() {
//                 None
//             } else {
//                 Some(stock.to_uppercase())
//             }
//         })
//         .collect();
//     println!("{:?}", capitalized_stocks);
// }

// FLATTEN METHOD ==============================
// fn main() {
//     let spreadsheet = [[100, 200, 300], [123, 456, 789], [987, 654, 321]];

//     let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
//     println!("{values:?}");
// }

// FLAT_MAP ====================================
// fn main() {
//     let attendees = [
//         "Bob, Mary, Kevin",
//         "Mike, Robbie, Matt, Austin",
//         "Piers, Liam",
//     ];

//     // let attendees: Vec<&str> = attendees
//     //     .iter()
//     //     .map(|group| group.split(", "))
//     //     .flatten()
//     //     .collect();
//     let attendees: Vec<&str> = attendees
//         .iter()
//         .flat_map(|group| group.split(", "))
//         .collect();
//     println!("{attendees:?}");
// }

// ENUMERATE METHOD =====================================
// fn main() {
//     let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];

//     let winners: Vec<&str> = applicants
//         .into_iter()
//         .enumerate()
//         .filter_map(
//             |(idx, applicant)| {
//                 if idx % 3 == 0 { Some(applicant) } else { None }
//             },
//         )
//         .collect();

//     println!("{winners:?}");
// }

// PARITION METHOD ======================================
// fn main() {
//     let numbers = [4, 8, 15, 16, 23, 42];

//     let groups: (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| number % 2 == 0);

//     println!("{:?}", groups.0);
//     println!("{:?}", groups.1);
// }

// ZIP METHOD ========================================
// fn main() {
//     let first_names = ["Casey", "Robert", "Cargo"];
//     let last_names = ["Johnson", "Smith", "Rustman"];

//     for (first, last) in first_names.iter().zip(last_names) {
//         println!("{} {}", first, last);
//     }

//     let complete_names: Vec<String> = first_names
//         .iter()
//         .zip(last_names)
//         .map(|(first, last)| format!("{first} {last}"))
//         .collect();

//     println!("{complete_names:?}");
// }

// FOLD METHOD =====================================
// use std::collections::HashMap;

// #[derive(Debug)]
// struct SupportStaff {
//     day: String,
//     employee: String,
// }

// fn main() {
//     let earnings = [4, 7, 9, 13];

//     let sum = earnings.into_iter().fold(0, |acc, current| acc + current);
//     println!("Total: {sum}");

//     let week = [
//         SupportStaff {
//             day: String::from("Monday"),
//             employee: String::from("Brian"),
//         },
//         SupportStaff {
//             day: String::from("Tuesday"),
//             employee: String::from("Cam"),
//         },
//         SupportStaff {
//             day: String::from("Wednesday"),
//             employee: String::from("Walter"),
//         },
//     ];

//     let map = week.into_iter().fold(HashMap::new(), |mut data, entry| {
//         data.insert(entry.day, entry.employee);
//         data
//     });
//     println!("{map:?}");
// }

// REDUCE METHOD =======================================
// fn main() {
//     let earnings = [4, 7, 9, 13];

//     let sum = earnings.into_iter().reduce(|acc, current| acc + current);
//     println!("{:?}", sum);

//     let address_portion = [
//         String::from("123 Elm Street"),
//         String::from("Suburbia"),
//         String::from("New Jersey"),
//     ];

//     println!("{}", address_portion.join(", "));

//     let address = address_portion.into_iter().reduce(|mut acc, portion| {
//         acc.push_str(", ");
//         acc.push_str(&portion);
//         acc
//     });
//     println!("{address:?}");
// }

// SUM/PRODUCT/MIN/MAX ====================================================
// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     let total: i32 = numbers.iter().sum();
//     println!("{total}");

//     let product: i32 = numbers.iter().product();
//     println!("{product}");

//     let min = numbers.iter().min();
//     println!("{min:?}");

//     let max = numbers.iter().max();
//     println!("{max:?}");

//     let count = numbers.iter().count();
//     println!("{count}");

//     let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];
//     println!("{numbers:?}");
//     let total: f64 = numbers
//         .iter()
//         .filter(|number| !number.is_nan())
//         .copied()
//         .fold(0.0, |acc, val| acc + val);
//     println!("{total}");

//     let max: Option<f64> = numbers
//         .iter()
//         .filter(|number| !number.is_nan())
//         .copied()
//         .reduce(|acc, val| acc.max(val));
//     println!("{max:?}");
// }

// LAST/NTH/NTH_BACK/POSTION ================================
// fn main() {
//     let performers = ["Rustful Five", "Rust In Peace", "Rustin Bieber"];

//     let last = performers.into_iter().last().unwrap();
//     println!("{last}");

//     let n_position = performers.into_iter().nth(1).unwrap();
//     println!("{n_position}");

//     let n_reverse_position = performers.into_iter().nth_back(1).unwrap();
//     println!("{n_reverse_position}");

//     let target_index = performers.into_iter().position(|el| el == "Rustin Bieber");
//     println!("{:?}", target_index);
// }

// TAKE/REV/SKIP/STEP_BY =======================================
// fn main() {
//     let fifty_numbers = 1..=50;

//     for number in fifty_numbers.clone().take(15).skip(5).step_by(2) {
//         print!("{number}/");
//     }

//     println!("{fifty_numbers:?}");
// }

// SORT/SORT_BY ==================================================
// #[derive(Debug)]
// struct GasStation {
//     snack_count: u32,
//     manager: String,
//     employee_count: u32,
// }
// fn main() {
//     let mut points = [3, 8, 11, 5, 1];
//     println!("{}", points.is_sorted());

//     points.sort();
//     println!("{points:?}");

//     points.reverse();
//     println!("{points:?}");
//     println!("{}", points.is_sorted());

//     let mut exercises = ["squat", "bench", "deadlift"];
//     exercises.sort();
//     println!("{:?}", exercises);

//     let mobil = GasStation {
//         snack_count: 100,
//         manager: String::from("Meg Mobil"),
//         employee_count: 3,
//     };
//     let exxon = GasStation {
//         snack_count: 130,
//         manager: String::from("Eric Exxon"),
//         employee_count: 4,
//     };
//     let shell = GasStation {
//         snack_count: 50,
//         manager: String::from("Shane Shell"),
//         employee_count: 2,
//     };

//     let mut stops = [mobil, exxon, shell];
//     stops.sort_by_key(|station| -(station.employee_count as i32));
//     println!("{:?}", stops);
// }

// LINES METHOD ===================================================
// use std::fs;
// use std::io::Result;
// fn main() -> Result<()> {
//     let contents = fs::read_to_string("story.txt")?;

//     for line in contents.lines() {
//         println!("{line}");
//     }

//     Ok(())
// }

// COMMAND LINE ARGUMENTS ==========================================
// use std::{env, process};

// #[derive(Debug)]
// struct Settings {
//     video_file: String,
//     subtitles: bool,
//     high_definition: bool,
// }

// fn main() {
//     let settings = collect_settings();
//     println!("{settings:?}");
// }

// fn collect_settings() -> Settings {
//     let mut args = env::args().skip(1).take(3);

//     let video_file = args.next().unwrap_or_else(|| {
//         eprintln!("No video file specified.");
//         process::exit(1);
//     });

//     let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

//     let subtitles = settings.next().unwrap_or(false);
//     let high_definition = settings.next().unwrap_or(false);

//     Settings {
//         video_file,
//         subtitles,
//         high_definition,
//     }
// }

// READING FILES FROM SYSTEM
// use std::fs;
// use std::io::Result;

// fn main() -> Result<()> {
//     for entry_result in fs::read_dir("./")? {
//         let entry = entry_result?;
//         let entry_name = entry.path();
//         let metadata = fs::metadata(&entry_name)?;
//         if metadata.is_file() {
//             println!("{entry_name:?}\n----------------------");
//             let content = fs::read_to_string(&entry_name)?;
//             println!("{content}");
//         }
//     }

//     Ok(())
// }

// FROMITERATOR TRAIT ====================
use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }

        Self { songs, users }
    }
}

fn main() {
    // let fifty_numbers = 1..=50;

    // let result = Vec::from_iter(fifty_numbers.clone());
    // println!("{result:?}");

    // let results = fifty_numbers.clone().collect::<Vec<i32>>();
    // println!("{results:?}");

    // let unique_set: HashSet<i32> = HashSet::from_iter(fifty_numbers.clone());
    // println!("{unique_set:?}");

    // let unique_set: HashSet<i32> = fifty_numbers.clone().collect::<HashSet<i32>>();
    // println!("{unique_set:?}");

    // let chars = ['H', 'e', 'l', 'l', 'o'];
    // let greeting = String::from_iter(chars);
    // println!("{greeting}");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    // let playlist = Playlist::from_iter(songs);
    // println!("{playlist:?}");

    let playlist = songs.into_iter().collect::<Playlist>();
    println!("{playlist:?}");
}
