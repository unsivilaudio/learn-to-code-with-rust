use super::customer_order::CustomerOrder;

#[derive(Debug)]
pub struct Customer {
    pub id: usize,
    pub orders: Vec<CustomerOrder>,
}
