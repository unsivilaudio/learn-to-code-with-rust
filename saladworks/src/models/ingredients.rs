#![allow(dead_code)]

pub trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_calories_of_vegetable() {
        let tomato = Vegetable::Tomato;
        let cucumber = Vegetable::Cucumber;
        let sweet_potato = Vegetable::SweetPotato;

        assert_eq!(tomato.calories(), 20);
        assert_eq!(cucumber.calories(), 15);
        assert_eq!(sweet_potato.calories(), 100);
    }

    #[test]
    fn gets_calories_of_protein() {
        let crispy_chicken = Protein::CrispyChicken;
        let fried_chicken = Protein::FriedChicken;
        let steak = Protein::Steak;
        let tofu = Protein::Tofu;

        assert_eq!(crispy_chicken.calories(), 400);
        assert_eq!(fried_chicken.calories(), 500);
        assert_eq!(steak.calories(), 300);
        assert_eq!(tofu.calories(), 200);
    }

    #[test]
    fn gets_calories_of_dressing() {
        let ranch = Dressing::Ranch;
        let vinaigrette = Dressing::Vinaigrette;
        let italian = Dressing::Italian;

        assert_eq!(ranch.calories(), 150);
        assert_eq!(vinaigrette.calories(), 120);
        assert_eq!(italian.calories(), 130);
    }
}
