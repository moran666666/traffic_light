pub trait Light {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Light for TrafficLight {
    fn time(&self) -> u8 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 45,
            TrafficLight::Yellow => 3,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    println!("the red light: {}", red.time());

    let green = TrafficLight::Green;
    println!("the green light: {}", green.time());

    let yellow = TrafficLight::Yellow;
    println!("the yellow light: {}", yellow.time());
}
