trait Investment<T> {
    fn amount(&self) -> T;

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

struct Bonus {
    amount: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income = Income { amount: 50000.50 };
    let mut bonus = Bonus { amount: 10000.23 };
    let mut rust_programming_time = QualityTime { minutes: 1000 };
    income.double_amount();
    bonus.double_amount();
    rust_programming_time.double_amount();
    println!("{rust_programming_time:?}");
}
