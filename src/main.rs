#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate iron;
// Message parsing
extern crate rmp;
extern crate rmp_serde;
extern crate serde;
extern crate uuid;
extern crate byteorder;
// giphy requests
extern crate toml;
extern crate hyper;
extern crate serde_json;

use std::io::Read;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status};

mod routing;
mod message;
mod giphy;

fn main() {
    Iron::new(|request : &mut Request| {
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
                        Some(msg) => println!("msg: {:?}", msg),
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
