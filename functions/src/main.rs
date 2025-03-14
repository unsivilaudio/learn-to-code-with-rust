// FUNCTIONS & ARGUMENTS ==============================
// fn main() {
//     open_store("Brooklyn");
//     bake_pizza(20, "pepporoni");
//     swim_in_profit();
//     swim_in_profit();
//     swim_in_profit();
//     open_store("Queens");
//     bake_pizza(15, "mushroom");
// }

// fn open_store(neighborhood: &str) {
//     println!("Opening my pizza store in {neighborhood}");
// }

// fn bake_pizza(number: i32, topping: &str) {
//     println!("Baking {number} {topping} pizza");
// }

// fn swim_in_profit() {
//     println!("So much $$$, so little time!")
// }

// RETURNS ===========================
// fn main() {
//     let result = square(5);
//     println!("The square of 5 is {result}");

//     let result = square(13);
//     println!("The square of 13 is {result}");
// }

// fn square(number: i32) -> i32 {
//     number * number
// }

// UNIT ===========================
// fn main() {
//     let result = mystery();
// }

// fn mystery() {
//     println!("Hello there");
// }

// FUNCTION BLOCKS ===========================
fn main() {
    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("{calculation}");
}
