#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

// main
#[macro_use] extern crate iron;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate openssl;
extern crate rustc_serialize;
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
use std::error::Error;
use std::env;
use std::process;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status};
use regex::Regex;
use openssl::crypto::hmac;
use openssl::crypto::hash::Type;
use rustc_serialize::hex::FromHex;

mod conf;
mod routing;
mod message;
mod giphy;
mod braid;

fn strip_leading_name(msg: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^/(\w+)\b").unwrap();
    }
    RE.replace(msg, "")
}

fn verify_hmac(mac: Vec<u8>, key: &[u8], data: &[u8]) -> bool {
    if let Some(mac) = String::from_utf8(mac).ok()
        .and_then(|mac_str| (&mac_str[..]).from_hex().ok()) {
            let generated: Vec<u8> = hmac::hmac(Type::SHA256, key, data).to_vec();
            mac == generated
        } else {
            false
        }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} <configuration toml file>", args[0]);
        process::exit(1);
    }
    let ref conf_filename = args[1];
    let conf = conf::load_conf(&conf_filename[..]).expect("Couldn't load conf file!");
    let bind_port = conf::get_conf_val(&conf, "general", "port")
        .expect("Missing key port in section general");
    let bind_addr = format!("localhost:{}", bind_port);
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
    let braid_token = conf::get_conf_val(&conf, "braid", "token").unwrap();
    println!("Bot {:?} starting", braid_conf.get("name").unwrap().as_str().unwrap());
    Iron::new(move |request : &mut Request| {
        let req_path = request.url.path.join("/");
        match request.method {
            method::Put => {
                if req_path == "message" {
                    let mac = try!(request.headers.get_raw("X-Braid-Signature")
                                   .and_then(|h| h.get(0))
                                   .ok_or(IronError::new(routing::MissingMac,
                                                         status::Unauthorized)));
                    let mut buf = Vec::new();
                    request.body.read_to_end(&mut buf).unwrap();
                    if !verify_hmac(mac.clone(), braid_token.as_bytes(), &buf[..]) {
                        println!("Bad mac");
                        return Err(IronError::new(routing::BadMac, status::Forbidden));
                    }
                    println!("Mac OK");
                    match message::decode_transit_msgpack(buf) {
                        Some(msg) => {
                            let braid_conf = braid_conf.clone();
                            let giphy_api_key = giphy_api_key.clone();
                            thread::spawn(move || {
                                let gif = giphy::request_gif(
                                    &giphy_api_key[..],
                                    strip_leading_name(&msg.content[..]))
                                    .unwrap_or("Couldn't find anything :(".to_owned());
                                println!("gif for message {:?}", gif);
                                let response_msg = message::response_to(
                                    msg, gif);
                                let braid_resp = braid::send_braid_request(
                                    &braid_conf, response_msg);
                                match braid_resp {
                                    Ok(_) => println!("Sent message to braid"),
                                    Err(e) =>
                                        println!("Failed to send to braid: {:?}",
                                                 e.description()),
                                }
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
    }).http(&bind_addr[..]).unwrap();
}
