#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light{
    fn time(&self) -> i32;
}

impl Light for TrafficLight {
    fn time(&self) -> i32 {
        return match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 45,
            TrafficLight::Yellow => 3,
        }
    }
}
fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("Red light will be {} seconds", red.time());
    println!("Green light will be {} seconds", green.time());
    println!("Yellow light will be {} seconds", yellow.time());
}
