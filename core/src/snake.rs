use crate::types::*;
use std::collections::VecDeque;

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy)]
pub struct SnakeNode {
    pub direction: Direction,
    pub position: FieldPoint,
    pub stuffed: bool,
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone)]
pub struct Snake {
    pub nodes: VecDeque<SnakeNode>,
    pub direction: WrappableDirection,
    pub score: u16,
    pub id: u16,
}

impl Snake {
    pub fn new(field: &mut Field, config: &GameConfig, id: u16) -> Snake {
        let (width, height) = config.dim;
        let max = FieldPoint {
            x: width,
            y: height,
        };
        let (x, y) = config.start;
        let start = FieldPoint { x, y };
        let to = config.direction;
        let mut snake = Snake {
            id,
            nodes: VecDeque::new(),
            direction: WrappableDirection { to, max },
            score: 0,
        };

        snake.egg_hatch(field, start, config.size);
        return snake;
    }

    pub fn head_to(&mut self, to: Direction) -> bool {
        if self.should_ignore_turn(to) {
            return false;
        }
        self.direction.to = to;
        return true;
    }

    pub fn next_head(&self) -> SnakeNode {
        let head = self.nodes.back().unwrap();
        let position = head.position.wrapping_add(self.direction);
        SnakeNode {
            direction: self.direction.to,
            position,
            stuffed: false,
        }
    }

    pub fn should_ignore_turn(&self, to: Direction) -> bool {
        let direction = self.nodes.back().unwrap().direction;
        return opposite_of(direction) == to || direction == to;
    }

    pub fn egg_hatch(&mut self, field: &mut Field, location: FieldPoint, size: u16) {
        self.nodes.push_front(SnakeNode {
            direction: self.direction.to,
            position: location,
            stuffed: false,
        });
        for _ in 1..size {
            let next_head = self.next_head();
            let SnakeNode { position, .. } = next_head;
            self.nodes.push_back(next_head);
            field.set(&position, true);
        }
    }
}
