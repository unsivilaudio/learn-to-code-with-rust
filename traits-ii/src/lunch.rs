use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

fn main() {
    let lunch1 = Lunch { cost: 19.99 };
    let lunch2 = Lunch { cost: 29.99 };
    println!("{:?}", lunch1 + lunch2);
}
