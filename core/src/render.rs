use crate::{food::FoodField, snake::Snake, types::Food};

pub trait GameRender {
    fn snake_full(&mut self, snake: &Snake, food_field: &FoodField);
    fn crawl(&mut self, snake: &Snake, food_field: &FoodField);
    fn grow(&mut self, snake: &Snake, food_field: &FoodField);
    fn added_food(&mut self, food: &Food);
    fn removed_food(&mut self, food: &Food);
    fn update_score(&mut self, score: u16);
}
