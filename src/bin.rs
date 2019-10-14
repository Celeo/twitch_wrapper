#![allow(unused)]

use log::debug;
use std::{env, fs, path::Path};
use twitch_wrapper::Twitch;

fn main() {
    kankyo::init().expect("Could not load environment");
    env_logger::init();
    debug!("Started");

    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID env var");
    let twitch = Twitch::new(&client_id);

    // let streams = twitch.get_streams(2).unwrap();
    // fs::write(
    //     Path::new("./output.json"),
    //     serde_json::to_string_pretty(&streams).unwrap(),
    // )
    // .unwrap();

    // let streams: twitch_wrapper::models::streams::StreamList = twitch
    //     .query("get", "streams", Some(&[("first", "2")]))
    //     .unwrap();
    // dbg!(&streams);

    // let streams: Vec<twitch_wrapper::models::streams::StreamListItem> = twitch
    //     .query_paginated("GET", "streams", None, 100, 101)
    //     .unwrap();
    // fs::write(
    //     Path::new("./output.json"),
    //     serde_json::to_string_pretty(&streams).unwrap(),
    // )
    // .unwrap();
}
