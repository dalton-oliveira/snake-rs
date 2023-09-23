use crate::types::*;
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
    pub fn opposite_of(direction: Direction) -> Direction {
        match direction {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
    pub fn opposite(&self) -> Direction {
        let direction = self.nodes.back().unwrap().direction;
        return Snake::opposite_of(direction);
    }

    pub fn head_to(&mut self, to: Direction) {
        if to == self.opposite() {
            return;
        }
        self.direction.to = to;
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

    pub fn new(field: &mut Vec<Vec<FieldElement>>, size: usize, to: Direction) -> Snake {
        let max = FieldPoint {
            x: field.len(),
            y: field[0].len(),
        };
        let start = FieldPoint {
            // x: max.x.div_euclid(2),
            // y: max.y.div_euclid(2),
            x: 0,
            y: 0,
        };
        let mut snake = Snake {
            nodes: LinkedList::new(),
            direction: WrappableDirection { to, max },
        };

        snake.egg_hatch(field, start, size);
        return snake;
    }
}
