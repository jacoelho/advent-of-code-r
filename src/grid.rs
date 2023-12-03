use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

impl Position2D {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn distance(&self, rhs: &Self) -> usize {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as usize
    }

    pub fn neighbours4(&self) -> Vec<Self> {
        let neighbours: Vec<Self> = vec![
            Self::new(1, 0),
            Self::new(-1, 0),
            Self::new(0, -1),
            Self::new(0, 1),
        ];

        neighbours
            .into_iter()
            .map(|n| n + *self)
            .filter(|p| p.x >= 0 && p.y >= 0)
            .collect::<Vec<_>>()
    }

    pub fn neighbours8(&self) -> Vec<Self> {
        let neighbours: Vec<Self> = vec![
            Self::new(-1, -1),
            Self::new(0, -1),
            Self::new(1, -1),
            Self::new(-1, 0),
            Self::new(1, 0),
            Self::new(-1, 1),
            Self::new(0, 1),
            Self::new(1, 1),
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
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::AddAssign for Position2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self { x: self.x + other.x, y: self.y + other.y };
    }
}

impl std::ops::Sub for Position2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}
