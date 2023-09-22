use snake::types::Direction;

pub struct SpritesBinary {}

impl SpritesBinary {
    pub fn head_only(direction: Direction) -> u8 {
        return match direction {
            Direction::Right => 0b0110_1100,
            Direction::Left => 0b1001_1100,
            Direction::Up => 0b1010_0110,
            Direction::Down => 0b0110_1010,
        };
    }

    pub fn head(direction: Direction, mouth_open: bool) -> [u8; 2] {
        let mouth = SpritesBinary::mouth(direction, mouth_open);
        let head = SpritesBinary::head_only(direction);
        return match direction {
            Direction::Right | Direction::Down => [head, mouth],
            Direction::Left | Direction::Up => [mouth, head],
        };
    }

    pub fn mouth(direction: Direction, mouth_open: bool) -> u8 {
        if mouth_open {
            return SpritesBinary::mouth_open(direction);
        }
        return SpritesBinary::full_block(direction);
    }

    pub fn corner(from: Direction, to: Direction) -> u8 {
        let dash = SpritesBinary::dash(from);
        let back_dash = SpritesBinary::back_dash(from);
        match (from, to) {
            (Direction::Right, Direction::Up) => dash,
            (Direction::Right, Direction::Down) => back_dash,
            (Direction::Left, Direction::Up) => back_dash,
            (Direction::Left, Direction::Down) => dash,
            (Direction::Down, Direction::Left) => dash,
            (Direction::Down, Direction::Right) => back_dash,
            (Direction::Up, Direction::Left) => back_dash,
            (Direction::Up, Direction::Right) => dash,
            (direction, _) => SpritesBinary::node_dash(direction),
        }
    }

    pub fn node_dash(direction: Direction) -> u8 {
        let back_dash = SpritesBinary::back_dash(direction);
        let dash = SpritesBinary::dash(direction);
        return match direction {
            Direction::Down | Direction::Right => back_dash,
            Direction::Up | Direction::Left => dash,
        };
    }

    pub fn node(direction: Direction, next_direction: Direction) -> [u8; 2] {
        let full_block = SpritesBinary::full_block(direction);
        let dash = SpritesBinary::corner(direction, next_direction);
        return match direction {
            Direction::Right | Direction::Down => [full_block, dash],
            Direction::Left | Direction::Up => [dash, full_block],
        };
    }

    pub fn tail_only(direction: Direction) -> u8 {
        return match direction {
            Direction::Left => 0b0010_1100,
            Direction::Right => 0b0001_1100,
            Direction::Up => 0b0110_0010,
            Direction::Down => 0b0010_0110,
        };
    }

    pub fn tail(direction: Direction) -> [u8; 2] {
        let dot = SpritesBinary::dot(direction);
        let tail_only = SpritesBinary::tail_only(direction);
        return match direction {
            Direction::Left | Direction::Up => [tail_only, dot],
            Direction::Right | Direction::Down => [dot, tail_only],
        };
    }

    pub fn full_block(direction: Direction) -> u8 {
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
            Direction::Left => 0b1001_0110,
            Direction::Right => 0b0110_1001,
            Direction::Up => 0b1001_0110,
            Direction::Down => 0b0110_1001,
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
}
