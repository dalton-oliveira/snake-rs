use std::collections::linked_list::Iter;

use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, WrappableDirection},
};

use crate::sprites::SpritesBinary;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[link(wasm_import_module = "/render.js")]
extern "C" {
    fn clearBlock(px: usize, py: usize, direction: Direction);
    fn drawSprite(sprite: u8, direction: Direction, px: usize, py: usize);
    fn setup(width: u32, height: u32);
}

pub struct CanvasRender {
    game_width: u32,
    game_height: u32,
    drawer: Option<Drawer>,
}

#[derive(Clone)]
struct Drawer {
    tail: Option<SnakeNode>,
    head: Option<SnakeNode>,
    pos: FieldPoint,
    to: WrappableDirection,
}

impl CanvasRender {
    pub fn new(width: u32, height: u32) -> CanvasRender {
        unsafe { setup(width, height) };
        CanvasRender {
            drawer: None,
            game_width: width,
            game_height: height,
        }
    }
    fn draw_sprite(&mut self, p: FieldPoint, sprite: u8, direction: Direction) {
        unsafe { drawSprite(sprite.reverse_bits(), direction, p.x, p.y) };
    }

    pub fn clear_block(&mut self, p: &FieldPoint, direction: Direction) {
        unsafe { clearBlock(p.x, p.y, direction) };
    }
}

// @todo prior to wrap, move only one block
impl GameRender for CanvasRender {
    fn snake_full(&mut self, snake: &Snake) {
        let nodes = &snake.nodes;
        let tail = nodes.front().unwrap();
        let mut drawer = Drawer::new(
            self.game_width,
            self.game_height,
            &tail.position,
            tail.direction,
        );
        drawer.draw_snake(&mut nodes.iter(), self);
        self.drawer = Some(drawer);
    }

    fn snake(&mut self, prev_tail: &Option<&SnakeNode>, game: &Game) {
        let mut drawer = self.drawer.clone().unwrap();

        let mut iter = game.snake.nodes.iter();
        let tail = iter.next().unwrap();
        let next = iter.next().unwrap();
        drawer.new_tail(tail, next.direction, self);
        let mut iter = game.snake.nodes.iter();
        let head = iter.next_back().unwrap();
        let neck = iter.next_back().unwrap();
        drawer.draw_neck_head(neck, head, self);
        self.drawer = Some(drawer);
    }

    fn food(&mut self, _p: &FieldPoint) {}
}

impl Drawer {
    fn draw_snake(&mut self, iter: &mut Iter<SnakeNode>, render: &mut CanvasRender) {
        if let Some(tail) = self.tail.clone() {
            self.clear_last_tail(render);
            self.pos = tail.position;
            self.to.to = tail.direction;
            self.walk();
        }
        self.draw_tail(iter, render);
    }
    fn new(width: u32, height: u32, pos: &FieldPoint, to: Direction) -> Drawer {
        let to = WrappableDirection {
            to,
            max: FieldPoint {
                x: width as usize * 2,
                y: height as usize * 2,
            },
        };
        Drawer {
            tail: None,
            head: None,
            pos: pos.clone(),
            to,
        }
    }
    fn walk(&mut self) {
        self.pos = self.pos.add(self.to);
    }
    fn draw_sprite(&mut self, sprite: u8, render: &mut CanvasRender) {
        let direction = self.to.to;
        render.draw_sprite(self.pos, sprite, direction);
    }
    fn clear_last_tail(&mut self, render: &mut CanvasRender) {
        match self.tail {
            None => return,
            Some(tail) => {
                self.pos = tail.position;
                self.to.to = tail.direction;
                render.clear_block(&self.pos, self.to.to);
                self.walk();
                render.clear_block(&self.pos, self.to.to);
            }
        }
    }
    fn draw_tail(&mut self, iter: &mut Iter<SnakeNode>, render: &mut CanvasRender) {
        let tail = iter.next().unwrap();
        self.tail = Some(tail.clone());
        self.to.to = tail.direction;
        self.draw_sprite(SpritesBinary::dot(self.to.to), render);
        self.walk();
        self.draw_sprite(SpritesBinary::tail(self.to.to), render);
        if let Some(node) = iter.next() {
            self.draw_nodes(node, iter, render);
        }
    }

    fn draw_nodes(
        &mut self,
        node: &SnakeNode,
        iter: &mut Iter<SnakeNode>,
        render: &mut CanvasRender,
    ) {
        let next = iter.next();
        if next.is_none() {
            self.draw_head(node, render);
            return;
        }
        let next = next.unwrap();
        self.to.to = node.direction;
        self.walk();
        self.draw_sprite(SpritesBinary::block(node.direction), render);
        self.walk();
        self.draw_sprite(
            SpritesBinary::corner(node.direction, next.direction),
            render,
        );
        self.draw_nodes(next, iter, render);
    }

    fn draw_head(&mut self, head: &SnakeNode, render: &mut CanvasRender) {
        self.to.to = head.direction;
        self.walk();
        self.head = Some(SnakeNode {
            direction: head.direction,
            position: self.pos,
        });
        self.draw_sprite(SpritesBinary::eyes(head.direction), render);
        self.walk();
        self.draw_sprite(SpritesBinary::mouth(head.direction, false), render);
    }

    fn draw_neck_head(&mut self, neck: &SnakeNode, head: &SnakeNode, render: &mut CanvasRender) {
        let prev_head = self.head.unwrap();
        self.pos = prev_head.position;
        self.to.to = prev_head.direction;
        self.draw_sprite(SpritesBinary::block(neck.direction), render);
        self.walk();
        self.draw_sprite(
            SpritesBinary::corner(neck.direction, head.direction),
            render,
        );
        self.draw_head(head, render);
    }

    fn new_tail(&mut self, tail: &SnakeNode, next_to: Direction, render: &mut CanvasRender) {
        self.clear_last_tail(render);
        self.to.to = tail.direction;
        self.walk();
        self.tail = Some(SnakeNode {
            direction: tail.direction,
            position: self.pos,
        });
        self.draw_sprite(SpritesBinary::dot(self.to.to), render);
        self.walk();
        self.draw_sprite(SpritesBinary::tail(self.to.to), render);
    }
}
