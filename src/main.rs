use async_std::task;
use chrono::{DateTime, Local, TimeZone};
use huelib::resource::{light, Modifier, ModifierType};
use huelib::{bridge, Bridge};
use std::time::Duration;
mod clap_fern;
mod cli_clap;
mod hue_ctrl;

fn main() {
    let clap_matches = cli_clap::cli_clap();
    clap_fern::log_setup(&clap_matches);

    let fut_1 = hue_ctrl::fred();
    task::block_on(async {
        fut_1.await;
    });
}
