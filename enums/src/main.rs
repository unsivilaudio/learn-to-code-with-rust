// ENUMS INTRO ============================================
// #[derive(Debug)]
// enum CardSuit {
//     Hearts,
//     Diamonds,
//     Spades,
//     Clubs,
// }

// struct Card {
//     rank: String,
//     suit: CardSuit,
// }

// fn main() {
//     let first_card = CardSuit::Hearts;
//     let mut second_card = CardSuit::Spades;
//     second_card = CardSuit::Clubs;
//     println!("{:?}", second_card);

//     let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
//     let card_suits = (CardSuit::Hearts, CardSuit::Clubs);
// }

// ASSOCIATED VALUES I ===============================
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal(String),
// }
// fn main() {
//     let visa = PaymentMethodType::CreditCard(String::from("4424-5678-9012-3456"));
//     println!("{:?}", visa);

//     let mastercard = PaymentMethodType::CreditCard(String::from("5502-6584-3354-1110"));
//     println!("{:?}", mastercard);
// }

// ASSOCIATED VALUES II ===============================
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal(String, String),
// }
// fn main() {
//     let mut my_payment_method = PaymentMethodType::CreditCard(String::from("4424-5678-9012-3456"));

//     my_payment_method = PaymentMethodType::PayPal(
//         String::from("happyface12"),
//         String::from("1dfwdv234kcklllksjfe343"),
//     );

//     println!("{:?}", my_payment_method);
// }

// STRUCT VARIANT ===============================
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal { username: String, password: String },
// }
// fn main() {
//     let visa = PaymentMethodType::CreditCard(String::from("1112-3354-4456-8753"));

//     let paypal = PaymentMethodType::PayPal {
//         username: String::from("bob"),
//         password: String::from("password"),
//     };

//     println!("{:?}", paypal);
// }

// NESTING ENUMS ==============================
// #[derive(Debug)]
// enum Beans {
//     Pinto,
//     Black,
// }
// #[derive(Debug)]
// enum Meat {
//     Chicken,
//     Steak,
// }

// #[derive(Debug)]
// enum ResturauntItem {
//     Burrito { meat: Meat, beans: Beans },
//     Bowl { meat: Meat, beans: Beans },
//     VeganPlate,
// }
// fn main() {
//     let lunch = ResturauntItem::Burrito {
//         meat: Meat::Steak,
//         beans: Beans::Pinto,
//     };
//     let dinner = ResturauntItem::Bowl {
//         meat: Meat::Chicken,
//         beans: Beans::Black,
//     };
//     let abandoned_meal = ResturauntItem::VeganPlate;

//     println!("Lunch was {lunch:?}, Dinner was {dinner:?}");
//     println!("Nobody ate {abandoned_meal:?}");
// }

// MATCH KEYWORD I/II =================================
// #[derive(Debug)]
// enum OperatingSystem {
//     Windows,
//     MacOs,
//     Linux,
// }
// fn main() {
//     let my_computer = OperatingSystem::MacOs;
//     let age = years_since_release(my_computer);
//     println!("My computer's operating system is {age} years old");

//     let dads_computer = OperatingSystem::Windows;
//     let age = years_since_release(dads_computer);
//     println!("My dad's computer is {age} years old");
// }

// fn years_since_release(os: OperatingSystem) -> u32 {
//     match os {
//         OperatingSystem::Windows => {
//             println!("Quite an old computer");
//             39
//         }
//         OperatingSystem::MacOs => 23,
//         OperatingSystem::Linux => 34,
//     }
// }

// MATCH KEYWORD III ===================================
// #[derive(Debug)]
// enum LaundryCycle {
//     Cold,
//     Hot { temperature: u32 },
//     Delicate(String),
// }

// fn main() {
//     wash_laundry(LaundryCycle::Cold);
//     wash_laundry(LaundryCycle::Hot { temperature: 100 });
//     wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
// }

// fn wash_laundry(cycle: LaundryCycle) {
//     match cycle {
//         LaundryCycle::Cold => {
//             println!("Running the laundry with cold temperature");
//         }
//         LaundryCycle::Hot { temperature } => {
//             println!("Running the laundry with a temperature of {temperature}");
//         }
//         LaundryCycle::Delicate(fabric_type) => {
//             println!("Running the laundry with a delicate cycle for {fabric_type}");
//         }
//     }
// }

// ENUM METHODS ===================================
// #[derive(Debug)]
// enum LaundryCycle {
//     Cold,
//     Hot { temperature: u32 },
//     Delicate(String),
// }

// impl LaundryCycle {
//     fn wash_laundry(&self) {
//         match self {
//             LaundryCycle::Cold => {
//                 println!("Running the laundry with cold temperature");
//             }
//             LaundryCycle::Hot { temperature } => {
//                 println!("Running the laundry with a temperature of {temperature}");
//             }
//             LaundryCycle::Delicate(fabric_type) => {
//                 println!("Running the laundry with a delicate cycle for {fabric_type}");
//             }
//         }
//     }
// }

// fn main() {
//     let cold_laundry = LaundryCycle::Cold;
//     cold_laundry.wash_laundry();

//     let hot_cycle = LaundryCycle::Hot { temperature: 100 };
//     hot_cycle.wash_laundry();

//     let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
//     delicate_cycle.wash_laundry();
// }

// MATCH KEYWORD IV ================================
// #[derive(Debug)]
// enum OnlineOrderStatus {
//     Ordered,
//     Packed,
//     Shipped,
//     Delivered,
// }

// impl OnlineOrderStatus {
//     fn check(&self) {
//         match self {
//             OnlineOrderStatus::Delivered => {
//                 println!("Your item has been delivered");
//             }
//             other_status => {
//                 println!("Your item is {:?}", other_status);
//             }
//         }
//     }
// }
// fn main() {
//     OnlineOrderStatus::Shipped.check();
// }

// MATCH KEYWORD V ==================================
// #[derive(Debug)]
// enum Milk {
//     Lowfat(i32),
//     Whole,
// }

// impl Milk {
//     fn drink(self) {
//         match self {
//             Milk::Lowfat(2) => {
//                 println!("Delicious, 2% milk is my favorite!");
//             }
//             Milk::Lowfat(percentage) => {
//                 println!("You've got the low fat {percentage} percent milk.");
//             }
//             Milk::Whole => {
//                 println!("You've got the whole milk!");
//             }
//         }
//     }
// }

// fn main() {
//     Milk::Lowfat(2).drink();
//     Milk::Whole.drink();
// }

// IF LET CONSTRUCT =================================
// #[derive(Debug)]
// enum Milk {
//     Lowfat(i32),
//     Whole,
//     NonDairy { kind: String },
// }
// fn main() {
//     let my_beverage = Milk::NonDairy {
//         kind: String::from("Oat"),
//     };

//     if let Milk::NonDairy { kind } = my_beverage {
//         println!("You beverage is {kind} milk.");
//     } else {
//         println!("You have some other variant.")
//     }
// }

// LET ELSE CONSTRUCT ===================================
#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::NonDairy {
        kind: String::from("Almond"),
    };

    let Milk::NonDairy { kind } = my_beverage else {
        println!("You do not have the non-dairy milk!");
        return;
    };

    println!("{kind} milk is available here");
}
