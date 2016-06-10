use std::io::Cursor;
use byteorder::{WriteBytesExt,ReadBytesExt,BigEndian};
use uuid::Uuid;
use serde;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer};

#[derive(Debug, PartialEq, Deserialize)]
struct Message {
    #[serde(rename="~:id", deserialize_with="deserialize_transit_uuid")]
    id: Uuid,
    #[serde(rename="~:group-id", deserialize_with="deserialize_transit_uuid")]
    group_id: Uuid,
    #[serde(rename="~:thread-id", deserialize_with="deserialize_transit_uuid")]
    thread_id: Uuid,
    #[serde(rename="~:user-id", deserialize_with="deserialize_transit_uuid")]
    user_id: Uuid,
    #[serde(rename="~:mentioned-user-ids", deserialize_with="deserialize_transit_uuid_seq")]
    mentioned_user_ids: Vec<Uuid>,
    #[serde(rename="~:mentioned-tag-ids", deserialize_with="deserialize_transit_uuid_seq")]
    mentioned_tag_ids: Vec<Uuid>,
    #[serde(rename="~:content")]
    content: String,
}

type TransitUuid = (String, (i64, i64));

fn deserialize_transit_uuid<D>(de: &mut D) -> Result<Uuid, D::Error>
where D: serde::Deserializer {
    let transit_uuid: TransitUuid = try!(Deserialize::deserialize(de));
    Ok(transit_to_uuid(transit_uuid))
}

fn deserialize_transit_uuid_seq<D>(de: &mut D) -> Result<Vec<Uuid>, D::Error>
where D: serde::Deserializer {
    let transit_uuids: Vec<TransitUuid> = try!(Deserialize::deserialize(de));
    Ok(transit_uuids.into_iter().map(transit_to_uuid).collect())
}

fn transit_to_uuid(transit_uuid: TransitUuid) -> Uuid {
    assert!(transit_uuid.0 == "~#u", "Mis-tagged transit");
    let mut wrtr = vec![];
    let hi64 = (transit_uuid.1).0;
    let lo64 = (transit_uuid.1).1;
    wrtr.write_i64::<BigEndian>(hi64).unwrap();
    wrtr.write_i64::<BigEndian>(lo64).unwrap();
    let mut bytes: [u8; 16] = [0; 16];
    for i in 0..wrtr.len() {
        bytes[i] = wrtr[i];
    }
    Uuid::from_bytes(&bytes).ok().unwrap()
}

fn uuid_to_transit(uuid: Uuid) -> TransitUuid {
    let bytes = uuid.as_bytes();
    let mut reader = Cursor::new(bytes);
    let hi64 = reader.read_i64::<BigEndian>().unwrap();
    let lo64 = reader.read_i64::<BigEndian>().unwrap();
    ("~#u".to_string(), (hi64, lo64))
}

/*
fn test_response() {
    let test_message = Message {
        id: Uuid::new_v4(),
        group_id: Uuid::new_v4(),
        thread_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        mentioned_user_ids: vec![],
        mentioned_tag_ids: vec![],
        content: "response back!".to_string()
    };
}
*/
pub fn decode_msgpack_body(msgpack_buf: Vec<u8>) {
    let cur = Cursor::new(&msgpack_buf[..]);
    let mut deserializer = Deserializer::new(cur);
    match Deserialize::deserialize(&mut deserializer) {
        Result::Ok(res) => {
            let message: Message = res;
            println!("message = {:?}", message);
            //test_response();
        }
        Result::Err(err) =>
            println!("error parsing: {:?}", err)
    }
}

