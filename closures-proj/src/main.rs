#[derive(Debug)]
struct SuperMarketItem {
    name: String,
    price: f64,
}

impl SuperMarketItem {
    fn discount_item(&mut self, discount: f64) {
        self.price *= 1.0 - discount;
        self.price = f64::trunc(self.price * 100.0) / 100.0;
    }
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SuperMarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut next: F)
    where
        F: FnMut(&mut SuperMarketItem),
    {
        let i = self.items.iter_mut();
        for item in i {
            next(item);
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self);
    }
}

fn main() {
    let mut shopping_cart = ShoppingCart {
        items: Vec::from([
            SuperMarketItem {
                name: String::from("APPLE"),
                price: 3.99,
            },
            SuperMarketItem {
                name: String::from("BANANA"),
                price: 2.99,
            },
        ]),
    };

    shopping_cart.traverse_items(|item| {
        item.discount_item(0.15);
    });

    shopping_cart.traverse_items(|item| {
        item.name = item.name.to_lowercase();
    });

    let mut total_price = 0.0;

    shopping_cart.checkout(|mut cart| {
        println!("{:?}", cart);
        cart.traverse_items(|item| {
            total_price += item.price;
        });
    });

    println!("Cart Total: ${:.2}", total_price);
}
