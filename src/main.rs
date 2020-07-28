use async_std::task;
use chrono::{DateTime, Local, TimeZone};
use huelib::resource::{light, Modifier, ModifierType};
use huelib::{bridge, Bridge};
use std::time::Duration;
mod hue_ctrl;

fn main() {
    let fut_1 = hue_ctrl::fred();
    task::block_on(async {
        fut_1.await;
    });
}
