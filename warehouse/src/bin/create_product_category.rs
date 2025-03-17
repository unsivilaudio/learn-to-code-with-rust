use fake::{Fake, Faker};

use warehouse::ProductCategory;

/// Create fictional category
fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
