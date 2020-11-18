struct Square<T> {
    side: T,
}
struct Rectangular<T> {
    length: T,
    width: T,
}

struct Circle<T> {
    radius: T,
}

struct Triangle<T> {
    base: T,
    hight: T,
}

pub trait CalculateArea {
    type Output;
    fn cal_area(&self) -> Self::Output;
}

impl<T: std::ops::Mul<Output = T> + Copy> CalculateArea for Square<T> {
    type Output = T;
    
    fn cal_area(&self) -> Self::Output {
        self.side * self.side
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> CalculateArea for Rectangular<T> {
    type Output = T;

    fn cal_area(&self) -> Self::Output {
        self.length * self.width
    }
}

impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> CalculateArea for Circle<T> {
    type Output = f64;
    
    fn cal_area(&self) -> Self::Output {
        (self.radius * self.radius).into() * std::f64::consts::PI
    }
}

impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> CalculateArea for Triangle<T> {
    type Output = f64;
    
    fn cal_area(&self) -> Self::Output {
        (self.base * self.hight).into() * 0.5
    }
}

fn main() {
    let s = Square {side: 8};
    println!("square: {}", s.cal_area());
    
    let r = Rectangular {length: 6, width: 3};
    println!("Rectangular: {}", r.cal_area());

    let c = Circle {radius: 5};
    println!("circle: {}", c.cal_area());

    let t = Triangle {base: 4, hight: 3};
    println!("Triangle: {}", t.cal_area());
}
