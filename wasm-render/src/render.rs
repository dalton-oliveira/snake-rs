use snake::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, Food, FoodType, WrappableDirection},
};

use crate::{sprites::Sprites, types::Screen, utils::to_base_10_array};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct BinaryRender {
    pos: FieldPoint,
    to: WrappableDirection,
    screen: Box<dyn Screen>,
}

impl GameRender for BinaryRender {
    fn snake(&mut self, snake: &Snake, food_field: &FoodField) {
        let mut iter = snake.nodes.iter();

        let mut node = match iter.next_back() {
            None => return,
            Some(head) => {
                self.head_at(head);
                let open = food_field.has_at(&snake.next_head().position).is_some();
                self.draw_snake_sprites(head.direction, Sprites::full_head(head.direction, open));
                head
            }
        };

        let tail = snake.nodes.front().unwrap();
        while let Some(prev) = iter.next_back() {
            if prev.position == tail.position {
                break;
            }
            let sprites = Sprites::full_node(prev.direction, node.direction, prev.stuffed);
            self.draw_snake_sprites(prev.direction, sprites);
            node = &prev;
        }
        self.draw_snake_sprites(node.direction, Sprites::full_tail(node.direction));
    }

    fn food(&mut self, food: &Food) {
        let p = FieldPoint {
            x: food.location.x * 2 + 1,
            y: food.location.y * 2 + 1,
        };
        match food.shape {
            FoodType::Basic => self.screen.field_sprite_3x3(Sprites::food(), &p),
            x => {
                let sprite = Sprites::special_food(x);
                self.screen.field_sprite_8x4(sprite, &p);
            }
        };
    }

    fn score(&mut self, score: u16) {
        self.draw_panel_digits(score, 4, 0);
    }
}

impl BinaryRender {
    pub fn new(width: u16, height: u16, screen: Box<dyn Screen>) -> BinaryRender {
        let max = FieldPoint {
            x: width * 2,
            y: height * 2,
        };
        screen.setup(max.x, max.y);
        let to = Direction::Right;
        let to = WrappableDirection { max, to };
        BinaryRender {
            screen,
            pos: max,
            to,
        }
    }
    fn head_at(&mut self, head: &SnakeNode) {
        let p = head.position;
        self.go_to(&SnakeNode {
            direction: head.direction,
            stuffed: false,
            position: FieldPoint {
                x: (p.x * 2) + 1,
                y: (p.y * 2) + 1,
            },
        });
    }
    pub fn draw_food_ticker(&mut self, food: &Food, idx: u8) {
        let sprite = Sprites::special_food(food.shape);
        self.screen
            .panel_sprite_8x4(sprite, -(14 + 20 * idx as i16), 2);
        self.draw_panel_digits(food.ticks_left as u16, 2, -(2 + idx as i16 * 5));
    }
    fn draw_snake_sprite(&mut self, sprite: u8) {
        let p = self.pos;
        match self.to.to {
            Direction::Left | Direction::Right => self.screen.field_sprite_2x4(sprite, &p),
            Direction::Up | Direction::Down => self.screen.field_sprite_4x2(sprite, &p),
        }
    }
    fn draw_snake_sprites(&mut self, direction: Direction, sprites: [u8; 2]) {
        self.draw_snake_sprite(sprites[0]);
        self.turn(direction);
        self.step();
        self.draw_snake_sprite(sprites[1]);
        self.step();
    }
    fn draw_panel_digits(&mut self, n: u16, digits: u8, x0: i16) {
        let score_digits = to_base_10_array(n, digits);
        for (x, digit) in score_digits.iter().enumerate() {
            let sprite = Sprites::digit(*digit);
            self.screen.panel_sprite_3x5(sprite, x0 + x as i16);
        }
    }
    fn go_to(&mut self, node: &SnakeNode) {
        self.pos = node.position;
        self.turn(node.direction);
    }
    fn turn(&mut self, direction: Direction) {
        self.to.to = direction;
    }
    fn step(&mut self) {
        self.pos = self.pos.wrapping_sub(self.to);
    }
}
