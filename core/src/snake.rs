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
    pub fn is_this_eat(element: FieldElement) -> bool {
        match element {
            FieldElement::Empty | FieldElement::Snake => false,
            _ => true,
        }
    }
    pub fn opposite_of(direction: Direction) -> Direction {
        match direction {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
    pub fn ignore_to(&self, to: Direction) -> bool {
        let direction = self.nodes.back().unwrap().direction;
        return Snake::opposite_of(direction) == to || direction == to;
    }

    pub fn head_to(&mut self, to: Direction) -> bool {
        if self.ignore_to(to) {
            return false;
        }
        self.direction.to = to;
        return true;
    }

    pub fn next_head(&self) -> SnakeNode {
        let head = self.nodes.back().unwrap();
        let position = head.position.add(self.direction);
        SnakeNode {
            direction: self.direction.to,
            position,
        }
    }

    pub fn egg_hatch(
        &mut self,
        field: &mut Vec<Vec<FieldElement>>,
        location: FieldPoint,
        size: usize,
    ) {
        self.nodes.push_front(SnakeNode {
            direction: self.direction.to,
            position: location,
        });
        for _ in 1..size {
            let next_head = self.next_head();
            let SnakeNode { position, .. } = next_head;
            self.nodes.push_back(next_head);
            field[position.x][position.y] = FieldElement::Snake;
        }
    }

    pub fn new(field: &mut Vec<Vec<FieldElement>>, config: GameConfig) -> Snake {
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
