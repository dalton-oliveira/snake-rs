use crate::{food::FoodField, snake::Snake, types::Food};

pub trait GameRender: Send + Sync {
    fn snake(&mut self, snake: &Snake, food_field: &FoodField);
    fn food(&mut self, food: &Food);
    fn score(&mut self, score: u16);
}
