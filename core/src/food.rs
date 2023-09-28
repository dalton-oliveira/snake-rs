use rand::{seq::SliceRandom, Rng};

use crate::types::{Field, FieldPoint, Food, FoodType};

const BAG: [FoodType; 6] = [
    FoodType::Whale,
    FoodType::Turtle,
    FoodType::Chameleon,
    FoodType::Elephant,
    FoodType::Alien,
    FoodType::Caterpillar,
];

pub struct FoodField {
    pub foods: Vec<Food>,
    pub count: u16,
    bag: [FoodType; BAG.len()],
}

impl FoodField {
    pub fn new() -> FoodField {
        FoodField {
            foods: Vec::new(),
            count: 0,
            bag: BAG.clone(),
        }
    }
    pub fn has_at(&self, p: &FieldPoint) -> bool {
        for i in 0..self.foods.len() {
            if self.foods[i].is_at(p) {
                return true;
            }
        }
        return false;
    }
    pub fn grab(&mut self, p: &FieldPoint) -> Option<Food> {
        for i in 0..self.foods.len() {
            let food = self.foods[i];
            if food.is_at(p) {
                self.foods.remove(i);
                return Some(food);
            }
        }
        return None;
    }

    pub fn add_food(&mut self, max: u16, field: &Field) -> Food {
        self.count += 1;
        let mut nth = rand::thread_rng().gen_range(0..max - self.foods.len() as u16 - 1);
        let mut idx: u16 = 0;

        // finds the nth free position
        loop {
            if !field.idx_filled(idx) {
                nth -= 1;
            }
            if nth == 0 {
                break;
            }
            idx += 1;
        }

        // won't overlap with current food
        while self.has_at(&field.from_idx(idx)) || field.idx_filled(idx) {
            idx = (idx + 1) % field.bit_set.len() as u16;
        }

        let food = Food::new(FoodType::Basic, field.from_idx(idx));
        self.foods.push(food);

        return food;
    }

    /// Special foods requires 2 slots on the field. It must be not placed on the last col,
    ///  as it can't wrap to the next row.
    pub fn add_special_food(&mut self, max: u16, field: &Field) -> Option<Food> {
        self.count += 1;

        let mut rng = rand::thread_rng();
        let max = max / 2 - (self.foods.len() * 2) as u16 - 1;
        let mut nth = rng.gen_range(0..max);

        // make sure each idx doesn't land on the last col
        let mut idx: u16 = field.width % 2;
        loop {
            if !field.idx_filled(idx) && !field.idx_filled(idx + 1) {
                nth -= 1;
            }
            if nth == 0 {
                break;
            }
            idx += 2;
        }

        // possibly well filled field. Skip then
        if idx as usize >= field.bit_set.len() {
            return None;
        }

        self.bag.shuffle(&mut rng);
        let food = Food::new(self.bag[0], field.from_idx(idx));
        self.foods.push(food);

        return Some(food);
    }
}
