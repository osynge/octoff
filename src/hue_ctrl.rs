use async_std::task;
use chrono::{DateTime, Local, TimeZone};
use huelib::resource::{light, Modifier, ModifierType};
use huelib::{bridge, Bridge};
use std::time::Duration;

fn detected(sensor: &huelib::resource::sensor::Sensor) {
    match sensor.state.presence {
        Some(_p) => {
            println!("{:?}", sensor);
            match sensor.state.last_updated {
                Some(foo) => {
                    let now = Local::now().naive_utc();
                    let dur = now.signed_duration_since(foo);
                    println!("signed_duration_since:{:?}", dur);
                    let bing = dur.num_minutes();
                    println!("mins=:{:?}", bing);
                }
                None => {}
            }
        }
        None => {}
    }
}

fn light(sensor: &huelib::resource::light::Light) {
    println!("{:?}", sensor)
}

fn rule(sensor: &huelib::resource::Rule) {
    println!("{:?}", sensor)
}

pub(super) async fn fred() {
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

        let modifier_on = light::StateModifier::new().on(true);
        let response = bridge.set_light_state("15", &modifier_on).unwrap();
        println!("{:?}", response);
        /*
        let lights = bridge.get_all_lights();
        match lights {
            Ok(p) => p.iter().for_each(|response| light(response)),
            Err(_) => {}
        }
        */
        let all_rules = bridge.get_all_rules();
        match all_rules {
            Ok(p) => p.iter().for_each(|response| rule(response)),
            Err(_) => {}
        }

        task::sleep(Duration::from_secs(3)).await;
    }
}
