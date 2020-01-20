use std::fmt::{Display, Formatter, Error};
use std::ops;


#[derive(Copy, Clone)]
struct XY { x: f32, y: f32 }


impl Display for XY {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}


impl ops::Add for XY {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}


impl ops::Sub for XY {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


impl ops::Mul<f32> for XY {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}


impl ops::Mul<XY> for f32 {
    type Output = XY;

    fn mul(self, rhs: XY) -> XY { rhs * self }
}


impl ops::Div<f32> for XY {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}


impl ops::Neg for XY {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}


fn dot(a: XY, b: XY) -> f32 { a.x * b.x + a.y * b.y }


fn cross(a: XY, b: XY) -> f32 { a.x * b.y - a.y * b.x }


macro_rules! xy {
    ($x:expr , $y:expr) => {XY { x: ($x) as f32, y: ($y) as f32 }};
}


fn main() {
    let a = xy!(5, 5);
    let b = xy!(3, 4);
    println!("{}", a);
    println!("{}", b);
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * 3.0);
    println!("{}", 3.0 * b);
    println!("{}", a / 3.0);
    println!("{}", -b);
    println!("{}", dot(a, b));
    println!("{}", cross(a, b));
}