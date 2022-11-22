// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;
impl Direction {

    fn turn_right(&self) -> Self  {
        match self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }


    fn turn_left(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South
        }
    }
}

#[derive(Copy, Clone)]
pub struct Robot{
    x: i32,
    y:i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {x, y, d}
    }

    #[must_use]
    pub fn turn_right(&mut self) -> Self {
        self.d = self.d.turn_right();
        *self
    }

    #[must_use]
    pub fn turn_left(&mut self) -> Self {
        self.d = self.d.turn_left();
        *self
    }

    #[must_use]
    pub fn advance(&mut self) -> Self {
        match self.d {
            North => self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1
        }
        *self

    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        // sample input = "RRAAAAALA"
        instructions.chars().for_each(|x| {
            match x {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => self
            };
        });
        self
        
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}