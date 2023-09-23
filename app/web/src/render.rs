use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, WrappableDirection},
};

use crate::sprites::SpritesBinary;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[link(wasm_import_module = "/render.js")]
extern "C" {
    fn drawSprite(sprite: u8, direction: Direction, px: usize, py: usize);
    // fn drawPanelSprite(px: usize, xOff: u8, sprite: u16);
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
        unsafe { setup(width, height) };
        let max = FieldPoint {
            x: width as usize * 2,
            y: height as usize * 2,
        };
        let to = Direction::Right;
        let to = WrappableDirection { max, to };
        BinaryRender {
            pos: max,
            to,
            tail: None,
            head: None,
        }
    }
    fn draw_sprite(&mut self, sprite: u8) {
        let p = self.pos;
        unsafe { drawSprite(sprite.reverse_bits(), self.to.to, p.x, p.y) };
    }
    fn draw_sprites(&mut self, sprites: [u8; 2]) {
        self.draw_sprite(sprites[0]);
        self.walk();
        self.draw_sprite(sprites[1]);
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
    // fn backward(&mut self) {
    //     let current = self.to.to;
    //     self.turn(Snake::opposite_of(current));
    //     self.walk();
    //     self.to.to = current;
    // }
    pub fn node(&mut self, node: &SnakeNode, next: &SnakeNode) {
        self.turn(node.direction);
        let sprites = SpritesBinary::full_node(node.direction, next.direction);
        self.draw_sprites(sprites);
    }
    pub fn draw_head(&mut self, head: &SnakeNode) {
        self.turn(head.direction);
        self.walk();
        self.save_head();
        self.draw_sprites(SpritesBinary::full_head(head.direction, false));
    }
    fn replace_head(&mut self, neck: &SnakeNode, head: &SnakeNode) {
        let prev_head = self.head.unwrap();
        self.go_to(&prev_head);
        self.node(neck, head);
        self.draw_head(head);
    }
    fn clear_tail(&mut self) {
        if let Some(tail) = self.tail {
            self.go_to(&tail);
            self.draw_sprites([0, 0]);
        }
    }
    fn update_tail(&mut self, tail: &SnakeNode, next_to: Direction) {
        self.clear_tail();
        self.turn(tail.direction);
        match self.tail {
            Some(_) => self.walk(),
            None => self.go_to(tail), // first iteraction
        }
        self.save_tail();
        if tail.direction == next_to {
            self.draw_sprites(SpritesBinary::full_tail(self.to.to));
        } else {
            // tricky and almost do the job
            self.draw_sprite(0);
            self.walk();
            self.turn(next_to);
            self.draw_sprite(0);
            self.draw_sprite(SpritesBinary::tail(self.to.to));
        }
    }
}

// @todo prior to wrap, move only one block
impl GameRender for BinaryRender {
    fn snake_full(&mut self, snake: &Snake) {
        let mut iter = snake.nodes.iter();

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
            self.node(node, next);
            node = &next;
        }
        self.draw_head(node);
    }

    fn snake(&mut self, game: &Game) {
        let mut iter = game.snake.nodes.iter();
        let (tail, next) = (iter.next(), iter.next());
        if tail.is_none() || next.is_none() {
            return;
        }
        self.update_tail(tail.unwrap(), next.unwrap().direction);

        let mut iter = game.snake.nodes.iter();
        let (head, neck) = (iter.next_back().unwrap(), iter.next_back().unwrap());
        self.replace_head(neck, head);
    }

    fn food(&mut self, _p: &FieldPoint) {}
}
