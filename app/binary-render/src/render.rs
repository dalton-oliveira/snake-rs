use snake::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{opposite_of, Direction, FieldPoint, Food, FoodType, WrappableDirection},
};

use crate::{sprites::SpritesBinary, types::Screen, utils::to_base_10_array};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct BinaryRender {
    tail: Option<SnakeNode>,
    head: Option<SnakeNode>,
    pos: FieldPoint,
    to: WrappableDirection,
    screen: Box<dyn Screen>,
}

impl GameRender for BinaryRender {
    fn snake_full(&mut self, snake: &Snake, food_field: &FoodField) {
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
            self.node(node, next, false);
            node = &next;
        }

        self.draw_head(snake, food_field);
        self.update_score(0);
    }

    fn crawl(&mut self, snake: &Snake, food_field: &FoodField) {
        let mut iter = snake.nodes.iter();
        let (tail, next) = (iter.next(), iter.next());
        if tail.is_none() || next.is_none() {
            return;
        }
        self.update_tail(tail.unwrap(), next.unwrap().direction);
        self.replace_head(snake, food_field, false);
        if food_field.last_special_tick > 0 {
            self.draw_panel_digits(food_field.last_special_tick, 2, -2);
        }
    }

    fn grow(&mut self, snake: &Snake, food_field: &FoodField) {
        self.replace_head(snake, food_field, true);
    }

    fn added_food(&mut self, food: &Food) {
        let p = food.location;
        match food.shape {
            FoodType::Basic => self.screen.field_sprite_3x3(SpritesBinary::food(), &p),
            x => {
                let sprite = SpritesBinary::special_food(x);
                self.screen.field_sprite_8x4(sprite, &p);
                self.screen.panel_sprite_8x4(sprite, -14, 2);
            }
        };
    }

    fn removed_food(&mut self, food: &Food) {
        match food.shape {
            FoodType::Basic => self.screen.field_sprite_3x3(0, &food.location),
            _ => {
                self.screen.field_sprite_8x4(0, &food.location);
                self.screen.panel_sprite_8x4(0, -14, 2);
                self.clear_panel_digits(2, -2)
            }
        }
    }

    fn update_score(&mut self, score: u16) {
        self.draw_panel_digits(score, 4, 0);
    }
}

impl BinaryRender {
    // @todo understand the  + 'static
    pub fn new(width: u16, height: u16, screen: impl Screen + 'static) -> BinaryRender {
        let max = FieldPoint {
            x: width * 2,
            y: height * 2,
        };
        screen.setup(max.x, max.y);
        let to = Direction::Right;
        let to = WrappableDirection { max, to };
        BinaryRender {
            screen: Box::new(screen),
            pos: max,
            to,
            tail: None,
            head: None,
        }
    }
    fn draw_snake_sprite(&mut self, sprite: u8) {
        let p = self.pos;
        match self.to.to {
            Direction::Left | Direction::Right => self.screen.field_sprite_2x4(sprite, &p),
            Direction::Up | Direction::Down => self.screen.field_sprite_4x2(sprite, &p),
        }
    }
    fn draw_snake_sprites(&mut self, sprites: [u8; 2]) {
        self.draw_snake_sprite(sprites[0]);
        self.walk();
        self.draw_snake_sprite(sprites[1]);
    }
    fn draw_panel_digits(&mut self, n: u16, digits: u8, x0: i16) {
        let score_digits = to_base_10_array(n, digits);
        for x in 0..score_digits.len() {
            let digit = score_digits[x];
            let sprite = SpritesBinary::digit(digit);
            self.screen.panel_sprite_3x5(sprite, x0 + x as i16);
        }
    }
    fn clear_panel_digits(&mut self, digits: u8, x0: i16) {
        for x in 0..digits {
            self.screen.panel_sprite_3x5(0, x0 + x as i16);
        }
    }
    fn go_to(&mut self, node: &SnakeNode) {
        self.pos = node.position.clone();
        self.turn(node.direction);
    }
    fn turn(&mut self, direction: Direction) {
        self.to.to = direction;
    }
    fn walk(&mut self) {
        self.pos = self.pos.add_wrapping(self.to);
    }
    fn step_back(&mut self) {
        let opposite = WrappableDirection {
            max: self.to.max,
            to: opposite_of(self.to.to),
        };
        self.pos = self.pos.add_wrapping(opposite);
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
        self.draw_snake_sprites(sprites);
    }
    pub fn draw_head(&mut self, snake: &Snake, food_field: &FoodField) {
        let head = snake.nodes.back().unwrap();
        let next_position = snake.next_head().position;
        let open = food_field.has_at(&next_position).is_some();
        self.turn(head.direction);
        self.walk();
        self.save_head();
        self.draw_snake_sprites(SpritesBinary::full_head(head.direction, open));
    }
    fn replace_head(&mut self, snake: &Snake, food_field: &FoodField, has_food: bool) {
        if let Some(prev) = self.head {
            self.go_to(&prev);
        }
        let mut iter = snake.nodes.iter();
        let (head, neck) = (iter.next_back().unwrap(), iter.next_back().unwrap());

        self.node(neck, head, has_food);
        self.draw_head(snake, food_field);
    }
    fn clear_tail(&mut self) {
        if let Some(tail) = self.tail {
            self.go_to(&tail);
            self.draw_snake_sprites([0, 0]);
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
            self.draw_snake_sprites(SpritesBinary::full_tail(self.to.to));
        } else {
            // tricky but does the job
            self.draw_snake_sprite(0);
            self.walk();
            self.turn(next_to);
            self.step_back();
            self.draw_snake_sprites(SpritesBinary::full_tail(next_to));
        }
    }
}
