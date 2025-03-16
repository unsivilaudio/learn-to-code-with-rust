#[derive(Debug)]
enum Tier {
    Ads,
    AdFree,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => println!(
                "You have limited access to the site's premium features for {price}, for {months} months",
            ),
            Subscription::Premium { tier } => println!("Your premium tier is {tier:?}"),
        }
    }
}

fn main() {
    Subscription::Free.summarize();
    Subscription::Basic(5.99, 24).summarize();
    let premium_tier = Subscription::Premium { tier: Tier::Ads };
    premium_tier.summarize();
}
