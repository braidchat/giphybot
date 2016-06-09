use uuid::Uuid;
use msgpack::{Decoder};
use std::collections::BTreeMap;
use rustc_serialize::{Decodable};

type TransitUuid = (String, (u64, u64));

#[derive(RustcEncodable, RustcDecodable, Debug)]
enum MessageField {
    Id(TransitUuid),
    //group_id: TransitUuid,
    //thread_id: TransitUuid,
    //user_id: TransitUuid,
    //mentioned_user_ids: Vec<TransitUuid>,
    //mentioned_tag_ids: Vec<TransitUuid>,
    Content(String),
}

pub fn decode_msgpack_body(msgpack_buf: Vec<u8>) {
    let mut decoder = Decoder::new(&msgpack_buf[..]);
    match Decodable::decode(&mut decoder) {
        Result::Ok(res) => {
            let res: BTreeMap<String, MessageField> = res;
            println!("result = {:?}", res);
        }
        Result::Err(err) =>
            println!("error parsing: {:?}", err)
    }
}

