#[macro_use]
extern crate iron;
extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use std::error::Error;
use std::fmt;
use std::io::Read;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status,request};
use msgpack::{Decoder};
use rustc_serialize::{Decodable};

// Route error handler
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

// Decoding transit+msgpack body
fn decode_msgpack_body<'a, 'b>(req_body: &mut request::Body<'a, 'b>) {
    let mut buf = Vec::new();
    req_body.read_to_end(&mut buf).unwrap();
    let mut decoder = Decoder::new(&buf[..]);
    let res: (u8, String) = Decodable::decode(&mut decoder).unwrap();
    println!("result = {:?}", res);
}

// Main
fn main() {
    Iron::new(|request : &mut Request| {
        let req_path = request.url.path.join("/");
        match request.method {
            method::Put => {
                if req_path == "message" {
                    let mac = request.headers.get_raw("X-Braid-Signature");
                    // TODO: verify mac
                    println!("Request mac = {:?}", mac);
                    {
                        decode_msgpack_body(&mut request.body);
                    }
                    Ok(Response::with((status::Ok, "ok")))
                } else {
                    Err(IronError::new(NoRoute, status::NotFound))
                }
            }
            _ => Err(IronError::new(NoRoute, status::NotFound))
        }
    }).http("localhost:9999").unwrap();
}
