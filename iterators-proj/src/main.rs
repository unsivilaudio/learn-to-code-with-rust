#[allow(unused, dead_code)]
use std::collections::HashSet;
use std::{collections::HashMap, env};

mod models;

use models::customer::Customer;
use models::customer_order::CustomerOrder;
use models::product::Product;

fn get_order_by<'a, P>(orders: &'a Vec<CustomerOrder>, procedure: P) -> Vec<&'a CustomerOrder>
where
    P: Fn(&CustomerOrder) -> bool,
{
    orders.iter().filter(|order| procedure(order)).collect()
}

fn get_order_by_filtermap<'a, P>(
    orders: &'a Vec<CustomerOrder>,
    procedure: P,
) -> Vec<&'a CustomerOrder>
where
    P: Fn(&CustomerOrder) -> Option<&CustomerOrder>,
{
    orders.iter().filter_map(|order| procedure(order)).collect()
}

fn sum_order_by_qty<'a, P>(orders: &'a Vec<CustomerOrder>, procedure: P) -> u32
where
    P: Fn(&CustomerOrder) -> u32,
{
    let mut sum_total: u32 = 0;
    for order in orders {
        let next_val = procedure(order);
        sum_total += next_val;
    }
    sum_total
}

fn get_orders_gte_qty<'a>(orders: &'a Vec<CustomerOrder>, qty: u32) -> Vec<&'a CustomerOrder> {
    orders
        .iter()
        .filter_map(|order| {
            if order.quantity >= qty {
                Some(order)
            } else {
                None
            }
        })
        .collect()
}

struct CLIArgs {
    quantity: u32,
}

fn get_cli_args() -> CLIArgs {
    let args = env::args().skip(1);
    let mut settings = args.map(|setting| setting.parse::<u32>());

    let args_quantity = settings.next();

    CLIArgs {
        quantity: args_quantity.unwrap_or(Ok(2)).unwrap(),
    }
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders = get_order_by(&orders, |order| order.product == Product::Blender);
    println!("Blender orders: {:?}", blender_orders);

    let microwaves_qty_total = sum_order_by_qty(&orders, |order| {
        if order.product == Product::Microwave {
            order.quantity
        } else {
            0
        }
    });

    println!("Total microwaves ordered: {}", microwaves_qty_total);

    let settings = get_cli_args();
    let quantity_orders = get_orders_gte_qty(&orders, settings.quantity);
    println!(
        "Showing orders with a quantity of {} or greater:",
        settings.quantity
    );
    println!("{:?}", quantity_orders);

    let pending_order_inventory =
        get_order_by_filtermap(
            &orders,
            |order| {
                if order.shipped { None } else { Some(order) }
            },
        );

    let mut hashmap_order_pending = HashMap::new();
    for order in pending_order_inventory {
        hashmap_order_pending
            .entry(&order.product)
            .and_modify(|item| *item = *item + order.quantity)
            .or_insert(order.quantity);
    }
    println!("Pending order totals: {:?}", hashmap_order_pending);

    let first_unshipped = orders.iter_mut().find(|order| !order.shipped);
    if let Some(order) = first_unshipped {
        order.shipped = true;
        println!("Changed shipped status: {:?}", order);
    }

    let mut customers = HashSet::<usize>::from_iter(customer_ids_by_order)
        .iter()
        .map(|id| Customer {
            id: *id,
            orders: Vec::new(),
        })
        .collect::<Vec<Customer>>();
    customers.sort_by_key(|customer| customer.id);
    for (order, id) in orders.into_iter().zip(customer_ids_by_order) {
        customers[id - 1].orders.push(order);
    }

    println!("Customer orders:");
    for customer in customers {
        println!("{:?}", customer);
    }
}
