#[macro_use]
extern crate iron;
extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use std::error::Error;
use std::fmt;

use iron::{Iron,Request,Response,Handler,IronResult,IronError};
use iron::{method,status};
use msgpack::{Decoder,Encoder};
use rustc_serialize::{Decodable,Encodable};

#[derive(Debug)]
pub struct NoRoute;

impl fmt::Display for NoRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No matching route found.")
    }
}

impl Error for NoRoute {
    fn description(&self) -> &str { "No Route" }
}

fn main() {
    Iron::new(|request : &mut Request| {
        let req_path = request.url.path.join("/");
        match request.method {
            method::Put => {
                if req_path == "message" {
                    let mac = request.headers.get_raw("X-Braid-Signature");
                    println!("Request mac = {:?}", mac);
                    Ok(Response::with((status::Ok, "ok")))
                } else {
                    Err(IronError::new(NoRoute, status::NotFound))
                }
            }
            _ => Err(IronError::new(NoRoute, status::NotFound))
        }
    }).http("localhost:9999").unwrap();
}
