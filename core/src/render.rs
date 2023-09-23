use crate::{game::Game, snake::Snake, types::FieldPoint};

pub trait GameRender {
    fn snake_full(&mut self, snake: &Snake);
    fn snake(&mut self, game: &Game);
    fn food(&mut self, p: &FieldPoint);
}
