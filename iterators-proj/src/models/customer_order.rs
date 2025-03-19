use super::product::Product;

#[derive(Debug)]
pub struct CustomerOrder {
    pub product: Product,
    pub quantity: u32,
    pub shipped: bool,
}

impl CustomerOrder {
    pub fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}
