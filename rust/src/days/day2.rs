use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Submarine {
    position: Position,
    aim: i32,
}
impl Submarine {
    /// Create a new Submarine with default values
    pub fn new(x: i32, y: i32, aim: i32) -> Self {
        Self {
            position: Position::new(x, y),
            aim,
        }
    }
    /// Go forward, increasing horizontal position and modifying aim
    pub fn forward(&mut self, magnitude: i32) {
        self.position.x += magnitude;
        self.position.y += self.aim * magnitude;
    }
    /// Go up, decreasing aim
    pub fn up(&mut self, magnitude: i32) {
        self.aim -= magnitude;
    }
    /// Go down, increasing aim
    pub fn down(&mut self, magnitude: i32) {
        self.aim += magnitude;
    }
    /// Get position
    pub fn get_pos(&self) -> Position {
        self.position
    }
    /// Get aim
    pub fn get_aim(&self) -> i32 {
        self.aim
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Command {
    pub direction: Direction,
    pub magnitude: i32,
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let direction = split[0].parse()?;
        let magnitude = split[1].parse()?;

        Ok(Self {
            direction,
            magnitude,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Invalid Direction given {}", s)),
        }
    }
}
