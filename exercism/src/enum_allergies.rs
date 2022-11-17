pub struct Allergies {
    score: u32 
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}


impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens:Vec<Allergen> = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ];

        let mut res: Vec<Allergen> = vec![];
        for aller in allergens {
            if self.is_allergic_to(&aller) == true {
                res.push(aller);
            }
        }

        res
    }
}