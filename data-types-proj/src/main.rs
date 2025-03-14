fn main() {
    let num: i32 = 1_337;
    let num_i16 = num as i16;

    let floater = 3.1415;
    println!("{floater:.3}");

    let with_milk = false;
    let with_sugar = true;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let my_arr: [i8; 4] = [12, 24, 36, 42];
    dbg!(my_arr);

    let my_tup = (12, 3.14, true, my_arr);
    dbg!(my_tup);
}
