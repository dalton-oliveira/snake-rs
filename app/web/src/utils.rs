use std::collections::LinkedList;

use snake::{
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, WrappableDirection},
};

pub fn build_snake(positions: Vec<(u32, u32, Direction)>, max: &FieldPoint) -> Snake {
    let to = positions.last().unwrap().2;
    let direction = WrappableDirection {
        to,
        max: max.clone(),
    };
    let mut snake = Snake {
        nodes: LinkedList::new(),
        direction,
    };
    for i in 0..positions.len() {
        let (x, y, direction) = positions[i];

        snake.nodes.push_back(SnakeNode {
            direction,
            position: FieldPoint {
                x: x as usize,
                y: y as usize,
            },
        })
    }
    return snake;
}

pub fn to_base_10_array(n: u16, digits: u8) -> Vec<u8> {
    let mut digits = vec![0; digits as usize];
    let mut n = n;
    let mut i = digits.len() - 1;
    while n > 0 {
        digits[i] = (n % 10) as u8;
        i -= 1;
        n /= 10;
    }

    return digits;
}
