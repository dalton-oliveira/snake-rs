use crate::{
    game::Game,
    snake::{Snake, SnakeNode},
    types::FieldPoint,
};

pub trait GameRender {
    fn snake_full(&mut self, snake: &Snake);
    fn snake(&mut self, prev_tail: &Option<&SnakeNode>, game: &Game);
    fn food(&mut self, p: &FieldPoint);
}
