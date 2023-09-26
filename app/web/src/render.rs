use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldElement, FieldPoint, WrappableDirection},
};

use crate::{sprites::SpritesBinary, utils::to_base_10_array};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[link(wasm_import_module = "/render/field.js")]
extern "C" {
    fn drawSprite4x2(sprite: u8, px: usize, py: usize);
    fn drawSprite2x4(sprite: u8, px: usize, py: usize);
    fn drawSprite3x3(sprite: u8, px: usize, py: usize);
    fn drawSprite8x4(sprite: u32, px: usize, py: usize);
}
#[link(wasm_import_module = "/render/panel.js")]
extern "C" {
    fn drawSprite3x5(sprite: u16, px: usize);
}
#[link(wasm_import_module = "/render/index.js")]
extern "C" {
    fn setup(width: u32, height: u32);
}

pub struct BinaryRender {
    tail: Option<SnakeNode>,
    head: Option<SnakeNode>,
    pos: FieldPoint,
    to: WrappableDirection,
}

impl BinaryRender {
    pub fn new(width: u32, height: u32) -> BinaryRender {
        let max = FieldPoint {
            x: width as usize * 2,
            y: height as usize * 2,
        };
        unsafe { setup(max.x as u32, max.y as u32) };
        let to = Direction::Right;
        let to = WrappableDirection { max, to };
        BinaryRender {
            pos: max,
            to,
            tail: None,
            head: None,
        }
    }
    fn draw_field_sprite(&mut self, sprite: u8) {
        let p = self.pos;
        let sprite = sprite.reverse_bits();
        unsafe {
            match self.to.to {
                Direction::Left | Direction::Right => drawSprite2x4(sprite, p.x, p.y),
                Direction::Up | Direction::Down => drawSprite4x2(sprite, p.x, p.y),
            }
        }
    }
    fn draw_field_sprites(&mut self, sprites: [u8; 2]) {
        self.draw_field_sprite(sprites[0]);
        self.walk();
        self.draw_field_sprite(sprites[1]);
    }
    fn draw_sprite_3x3(&mut self, sprite: u8, p: FieldPoint) {
        unsafe { drawSprite3x3(sprite.reverse_bits(), p.x * 2 + 1, p.y * 2 + 1) };
    }
    fn draw_sprite_8x4(&mut self, sprite: u32, p: FieldPoint) {
        unsafe { drawSprite8x4(sprite.reverse_bits(), p.x * 2 + 1, p.y * 2 + 1) };
    }
    fn go_to(&mut self, node: &SnakeNode) {
        self.pos = node.position.clone();
        self.turn(node.direction);
    }
    fn turn(&mut self, direction: Direction) {
        self.to.to = direction;
    }
    fn walk(&mut self) {
        self.pos = self.pos.add(self.to);
    }
    fn save_tail(&mut self) {
        self.tail = Some(SnakeNode {
            direction: self.to.to,
            position: self.pos,
        });
    }
    fn save_head(&mut self) {
        self.head = Some(SnakeNode {
            direction: self.to.to,
            position: self.pos,
        });
    }
    pub fn node(&mut self, node: &SnakeNode, next: &SnakeNode, has_food: bool) {
        self.turn(node.direction);
        let sprites = SpritesBinary::full_node(node.direction, next.direction, has_food);
        self.draw_field_sprites(sprites);
    }
    pub fn draw_head(&mut self, head: &SnakeNode, game: &Game) {
        let next_position = game.snake.next_head().position;
        let open = Snake::is_this_eat(game.field[next_position.x][next_position.y]);
        self.turn(head.direction);
        self.walk();
        self.save_head();
        self.draw_field_sprites(SpritesBinary::full_head(head.direction, open));
    }
    fn replace_head(&mut self, game: &Game, has_food: bool) {
        if let Some(prev) = self.head {
            self.go_to(&prev);
        }
        let mut iter = game.snake.nodes.iter();
        let (head, neck) = (iter.next_back().unwrap(), iter.next_back().unwrap());

        self.node(neck, head, has_food);
        self.draw_head(head, game);
    }
    fn clear_tail(&mut self) {
        if let Some(tail) = self.tail {
            self.go_to(&tail);
            self.draw_field_sprites([0, 0]);
        }
    }
    fn update_tail(&mut self, tail: &SnakeNode, next_to: Direction) {
        self.clear_tail();
        self.turn(tail.direction);
        match self.tail {
            Some(_) => self.walk(),
            None => {
                let p = tail.position;
                let (off_x, off_y) = match tail.direction {
                    Direction::Right => (0, 1),
                    Direction::Left => (2, 1),
                    Direction::Down => (1, 0),
                    Direction::Up => (1, 2),
                };
                self.go_to(&SnakeNode {
                    direction: tail.direction,
                    position: FieldPoint {
                        x: (p.x * 2) + off_x,
                        y: (p.y * 2) + off_y,
                    },
                })
            } // first iteraction
        }
        self.save_tail();
        if tail.direction == next_to {
            self.draw_field_sprites(SpritesBinary::full_tail(self.to.to));
        } else {
            // tricky and almost do the job
            self.draw_field_sprite(0);
            self.walk();
            self.turn(next_to);
            self.draw_field_sprite(0);
            self.draw_field_sprite(SpritesBinary::tail(self.to.to));
        }
    }
    fn update_score(&mut self, score: usize) {
        let score_digits = to_base_10_array(score, 4);
        for x in 0..score_digits.len() {
            let digit = score_digits[x];
            unsafe { drawSprite3x5(SpritesBinary::digit(digit).reverse_bits(), x) }
        }
    }
}

impl GameRender for BinaryRender {
    fn snake_full(&mut self, game: &Game) {
        let mut iter = game.snake.nodes.iter();

        if let Some(tail) = iter.next() {
            self.update_tail(tail, tail.direction);
        } else {
            return;
        }
        let node = iter.next();
        if node.is_none() {
            return;
        }
        let mut node = node.unwrap();
        while let Some(next) = iter.next() {
            self.walk();
            self.node(node, next, false);
            node = &next;
        }

        self.draw_head(node, game);
        self.update_score(0);
    }

    fn snake(&mut self, game: &Game) {
        let mut iter = game.snake.nodes.iter();
        let (tail, next) = (iter.next(), iter.next());
        if tail.is_none() || next.is_none() {
            return;
        }
        self.update_tail(tail.unwrap(), next.unwrap().direction);
        self.replace_head(game, false);
    }

    fn eat(&mut self, game: &Game, p: FieldPoint) {
        let element = game.field[p.x][p.y];
        match element {
            FieldElement::Treat => self.draw_sprite_3x3(0, p),
            _ => self.draw_sprite_8x4(0, p),
        }
        self.replace_head(game, true);
        self.update_score(game.score);
    }

    fn food(&mut self, game: &Game, p: FieldPoint) {
        // self.replace_head(game);
        let element = game.field[p.x][p.y];
        match element {
            FieldElement::Treat => self.draw_sprite_3x3(SpritesBinary::food(), p),
            x => self.draw_sprite_8x4(SpritesBinary::special_food(x), p),
        };
    }
}
