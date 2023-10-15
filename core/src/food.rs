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
    pub minimum: u16,
    pub count: u16,
    bag: [FoodType; BAG.len()],
}

impl FoodField {
    pub fn new() -> FoodField {
        FoodField {
            foods: Vec::new(),
            count: 0,
            minimum: 0,
            bag: BAG.clone(),
        }
    }
    pub fn total_filled(&self) -> u16 {
        let mut total: u16 = 0;
        for f in self.foods.iter() {
            total += f.size as u16;
        }
        return total;
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

    pub fn add_food(&mut self, field: &Field) {
        let total_filled = self.total_filled();
        if total_filled >= self.minimum {
            return;
        }
        let max = (field.bit_set.len() - field.bit_set.count_ones(..)) as u16 - total_filled;
        if max < 1 {
            return;
        }

        // @todo clear-me, way too complex
        let mut nth = rand::thread_rng().gen_range(0..max);
        let mut idx: u16 = 0;

        // finds the nth free position
        let max_len = field.bit_set.len() as u16;
        loop {
            if nth == 0 || idx == max_len {
                break;
            }
            if !field.idx_filled(idx) {
                nth -= 1;
            }
            idx += 1;
        }

        // won't overlap with current food
        while idx < max_len && self.has_at(&field.from_idx(idx)).is_some() || field.idx_filled(idx)
        {
            idx = (idx + 1) % field.bit_set.len() as u16;
        }

        if idx == max_len {
            return;
        }

        let food = Food::new(FoodType::Basic, field.from_idx(idx));
        self.set_food(food);
        self.count += 1;
        if self.count % 5 == 0 {
            let food = self.random_special(max - 1, field);
            if let Some(food) = food {
                self.set_food(food);
            }
        }
    }

    /// Special foods requires 2 slots on the field. It must be not placed on the last col,
    ///  as it can't wrap to the next row.
    pub fn random_special(&mut self, max: u16, field: &Field) -> Option<Food> {
        let required = (self.foods.len() * 2) as u16;
        // check enought space
        let max = max / 2;
        if max < 1 || required >= max - 1 {
            return None;
        }
        let max = max - required - 1;
        let mut rng = rand::thread_rng();
        let mut nth = rng.gen_range(0..max);

        // make sure each idx doesn't land on the last col
        let mut idx: u16 = field.width % 2;
        while nth > 0 && (idx as usize + 1) < field.bit_set.len() {
            let p1 = field.from_idx(idx);
            let p2 = FieldPoint {
                x: p1.x + 1,
                y: p1.y,
            };
            if !field.filled(&p1) && !field.filled(&p2) {
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
