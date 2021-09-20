extern crate communicator;

pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {  }
    }
  }
}

enum TrafficLight {
  Red,
  Yellow,
  Green
}

enum Name {
  Kim,
  Choi,
  Park,
}

use a::series::of::nested_modules;

use TrafficLight::{Red, Yellow};

use Name::*;

fn main() {
  communicator::client::connect();
  a::series::of::nested_modules();
  nested_modules();

  let red = Red;
  let yellow = Yellow;
  let green = TrafficLight::Green;

  let kim = Kim;
  let choi = Choi;
  let park = Park;
}
