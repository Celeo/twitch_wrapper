#![allow(unused)]

use log::debug;
use std::{env, fs, path::Path};
use twitch_wrapper::Twitch;

fn main() {
    kankyo::init().expect("Could not load environment");
    env_logger::init();
    debug!("Started");

    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID env var");
    let _twitch = Twitch::new(&client_id);

    // ...
}
