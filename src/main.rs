use huelib::{bridge, Bridge};
use std::time::Duration;

use async_std::task;

fn detected(sensor: &huelib::resource::sensor::Sensor) {
    match sensor.state.presence {
        Some(_p) => println!("{:?}", sensor),
        None => {}
    }
}

#[async_std::main]
async fn main() {
    // Get the IP address of the bridge that was first discovered in the local network.
    let ip_address = bridge::discover()
        .expect("Failed to discover bridges")
        .pop()
        .expect("No bridges found in the local network");

    // Create a bridge with IP address and username.
    let bridge = Bridge::new(ip_address, "oUws6LxbEowJ6av5ksrsryMV3Y9MqV7YGglfJtL4");

    loop {
        let sensors = bridge.get_all_sensors();
        match sensors {
            Ok(p) => p.iter().for_each(|response| detected(response)),
            Err(_) => {}
        }
        task::sleep(Duration::from_secs(3)).await;
    }
}
