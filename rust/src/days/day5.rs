use std::{str::FromStr, cmp, fmt};

pub enum LineType {
    Horizontal {left: usize, right: usize, y: usize},
    Vertical {bottom: usize, top: usize, x: usize},
    Diagnol45Degrees{left_side: Point, right_side: Point, positive_slope: bool, length: usize},
    Diagonal,
}

#[derive(Debug, Clone, Copy)]
/// Point(x,y)
pub struct Point(pub usize, pub usize);

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points_str: Vec<&str> = s.split(",").collect();
        let x = points_str[0].parse()?;
        let y = points_str[1].parse()?;

        Ok(Point(x, y))
    }
}

#[derive(Debug)]
/// Line segment Point1(x1,y1), Point2(x2,y2)
pub struct LineSegment(Point, Point);

impl LineSegment {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self(
            Point(x1,y1),
            Point(x2,y2)
        )
    }
    /*
        let left = cmp::min(self.0.0, self.1.0);
        let right = cmp::max(self.0.0, self.1.0);
        let y = self.0.1;
        let bottom = cmp::min(self.0.1, self.1.1);
        let top = cmp::max(self.0.1, self.1.1);
        let x = self.0.0;
     */
    
    fn x(&self) -> usize {
        assert!(self.is_vertical());
        self.0.0
    }
    fn y(&self) -> usize {
        assert!(self.is_horizontal());
        self.0.1
    }
    fn top(&self) -> usize {
        assert!(self.is_vertical());
        cmp::max(self.0.1, self.1.1)
    }
    fn bottom(&self) -> usize {
        assert!(self.is_vertical());
        cmp::min(self.0.1, self.1.1)
    }
    fn left(&self) -> usize {
        assert!(self.is_horizontal());
        cmp::min(self.0.0, self.1.0)
    }
    fn right(&self) -> usize {
        assert!(self.is_horizontal());
        cmp::max(self.0.0, self.1.0)
    }
    
    pub fn grid_data(&self) -> LineType {
        if self.is_horizontal() {
            LineType::Horizontal{left: self.left(), right: self.right(), y: self.y()}
        } else if self.is_vertical() {
            LineType::Vertical{bottom: self.bottom(), top: self.top(), x: self.x()}
        } else if self.is_45_degrees() {
            let (left_side, right_side) = if self.0.0 < self.1.0 {(self.0, self.1)} else {(self.1, self.0)};
            let positive_slope = left_side.1 < right_side.1;
            let length = (left_side.0 as i32 - right_side.0 as i32).abs() as usize;
            LineType::Diagnol45Degrees{left_side, right_side, positive_slope, length }
        }
        else {
            LineType::Diagonal
        }
    }
    /// Returns true if line is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.0.1 == self.1.1
    }
    /// Returns true if line is vertical
    pub fn is_vertical(&self) -> bool {
        self.0.0 == self.1.0
    }
    pub fn is_45_degrees(&self) -> bool {
        let x1 = self.0.0 as i32;
        let y1 = self.0.1 as i32;
        let x2 = self.1.0 as i32;
        let y2 = self.1.1 as i32;
        ((x2-x1)/(y2-y1)).abs() == 1
    }

}

impl FromStr for LineSegment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points_str: Vec<&str> = s.split(" -> ").collect();
        let p1 = points_str[0].parse()?;
        let p2 = points_str[1].parse()?;

        Ok(LineSegment(p1, p2))
    }
}

#[derive(Debug)]
/// 10x10 grid
pub struct Grid(Vec<Vec<u16>>);

impl Grid {
    pub fn new(grid_size: usize) -> Self {
        let mut outer_vec = Vec::new();
        let mut inner_vec = Vec::new();
        inner_vec.resize(grid_size, 0u16);
        outer_vec.resize(grid_size, inner_vec.clone());
        Grid(outer_vec)
    }
    pub fn apply_line(&mut self, line: &LineSegment) {
        // println!("Doing line {:?}", line);
        // 
        match line.grid_data() {
            LineType::Horizontal { left, right, y } => {
                // println!("Horizontal Line, filling in lines {left} - {right}, {}", right - left);
                for i in left..=right {
                    self.0[y][i] += 1;
                }
            },
            LineType::Vertical { bottom, top, x } => {
                // println!("Vertical Line, filling in lines {bottom} - {top}, {}", top - bottom);
                for j in bottom..=top {
                    self.0[j][x] += 1;
                }
            },
            LineType::Diagnol45Degrees { left_side, right_side, positive_slope, length } => {
                println!("{:?}, {:?}", left_side, right_side);
                for i in 0..=length {
                    if positive_slope {
                        // println!("{},{}",left_side.0 + i, left_side.1 + i);
                        self.0[left_side.1 + i][left_side.0 + i] += 1;
                    }
                    else {
                        // println!("{},{}",left_side.0 + i, left_side.1 - i);
                        self.0[left_side.1 - i][left_side.0+i] += 1;
                    }
                    
                }
            },
            LineType::Diagonal => println!("Diagonal Line"),
        }
    }
    pub fn count_overlaps(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&cell| cell >= 2).count())
            .sum()
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            
            for &cell in row {
                if cell < 1 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }   
            write!(f, "\n")?;
        }
        Ok(())
        
    }
}


pub mod part1 {

}
pub mod part2 {
    
}