use snake::types::Direction;

pub struct SpritesBinary {}

impl SpritesBinary {
    pub fn food() -> u8 {
        return 0b010_101_01;
    }

    #[allow(dead_code)]
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

    pub fn full_node(from: Direction, to: Direction) -> [u8; 2] {
        return [SpritesBinary::block(from), SpritesBinary::corner(from, to)];
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
    pub fn node_dash(direction: Direction) -> u8 {
        return match direction {
            Direction::Down | Direction::Right => SpritesBinary::back_dash(direction),
            Direction::Up | Direction::Left => SpritesBinary::dash(direction),
        };
    }
}
