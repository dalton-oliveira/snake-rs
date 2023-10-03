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

#[derive(bincode::Encode, bincode::Decode, PartialEq, Debug, Clone)]
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

    pub fn has_at(&self, p: &FieldPoint) -> Option<usize> {
        for i in 0..self.foods.len() {
            if self.foods[i].is_at(p) {
                return Some(i);
            }
        }
        return None;
    }

    pub fn grab(&mut self, p: &FieldPoint) -> Option<Food> {
        let i = self.has_at(p);
        if let Some(i) = i {
            let food = self.foods.remove(i);
            return Some(food);
        }
        return None;
    }

    pub fn set_food(&mut self, food: Food) {
        self.foods.push(food);
        self.count += 1;
    }

    pub fn add_food(&mut self, max: u16, field: &Field) {
        self.count += 1;
        let mut nth = rand::thread_rng().gen_range(0..max - 1);
        let mut idx: u16 = 0;

        // finds the nth free position
        loop {
            if nth == 0 {
                break;
            }
            if !field.idx_filled(idx) {
                nth -= 1;
            }
            idx += 1;
        }

        // won't overlap with current food
        while self.has_at(&field.from_idx(idx)).is_some() || field.idx_filled(idx) {
            idx = (idx + 1) % field.bit_set.len() as u16;
        }

        let food = Food::new(FoodType::Basic, field.from_idx(idx));
        self.set_food(food);

        if self.count % 6 == 0 {
            let food = self.random_special(max - 1, field);
            if let Some(food) = food {
                self.set_food(food);
            }
        }
    }

    /// Special foods requires 2 slots on the field. It must be not placed on the last col,
    ///  as it can't wrap to the next row.
    pub fn random_special(&mut self, max: u16, field: &Field) -> Option<Food> {
        let mut rng = rand::thread_rng();
        let max = max / 2 - (self.foods.len() * 2) as u16;
        let mut nth = rng.gen_range(0..max - 1);

        // make sure each idx doesn't land on the last col
        let mut idx: u16 = field.width % 2;
        loop {
            if nth == 0 {
                break;
            }
            if !field.idx_filled(idx) && !field.idx_filled(idx + 1) {
                nth -= 1;
            }
            idx += 2;
        }

        // possibly well filled field. Skip then
        if idx as usize >= field.bit_set.len() {
            return None;
        }

        self.bag.shuffle(&mut rng);
        let food = Food::new(self.bag[0], field.from_idx(idx));
        return Some(food);
    }

    pub fn tick(&mut self) {
        self.foods.retain_mut(|food| {
            if food.ticks_left == 1 {
                return false;
            }
            if food.ticks_left > 1 {
                food.ticks_left -= 1;
            }
            return true;
        });
    }
}
