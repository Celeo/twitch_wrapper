#![allow(unused)]

use log::debug;
use std::env;
use twitch_wrapper::Twitch;

fn main() {
    kankyo::init().expect("Could not load environment");
    env_logger::init();
    debug!("Started");

    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID env var");
    let twitch = Twitch::new(&client_id);
    let streams = twitch.get_streams(3).unwrap();

    dbg!(&streams);
}
