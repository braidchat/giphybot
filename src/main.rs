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
// giphy requests
extern crate hyper;
extern crate serde_json;
// configuration
extern crate toml;

use std::io::Read;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status};
use regex::Regex;

mod routing;
mod message;
mod giphy;
mod conf;

fn strip_leading_name(msg: String) -> String {
    lazy_static! {
        static ref RE: Regex  = Regex::new(r"^/(\w+)\b").unwrap();
    }
    RE.replace(&msg[..], "")
}

fn main() {
    let conf = conf::load_conf("conf.toml").expect("Couldn't load conf file!");
    let bot_name = conf::get_conf_val(&conf, "braid", "name")
        .expect("Missing braid bot name");
    let api_key = conf::get_conf_val(&conf, "giphy", "api_key")
        .expect("Missing giphy api key");
    println!("Bot {:?} starting", bot_name);
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
                            println!("msg: {:?}", msg);
                            let gif = giphy::request_gif(
                                &api_key[..],
                                strip_leading_name(msg.content));
                            println!("gif for message {:?}", gif);
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
