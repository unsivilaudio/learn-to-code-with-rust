// STRUCT INSTANCE/ACCESS/OVERWRITE ==================
// fn main() {
//     struct Coffee {
//         price: f64,
//         name: String,
//         is_hot: bool,
//     }

//     let mut beverage = Coffee {
//         name: String::from("Mocha"),
//         price: 4.99,
//         is_hot: false,
//     };

//     beverage.name = String::from("Caramel Machiato");
//     beverage.price = 6.99;
//     beverage.is_hot = true;

//     println!(
//         "My {} this morning cost {}. It is {} that it was hot",
//         beverage.name, beverage.price, beverage.is_hot
//     );
// }

// STRUCT IN FN ==================
// struct Coffee {
//     price: f64,
//     name: String,
//     is_hot: bool,
// }
// fn main() {
//     let mocha = make_coffee(String::from("Mocha"), 4.99, true);

//     let caramel_macchiato = Coffee {
//         name: mocha.name.clone(),
//         ..mocha
//     };

//     println!("{}", caramel_macchiato.name);
//     println!("{}", mocha.name);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// PASSING STRUCT TO FN / DERIVE-DEBUG ==================
// #[derive(Debug)]
// struct Coffee {
//     price: f64,
//     name: String,
//     is_hot: bool,
// }
// fn main() {
//     let mocha = make_coffee(String::from("Mocha"), 4.99, true);
//     // drink_coffee(&mut mocha);

//     // println!("{}", mocha);
//     println!("{:?}", mocha);
//     println!("{:#?}", mocha);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// fn drink_coffee(coffee: &mut Coffee) {
//     println!("Drinking my delicious {}", coffee.name);
//     coffee.is_hot = false;
//     coffee.price = 10.99
// }

// STRUCT METHODS =======================
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn display_song_info(self) {
//         // self - Imutable struct value (self parameter takes ownership)
//         // mut self - Mutable struct value (self parareter takes ownership, has permission to mutate)
//         // &self - Immutable reference to the struct instance (no ownership moved)
//         // &mut self - Mutable reference (no ownership moves, have permission to mutate)
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {} seconds", self.duration_secs);
//     }
// }
// fn main() {
//     let song = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     song.display_song_info();
// }

// STRUCT METHODS (EXAMPLES) =======================
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     // self - Imutable struct value (self parameter takes ownership)
//     // -> &self - Immutable reference to the struct instance (no ownership moved)
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {} seconds", self.duration_secs);
//     }
//     // mut self - Mutable struct value (self parameter takes ownership, has permission to mutate)
//     // -> &mut self - Mutable reference (no ownership moves, have permission to mutate)
//     fn double_length(&mut self) {
//         self.duration_secs *= 2;
//     }
// }
// fn main() {
//     let mut song = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     song.display_song_info();
//     song.double_length();
//     song.display_song_info();
// }

// STRUCT METHODS (MULTI PARAMS/SELF INVOKED METHODS) =======================
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {} seconds", self.duration_secs);
//         println!("Years since release: {}", self.years_since_release());
//     }
//     fn double_length(&mut self) {
//         self.duration_secs *= 2;
//     }
//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }
//     fn years_since_release(&self) -> u32 {
//         2025 - self.release_year
//     }
// }
// fn main() {
//     let blank_space = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };
//     blank_space.display_song_info();
// }

// ASSOCIATED FUNCTIONS/MULTIPLE IMPL =======================
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
//         Self {
//             title,
//             release_year,
//             duration_secs,
//         }
//     }
// }

// impl TaylorSwiftSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {} seconds", self.duration_secs);
//         println!("Years since release: {}", self.years_since_release());
//     }
//     fn double_length(&mut self) {
//         self.duration_secs *= 2;
//     }
//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }
//     fn years_since_release(&self) -> u32 {
//         2025 - self.release_year
//     }
// }
// fn main() {
//     let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);
//     blank_space.display_song_info();
// }

// BUILDER PATTERN =======================================
// #[derive(Debug)]
// struct Computer {
//     cpu: String,
//     memory: u32,
//     hard_drive_capacity: u32,
// }

// impl Computer {
//     fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
//         Self {
//             cpu,
//             memory,
//             hard_drive_capacity,
//         }
//     }
// }

// impl Computer {
//     fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
//         self.cpu = new_cpu;
//         self
//     }
//     fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
//         self.memory = new_memory;
//         self
//     }
//     fn upgrade_hard_drive(&mut self, new_hard_drive: u32) -> &mut Self {
//         self.hard_drive_capacity = new_hard_drive;
//         self
//     }
// }
// fn main() {
//     let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

//     computer
//         .upgrade_cpu(String::from("M4 Max"))
//         .upgrade_hard_drive(4)
//         .upgrade_memory(128);

//     println!("{:#?}", computer);
// }

// TUPLE STRUCTS =======================================
// struct ShortDuration(u32, u32); // hours, minutes

// struct LongDuration(u32, u32); // years, months
// fn main() {
//     let work_shift = ShortDuration(8, 0);
//     println!("{} hours and {} minutes", work_shift.0, work_shift.1);

//     let era = LongDuration(5, 3);
//     println!("{} years {} months", era.0, era.1);

//     go_to_work(work_shift);
//     // accept_tuple(work_shift); // -- ERROR WRONG TYPE

//     // let work_shift = (8, 0);
//     // let era = (5, 3);
//     // go_to_work(work_shift);
//     // go_to_work(era);
// }

// fn go_to_work(length: ShortDuration) {
//     println!("Passing time {} hours, {} minutes", length.0, length.1);
// }

// fn accept_tuple(length: (u32, u32)) {
//     println!("{}", length.0);
// }

// UNIT_LIKE STRUCTS =====================================
struct Empty;
fn main() {
    let my_empty_struct = Empty;
}
