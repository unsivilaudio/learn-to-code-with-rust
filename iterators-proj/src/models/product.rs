#[derive(Debug, Hash)]
pub enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

impl Eq for Product {}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Product::Blender => match other {
                Product::Blender => true,
                _ => false,
            },
            Product::Microwave => match other {
                Product::Microwave => true,
                _ => false,
            },
            Product::Toaster => match other {
                Product::Toaster => true,
                _ => false,
            },
            Product::Fridge => match other {
                Product::Fridge => true,
                _ => false,
            },
        }
    }
}
