use crate::{game::GameConfig, types::*};
use std::collections::LinkedList;

#[derive(Debug, Clone, Copy)]
pub struct SnakeNode {
    pub direction: Direction,
    pub position: FieldPoint,
}

#[derive(Debug)]
pub struct Snake {
    pub nodes: LinkedList<SnakeNode>,
    pub direction: WrappableDirection,
}

impl Snake {
    pub fn should_ignore_turn(&self, to: Direction) -> bool {
        let direction = self.nodes.back().unwrap().direction;
        return opposite_of(direction) == to || direction == to;
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
        let position = head.position.add_wrapping(self.direction);
        SnakeNode {
            direction: self.direction.to,
            position,
        }
    }

    pub fn egg_hatch(&mut self, field: &mut Field, location: FieldPoint, size: u16) {
        self.nodes.push_front(SnakeNode {
            direction: self.direction.to,
            position: location,
        });
        for _ in 1..size {
            let next_head = self.next_head();
            let SnakeNode { position, .. } = next_head;
            self.nodes.push_back(next_head);
            field.set(&position, true);
        }
    }

    pub fn new(field: &mut Field, config: GameConfig) -> Snake {
        let (width, height) = config.dim;
        let max = FieldPoint {
            x: width,
            y: height,
        };
        let (x, y) = config.start;
        let start = FieldPoint { x, y };
        let to = config.direction;
        let mut snake = Snake {
            nodes: LinkedList::new(),
            direction: WrappableDirection { to, max },
        };

        snake.egg_hatch(field, start, config.size);
        return snake;
    }
}
