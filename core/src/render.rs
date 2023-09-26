use crate::{game::Game, types::FieldPoint};

pub trait GameRender {
    fn snake_full(&mut self, game: &Game);
    fn snake(&mut self, game: &Game);
    fn eat(&mut self, game: &Game, p: FieldPoint);
    fn food(&mut self, game: &Game, p: FieldPoint);
}
