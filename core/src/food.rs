use rand::{seq::SliceRandom, Rng};

use crate::{
    render::GameRender,
    types::{Field, FieldPoint, Food, FoodType},
};

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
    ticks_for_special: u16,
    pub last_special_tick: u16,
    bag: [FoodType; BAG.len()],
}

impl FoodField {
    pub fn new() -> FoodField {
        FoodField {
            foods: Vec::new(),
            ticks_for_special: 30, // @todo move to GameConfig
            last_special_tick: 0,
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

    pub fn grab(&mut self, p: &FieldPoint, game_render: &mut impl GameRender) -> Option<Food> {
        let i = self.has_at(p);
        if let Some(i) = i {
            let food = self.foods.remove(i);
            game_render.removed_food(&food);
            if food.shape != FoodType::Basic {
                self.last_special_tick = 0;
            }
            return Some(food);
        }
        return None;
    }

    pub fn add_food(&mut self, max: u16, field: &Field, game_render: &mut impl GameRender) -> Food {
        self.count += 1;
        let mut nth = rand::thread_rng().gen_range(0..max - 1);
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
        while self.has_at(&field.from_idx(idx)).is_some() || field.idx_filled(idx) {
            idx = (idx + 1) % field.bit_set.len() as u16;
        }

        let food = Food::new(FoodType::Basic, field.from_idx(idx));
        self.foods.push(food);
        game_render.added_food(&food);

        if self.count % 6 == 0 {
            let food = self.add_special_food(max - 1, field);
            if let Some(food) = food {
                game_render.added_food(&food);
            }
        }

        return food;
    }

    /// Special foods requires 2 slots on the field. It must be not placed on the last col,
    ///  as it can't wrap to the next row.
    pub fn add_special_food(&mut self, max: u16, field: &Field) -> Option<Food> {
        if self.last_special_tick > 0 {
            return None;
        }

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
        self.count += 1;
        self.last_special_tick = self.ticks_for_special;
        return Some(food);
    }

    pub fn tick(&mut self, game_render: &mut impl GameRender) {
        if self.last_special_tick == 1 {
            let mut idx = 0;
            for i in 0..self.foods.len() {
                if self.foods[i].shape != FoodType::Basic {
                    idx = i;
                    break;
                }
            }
            let food = self.foods.remove(idx);
            game_render.removed_food(&food);
        }
        if self.last_special_tick > 0 {
            self.last_special_tick -= 1;
        }
    }
}
