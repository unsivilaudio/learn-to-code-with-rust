#![allow(dead_code)]

use super::ingredients::{Caloric, Dressing, Protein, Vegetable};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    pub fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.vegetables.len() > 0
    }

    pub fn has_duplicate_vegetables(&self) -> bool {
        HashSet::<&Vegetable>::from_iter(&self.vegetables).len() != self.vegetables.len()
    }
}

impl Caloric for Salad {
    fn calories(&self) -> u32 {
        self.dressing.calories()
            + self.protein.calories()
            + self
                .vegetables
                .iter()
                .fold(0, |acc, vegetable| acc + &vegetable.calories())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_salad() {
        let salad = Salad::new(
            Protein::CrispyChicken,
            vec![Vegetable::Cucumber],
            Dressing::Italian,
        );

        assert!(salad.is_valid());
    }

    #[test]
    fn salad_has_valid_number_of_vegetables() {
        let vegetables = vec![Vegetable::Cucumber];
        let salad = Salad::new(Protein::Steak, vegetables, Dressing::Italian);

        assert_eq!(salad.is_valid(), true);
    }

    #[test]
    fn salad_has_duplicate_vegetable_entries() {
        let vegetables = vec![Vegetable::Cucumber, Vegetable::Cucumber];
        let salad = Salad::new(Protein::Steak, vegetables, Dressing::Italian);

        assert_eq!(salad.has_duplicate_vegetables(), true);
    }

    #[test]
    fn salad_does_not_have_duplicate_vegetable_entries() {
        let vegetables = vec![Vegetable::Cucumber, Vegetable::Tomato];
        let salad = Salad::new(Protein::Steak, vegetables, Dressing::Italian);

        assert_eq!(salad.has_duplicate_vegetables(), false);
    }

    #[test]
    fn gets_total_number_of_calories() {
        let vegetables = vec![Vegetable::Cucumber, Vegetable::Tomato];
        let salad = Salad::new(Protein::Steak, vegetables, Dressing::Italian);

        assert_eq!(salad.calories(), 465);
    }
}
