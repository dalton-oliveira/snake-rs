use crate::{game::Game, types::Food};

pub trait GameRender {
    fn snake_full(&mut self, game: &Game);
    fn snake(&mut self, game: &Game);
    fn eat(&mut self, game: &Game, food: &Food);
    fn added_food(&mut self, food: &Food);
}
