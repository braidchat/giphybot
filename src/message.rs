use std::io::Cursor;
use byteorder::{WriteBytesExt,ReadBytesExt,BigEndian};
use uuid::Uuid;
use serde;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename="~:id", deserialize_with="deserialize_transit_uuid", serialize_with="serialize_transit_uuid")]
    id: Uuid,
    #[serde(rename="~:group-id", deserialize_with="deserialize_transit_uuid", serialize_with="serialize_transit_uuid")]
    group_id: Uuid,
    #[serde(rename="~:thread-id", deserialize_with="deserialize_transit_uuid", serialize_with="serialize_transit_uuid")]
    thread_id: Uuid,
    #[serde(rename="~:user-id", deserialize_with="deserialize_transit_uuid", serialize_with="serialize_transit_uuid")]
    user_id: Uuid,
    #[serde(rename="~:mentioned-user-ids", deserialize_with="deserialize_transit_uuid_seq", serialize_with="serialize_transit_uuid_seq")]
    mentioned_user_ids: Vec<Uuid>,
    #[serde(rename="~:mentioned-tag-ids", deserialize_with="deserialize_transit_uuid_seq", serialize_with="serialize_transit_uuid_seq")]
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

fn serialize_transit_uuid<S>(uuid: &Uuid, se: &mut S) -> Result<(), S::Error>
where S: serde::Serializer {
    let transit_uuid = uuid_to_transit(uuid);
    match transit_uuid.serialize(se) {
        Ok(_) => Ok(()),
        Err(_) => Err(serde::ser::Error::custom("Failed to serialize uuid")),
    }
}

fn serialize_transit_uuid_seq<S>(uuids: &Vec<Uuid>, se: &mut S) -> Result<(), S::Error>
where S: serde::Serializer {
    let transit_uuids: Vec<TransitUuid> = uuids.into_iter().map(uuid_to_transit).collect();
    match transit_uuids.serialize(se) {
        Ok(_) => Ok(()),
        Err(_) => Err(serde::ser::Error::custom("Failed to serialize uuid vector")),
    }
}

fn uuid_to_transit(uuid: &Uuid) -> TransitUuid {
    let bytes = uuid.as_bytes();
    let mut reader = Cursor::new(bytes);
    let hi64 = reader.read_i64::<BigEndian>().unwrap();
    let lo64 = reader.read_i64::<BigEndian>().unwrap();
    ("~#u".to_string(), (hi64, lo64))
}

fn test_response() {
    use std::io::prelude::*;
    use std::fs::File;
    //use std::io::Write;
    use rmp::Marker;
    use rmp::encode::{ValueWriteError, write_map_len, write_str};
    use rmp_serde::encode::VariantWriter;

    struct StructMapWriter;

    impl VariantWriter for StructMapWriter {
        fn write_struct_len<W>(&self, wr: &mut W, len: u32) -> Result<Marker, ValueWriteError>
            where W: Write
            {
                write_map_len(wr, len)
            }

        fn write_field_name<W>(&self, wr: &mut W, _key: &str) -> Result<(), ValueWriteError>
            where W: Write
            {
                write_str(wr, _key)
            }
    }

    let test_message = Message {
        id: Uuid::new_v4(),
        group_id: Uuid::new_v4(),
        thread_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        mentioned_user_ids: vec![Uuid::new_v4()],
        mentioned_tag_ids: vec![Uuid::new_v4(), Uuid::new_v4()],
        content: "response back!".to_string()
    };

    let mut buf = vec![];
    test_message.serialize(&mut Serializer::with(&mut &mut buf, StructMapWriter)).ok().unwrap();
    let mut f: File = File::create("message_from_bot.msgpack").ok().unwrap();
    f.write_all(&buf[..]).ok().unwrap();
}

pub fn decode_transit_msgpack(msgpack_buf: Vec<u8>) -> Option<Message> {
    let cur = Cursor::new(&msgpack_buf[..]);
    let mut deserializer = Deserializer::new(cur);
    Deserialize::deserialize(&mut deserializer).ok()
}

