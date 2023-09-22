use std::collections::{linked_list::Iter, LinkedList};

use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, WrappableDirection},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::sprites::SpritesBinary;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct CanvasRender {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    game_width: u32,
    game_height: u32,
    drawer: Option<Drawer>,
    last_tail: Option<SnakeNode>,
    last_head: Option<SnakeNode>,
}

const PIXEL_W: u32 = 10;
const PIXEL_H: u32 = 10;
const PIXEL_X_OFFSET: u32 = 2;
const PIXEL_Y_OFFSET: u32 = 2;
const BLOCK_PIXEL_SIZE: u32 = 2;
const BLOCK_W: u32 = (PIXEL_W + GRID_SIZE) * BLOCK_PIXEL_SIZE;
const BLOCK_H: u32 = (PIXEL_H + GRID_SIZE) * BLOCK_PIXEL_SIZE; // coming soon...

const PIXEL_SIZE: u32 = 10;
const GRID_SIZE: u32 = 2;
const BLOCK_SIZE: u32 = (PIXEL_SIZE + GRID_SIZE) * BLOCK_PIXEL_SIZE;
const Y_OFFSET: u32 = PIXEL_SIZE;
const X_OFFSET: u32 = PIXEL_SIZE * 2;

#[derive(Clone)]
struct Drawer {
    tail: Option<SnakeNode>,
    head: Option<SnakeNode>,
    pos: FieldPoint,
    to: WrappableDirection,
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
        let pos = self.pos.add(self.to);
        log!(
            "walk_from: {},{} -> {},{} ({:?})",
            self.pos.x,
            self.pos.y,
            pos.x,
            pos.y,
            self.to.to
        );
        self.pos = pos;
    }

    fn draw_sprite(&mut self, sprite: u8, render: &mut CanvasRender) {
        let direction = self.to.to;
        log!(
            "draw_sprite: {},{} {} ({:?})",
            self.pos.x,
            self.pos.y,
            sprite,
            direction
        );
        render.draw_sprite(&self.pos, sprite, direction);
    }

    fn clear_last_tail(&mut self, render: &mut CanvasRender) {
        match self.tail {
            None => return,
            Some(tail) => {
                self.pos = tail.position;
                self.to.to = tail.direction;
                log!("clear_tail: {:?}, {:?}", self.pos, self.to.to);
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
        self.draw_sprite(SpritesBinary::tail_only(self.to.to), render);
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
        self.draw_sprite(SpritesBinary::full_block(node.direction), render);
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
        self.draw_sprite(SpritesBinary::head_only(head.direction), render);
        self.walk();
        self.draw_sprite(SpritesBinary::mouth(head.direction, false), render);
    }

    fn draw_neck_head(&mut self, neck: &SnakeNode, head: &SnakeNode, render: &mut CanvasRender) {
        let prev_head = self.head.unwrap();
        self.pos = prev_head.position;
        self.to.to = prev_head.direction;
        self.draw_sprite(SpritesBinary::full_block(neck.direction), render);
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
        self.draw_sprite(SpritesBinary::tail_only(self.to.to), render);
    }
}

impl CanvasRender {
    pub fn new(width: u32, height: u32) -> CanvasRender {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("snake-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        canvas.set_width(width * BLOCK_SIZE * 2 + PIXEL_SIZE * 2);
        canvas.set_height(height * BLOCK_SIZE * 2 + PIXEL_SIZE * 2);
        CanvasRender {
            canvas,
            context,
            drawer: None,
            last_tail: None,
            last_head: None,
            game_width: width,
            game_height: height,
        }
    }

    fn draw_sprite(&mut self, p: &FieldPoint, sprite: u8, direction: Direction) {
        match direction {
            Direction::Left | Direction::Right => {
                self.vertical_block(p.x as u32, p.y as u32, sprite)
            }
            Direction::Up | Direction::Down => {
                self.horizontal_block(p.x as u32, p.y as u32, sprite)
            }
        }
    }

    fn draw_sprites(&mut self, b_x: u32, b_y: u32, sprites: [u8; 2], direction: Direction) {
        match direction {
            Direction::Left => {
                self.vertical_block(b_x - 1, b_y, sprites[0]);
                self.vertical_block(b_x, b_y, sprites[1]);
            }
            Direction::Right => {
                self.vertical_block(b_x, b_y, sprites[0]);
                self.vertical_block(b_x + 1, b_y, sprites[1]);
            }
            Direction::Up => {
                self.horizontal_block(b_x, b_y - 1, sprites[0]);
                self.horizontal_block(b_x, b_y, sprites[1]);
            }
            Direction::Down => {
                self.horizontal_block(b_x, b_y, sprites[0]);
                self.horizontal_block(b_x, b_y + 1, sprites[1]);
            }
        }
    }

    fn block_to_dot(b_x: u32, b_y: u32) -> (u32, u32) {
        return (
            PIXEL_X_OFFSET + b_x * BLOCK_PIXEL_SIZE,
            PIXEL_Y_OFFSET + b_y * BLOCK_PIXEL_SIZE,
        );
    }

    pub fn vertical_block(&mut self, x0: u32, y0: u32, block: u8) {
        self.clear_vertical_block(x0, y0);
        let (x0, y0) = CanvasRender::block_to_dot(x0, y0);
        let block = block.reverse_bits();
        for i in 0..8 {
            let mask = 1 << i;
            if block & mask == 0 {
                continue;
            }
            let x = i % 2;
            let y = i / 2;
            self.dot(x0 + x, y0 + y - 1);
        }
    }

    pub fn horizontal_block(&mut self, x0: u32, y0: u32, block: u8) {
        self.clear_horizontal_block(x0, y0);
        let (x0, y0) = CanvasRender::block_to_dot(x0, y0);
        let block = block.reverse_bits();
        for i in 0..8 {
            let mask = 1 << i;
            if block & mask == 0 {
                continue;
            }
            let x = i % 4;
            let y = i / 4;
            self.dot(x0 + x - 1, y0 + y);
        }
    }

    pub fn clear_block(&mut self, p: &FieldPoint, direction: Direction) {
        match direction {
            Direction::Down | Direction::Up => self.clear_horizontal_block(p.x as u32, p.y as u32),
            Direction::Left | Direction::Right => self.clear_vertical_block(p.x as u32, p.y as u32),
        }
    }

    pub fn clear_horizontal_block(&mut self, b_x: u32, b_y: u32) {
        let (x0, y0) = CanvasRender::block_to_dot(b_x, b_y);
        let (w, h) = CanvasRender::pixels_to_canvas(4, 2);
        let (x0, y0) = CanvasRender::dot_to_pos(x0 - 1, y0);
        self.context.clear_rect(x0, y0, w, h);
    }

    pub fn clear_vertical_block(&mut self, b_x: u32, b_y: u32) {
        let (x0, y0) = CanvasRender::block_to_dot(b_x, b_y);
        let (w, h) = CanvasRender::pixels_to_canvas(2, 4);
        let (x0, y0) = CanvasRender::dot_to_pos(x0, y0 - 1);
        self.context.clear_rect(x0, y0, w, h);
    }

    fn pixels_to_canvas(w: u32, h: u32) -> (f64, f64) {
        return (
            (w * (PIXEL_W + GRID_SIZE)).into(),
            (h * (PIXEL_H + GRID_SIZE)).into(),
        );
    }

    fn dot_to_pos(x: u32, y: u32) -> (f64, f64) {
        return (
            ((x * (PIXEL_W + GRID_SIZE) + GRID_SIZE) as f64 - 1.0),
            ((y * (PIXEL_H + GRID_SIZE) + GRID_SIZE) as f64 - 1.0),
        );
    }

    pub fn dot(&mut self, x: u32, y: u32) {
        let ctx = &self.context;
        let (x0, y0) = CanvasRender::dot_to_pos(x, y);
        ctx.fill_rect(x0, y0, PIXEL_W.into(), PIXEL_H.into())
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
