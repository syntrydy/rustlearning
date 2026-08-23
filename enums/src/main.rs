#![allow(unused_variables)]
mod core;

use std::option;

use crate::core::*;

fn main() {
    let color_a = Colors::BLUE;
    let color_b = Colors::GREEN;
    let color_c = Colors::RED;
    println!(" {:#?}", color_b);
    let dead = Status::Dead(2000);
    let alive = Status::Alive;
    println!(" {:?} ans size is {} bytes", dead, size_of::<Status>());
    check_ordering();
    let status = HttpStatus::Ok;
    let status = HttpStatus::NotFound;
    let status = HttpStatus::UnAuthenticated;
    println!(
        "Status code is {:?} and size is {} bytes",
        status as u32,
        size_of::<HttpStatus>()
    );
    let network_a = NetworkMessage::Post("CLOSE EVRYTHING".to_string());
    let network_b = NetworkMessage::Ping;
    let network_c = NetworkMessage::Quit {
        _id: "0256".to_string(),
        reason: Some("Out of memory".to_string()),
    };
    process_message(network_a);
    //trafic light
    let light = TrafficLight::GREEN;
    println!("Can i cross now?: {}", light.can_go());
    if let TrafficLight::RED = light {
        println!("Don't try");
    } else {
        println!("Be ready");
    }
}
