#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestation {
            true => None,
            false => match self.reservations {
                ..11 => Some(Food {
                    name: "Uni Sashimi".to_string(),
                }),
                _ => Some(Food {
                    name: "Strip Steak".to_string(),
                }),
            },
        }
    }
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        match self.has_mice_infestation {
            true => Err("Sorry, we have a mice problem".to_string()),
            false => {
                if address.is_empty() {
                    return Err("No delivery address specified".to_string());
                }
                Ok(Food {
                    name: String::from("Burger"),
                })
            }
        }
    }
}

fn main() {
    let first_resturaunt = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", first_resturaunt.chef_special());
    println!("{:?}", first_resturaunt.deliver_burger("123 Elm Street"));

    let second_resturaunt = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", second_resturaunt.chef_special());
    println!("{:?}", second_resturaunt.deliver_burger(""));
    println!("{:?}", second_resturaunt.deliver_burger("123 Some Street"));
}
