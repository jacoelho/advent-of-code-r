use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

impl Position2D {
    pub fn new(x: i32, y: i32) -> Self {
        Position2D { x, y }
    }

    pub fn distance(&self, rhs: &Position2D) -> usize {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as usize
    }

    pub fn neighbours4(&self) -> Vec<Position2D> {
        let neighbours: Vec<Position2D> = vec![
            Position2D::new(1, 0),
            Position2D::new(-1, 0),
            Position2D::new(0, -1),
            Position2D::new(0, 1),
        ];

        neighbours
            .into_iter()
            .map(|n| n + *self)
            .filter(|p| p.x >= 0 && p.y >= 0)
            .collect::<Vec<_>>()
    }
}

impl fmt::Debug for Position2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Position2D {
    type Output = Position2D;

    fn add(self, other: Position2D) -> Position2D {
        Position2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Position2D {
    fn add_assign(&mut self, other: Position2D) {
        *self = Position2D {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for Position2D {
    type Output = Position2D;

    fn sub(self, other: Position2D) -> Position2D {
        Position2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
