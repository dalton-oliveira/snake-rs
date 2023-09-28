use snake::types::{Direction, FoodType};

pub struct SpritesBinary {}

impl SpritesBinary {
    pub fn food() -> u8 {
        return 0b010_101_01;
    }
    pub fn special_food(element: FoodType) -> u32 {
        match element {
            FoodType::Whale => 0b00001100_10011010_10111110_01111111,
            FoodType::Turtle => 0b11000100_11001110_00111111_00001010,
            FoodType::Alien => 0b01010100_11111111_10111101_10100101,
            FoodType::Chameleon => 0b01010100_10111110_11111111_00100100,
            FoodType::Elephant => 0b00011000_00101101_01111111_00011110,
            FoodType::Caterpillar => 0b00000000_10000000_11111111_01010101,
            _ => 0b11111111_11111111_11111111_11111111,
        }
    }
    pub fn digit(digit: u8) -> u16 {
        match digit {
            0 => 0b111_101_101_101_111_0,
            1 => 0b010_110_010_010_010_0,
            2 => 0b111_001_111_100_111_0,
            3 => 0b111_001_111_001_111_0,
            4 => 0b101_101_111_001_001_0,
            5 => 0b111_100_111_001_111_0,
            6 => 0b111_100_111_101_111_0,
            7 => 0b111_001_010_010_010_0,
            8 => 0b111_101_111_101_111_0,
            9 => 0b111_101_111_001_111_0,
            _ => 0,
        }
    }
    pub fn eyes(direction: Direction) -> u8 {
        return match direction {
            Direction::Right => 0b0110_1100,
            Direction::Left => 0b1001_1100,
            Direction::Up => 0b1010_0110,
            Direction::Down => 0b0110_1010,
        };
    }

    pub fn full_head(from: Direction, open: bool) -> [u8; 2] {
        return [SpritesBinary::eyes(from), SpritesBinary::mouth(from, open)];
    }
    pub fn dash_with_food(sprite: u8, direction: Direction) -> u8 {
        match direction {
            Direction::Down | Direction::Up => sprite | 0b1001_1001,
            Direction::Left | Direction::Right => sprite | 0b1100_0011,
        }
    }
    pub fn full_node(from: Direction, to: Direction, has_food: bool) -> [u8; 2] {
        let sprites = [SpritesBinary::block(from), SpritesBinary::corner(from, to)];
        if has_food == false {
            return sprites;
        }
        if from == to {
            return [sprites[0], SpritesBinary::dash_with_food(sprites[1], from)];
        }
        return [
            SpritesBinary::block_with_food(from, to, sprites[0]),
            sprites[1],
        ];
    }
    pub fn full_tail(direction: Direction) -> [u8; 2] {
        return [
            SpritesBinary::dot(direction),
            SpritesBinary::tail(direction),
        ];
    }

    pub fn tail(direction: Direction) -> u8 {
        return match direction {
            Direction::Left => 0b0010_1100,
            Direction::Right => 0b0001_1100,
            Direction::Up => 0b0110_0010,
            Direction::Down => 0b0010_0110,
        };
    }
    pub fn block(direction: Direction) -> u8 {
        return match direction {
            Direction::Up | Direction::Down => 0b0110_0110,
            Direction::Left | Direction::Right => 0b0011_1100,
        };
    }
    pub fn back_dash(direction: Direction) -> u8 {
        match direction {
            Direction::Up | Direction::Down => 0b0100_0010,
            Direction::Left | Direction::Right => 0b0010_0100,
        }
    }
    pub fn dash(direction: Direction) -> u8 {
        return match direction {
            Direction::Right | Direction::Left => 0b0001_1000,
            Direction::Up | Direction::Down => 0b0010_0100,
        };
    }
    pub fn mouth_open(direction: Direction) -> u8 {
        return match direction {
            Direction::Left | Direction::Up => 0b1001_0110,
            Direction::Right | Direction::Down => 0b0110_1001,
        };
    }
    pub fn mouth(direction: Direction, mouth_open: bool) -> u8 {
        return match mouth_open {
            true => SpritesBinary::mouth_open(direction),
            false => SpritesBinary::block(direction),
        };
    }
    pub fn dot(direction: Direction) -> u8 {
        return match direction {
            Direction::Left => 0b0000_1000,
            Direction::Right => 0b0000_0100,
            Direction::Up => 0b0010_0000,
            Direction::Down => 0b0000_0010,
        };
    }
    pub fn corner(from: Direction, to: Direction) -> u8 {
        let dash = SpritesBinary::dash(from);
        let back_dash = SpritesBinary::back_dash(from);
        match (from, to) {
            (Direction::Right, Direction::Up) => dash,
            (Direction::Left, Direction::Down) => dash,
            (Direction::Down, Direction::Left) => dash,
            (Direction::Up, Direction::Right) => dash,
            (Direction::Right, Direction::Down) => back_dash,
            (Direction::Left, Direction::Up) => back_dash,
            (Direction::Down, Direction::Right) => back_dash,
            (Direction::Up, Direction::Left) => back_dash,
            (direction, _) => SpritesBinary::node_dash(direction), // not a corner
        }
    }

    pub fn block_with_food(from: Direction, to: Direction, sprite: u8) -> u8 {
        let mask = match (from, to) {
            (Direction::Right, Direction::Up) => 0b0100_0000,
            (Direction::Left, Direction::Down) => 0b1000_0000,
            (Direction::Down, Direction::Left) => 0b0000_1000,
            (Direction::Up, Direction::Right) => 0b0001_0000,
            (Direction::Right, Direction::Down) => 0b0000_0001,
            (Direction::Left, Direction::Up) => 0b1000_0000,
            (Direction::Down, Direction::Right) => 0b0000_0001,
            (Direction::Up, Direction::Left) => 0b1000_0000,
            (_, _) => 0,
        };
        return sprite | mask;
    }
    pub fn node_dash(direction: Direction) -> u8 {
        return match direction {
            Direction::Down | Direction::Right => SpritesBinary::back_dash(direction),
            Direction::Up | Direction::Left => SpritesBinary::dash(direction),
        };
    }
}
