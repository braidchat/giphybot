#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

// main
#[macro_use] extern crate iron;
extern crate regex;
#[macro_use] extern crate lazy_static;
// Message parsing
extern crate rmp;
extern crate rmp_serde;
extern crate serde;
extern crate uuid;
extern crate byteorder;
// giphy/braid requests
extern crate hyper;
extern crate mime;
extern crate serde_json;
// configuration
extern crate toml;

use std::io::Read;
use std::thread;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status};
use regex::Regex;

mod conf;
mod routing;
mod message;
mod giphy;
mod braid;

fn strip_leading_name(msg: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^/(\w+)\b").unwrap();
    }
    RE.replace(&msg[..], "")
}

fn main() {
    let conf = conf::load_conf("conf.toml").expect("Couldn't load conf file!");
    let giphy_api_key = conf::get_conf_val(&conf, "giphy", "api_key")
        .expect("Missing giphy api key");
    let braid_conf = conf::get_conf_group(&conf, "braid")
        .expect("Missing braid config information");
    let keys = ["name", "api_url", "app_id", "token"];
    for i in 0..keys.len() {
        let k = keys[i];
        if !braid_conf.contains_key(k) {
            panic!("Missing braid configuration key '{}'", k);
        }
    }
    println!("Bot {:?} starting", braid_conf.get("name").unwrap().as_str().unwrap());
    Iron::new(move |request : &mut Request| {
        let req_path = request.url.path.join("/");
        match request.method {
            method::Put => {
                if req_path == "message" {
                    let mac = request.headers.get_raw("X-Braid-Signature");
                    // TODO: verify mac
                    println!("Request mac = {:?}", mac);
                    let mut buf = Vec::new();
                    request.body.read_to_end(&mut buf).unwrap();
                    match message::decode_transit_msgpack(buf) {
                        Some(msg) => {
                            let braid_conf = braid_conf.clone();
                            let giphy_api_key = giphy_api_key.clone();
                            thread::spawn(move || {
                                println!("msg: {:?}", msg);
                                let gif = giphy::request_gif(
                                    &giphy_api_key[..],
                                    strip_leading_name(msg.content));
                                println!("gif for message {:?}", gif);
                                braid::send_braid_request(
                                    &braid_conf, message::random_message());
                            });
                        },
                        None => println!("Couldn't parse message")
                    }
                    Ok(Response::with((status::Ok, "ok")))
                } else {
                    Err(IronError::new(routing::NoRoute, status::NotFound))
                }
            }
            _ => Err(IronError::new(routing::NoRoute, status::NotFound))
        }
    }).http("localhost:9999").unwrap();
}
