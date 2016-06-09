#[macro_use]
extern crate iron;
extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;
extern crate uuid;

use std::io::Read;

use iron::{Iron,Request,Response,IronError};
use iron::{method,status};

mod routing;
mod message;

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
                    let mut buf = Vec::new();
                    request.body.read_to_end(&mut buf).unwrap();
                    message::decode_msgpack_body(buf);
                    Ok(Response::with((status::Ok, "ok")))
                } else {
                    Err(IronError::new(routing::NoRoute, status::NotFound))
                }
            }
            _ => Err(IronError::new(routing::NoRoute, status::NotFound))
        }
    }).http("localhost:9999").unwrap();
}
