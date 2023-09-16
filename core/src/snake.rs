use crate::types::*;
use std::collections::LinkedList;

#[derive(Debug)]
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
    pub fn opposite(&self) -> Direction {
        match self.direction.to {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    pub fn head_to(&mut self, to: Direction) {
        if to == self.opposite() {
            return;
        }
        self.direction.to = to;
    }

    pub fn next_head(&self) -> SnakeNode {
        let head = self.nodes.front().unwrap();
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
            self.nodes.push_front(next_head);
            field[position.x][position.y] = FieldElement::Snake;
        }
    }

    pub fn new(field: &mut Vec<Vec<FieldElement>>, size: usize) -> Snake {
        let max = FieldPoint {
            x: field.len(),
            y: field[0].len(),
        };
        let start = FieldPoint {
            x: max.x.div_euclid(2),
            y: max.y.div_euclid(2),
        };
        let mut snake = Snake {
            nodes: LinkedList::new(),
            direction: WrappableDirection {
                to: Direction::Right,
                max,
            },
        };

        snake.egg_hatch(field, start, size);
        return snake;
    }
}
