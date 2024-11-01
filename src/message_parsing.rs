use byteorder::{ByteOrder, LE};
use core::str;
use std::collections::BTreeMap;

const MESSAGE_SEPARATOR: &str =
    "================================================================================\n";

pub fn parse_message(definition: &str) -> &str {
    match definition {
        "float32 data" => "float32",
        _ => todo!(),
    }
}

type FieldName<'a> = &'a str;
type FieldType<'a> = &'a str;

pub fn parse_message_definition(
    definition: &str,
) -> BTreeMap<FieldName, BTreeMap<FieldName, FieldType>> {
    let mut type_map = BTreeMap::new();

    let sections = definition.split(MESSAGE_SEPARATOR);

    for (index, section) in sections.enumerate() {
        let mut field_type = None;
        if index == 0 {
            field_type = Some("main");
            type_map.insert(field_type.unwrap(), BTreeMap::new());
        }

        for line in section.split('\n') {
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            if line.starts_with("MSG: ") {
                field_type = Some(line.trim_start_matches("MSG: "));
                type_map.insert(field_type.unwrap(), BTreeMap::new());
                continue;
            }

            // Discard inline comments
            let raw_line = match line.split_once('#') {
                Some((raw, _comment)) => raw.trim(),
                None => line.trim(),
            };

            match raw_line.split_once(' ') {
                Some((sub_field_type, sub_field_name)) => match field_type {
                    Some(ft) => {
                        type_map.entry(ft).and_modify(|fields| {
                            fields.insert(sub_field_name, sub_field_type);
                        });
                    }
                    None => panic!(
                        "message type has to be the first line in a section, beginning with MSG"
                    ),
                },
                None => panic!("Invalid message definition line: {}", raw_line),
            }
        }
    }
    type_map
}

fn is_primitive_type(type_definition: &str) -> bool {
    let primitive_types = [
        "bool", "int8", "uint8", "int16", "uint16", "int32", "uint32", "int64", "uint64",
        "float32", "float64", "time", "duration",
    ];
    primitive_types.contains(&type_definition)
}

fn parse_primivate_data<'a>(type_definition: &'a str, data: &'a [u8]) -> &'a str {
    let parsed_data = match type_definition {
        "bool" => format!("{}", parse_bool(&data[..1]).unwrap()),
        "int8" => format!("{}", parse_int8(&data[..1]).unwrap()),
        "uint8" => format!("{}", parse_uint8(&data[..1]).unwrap()),
        "int16" => format!("{}", parse_int16(&data[..2]).unwrap()),
        "uint16" => format!("{}", parse_uint16(&data[..2]).unwrap()),
        "int32" => format!("{}", parse_uint16(&data[..4]).unwrap()),
        "uint32" => format!("{}", parse_uint32(&data[..4]).unwrap()),
        "int64" => format!("{}", parse_int64(&data[..8]).unwrap()),
        "uint64" => format!("{}", parse_uint64(&data[..8]).unwrap()),
        "float32" => format!("{}", parse_float32(&data[..8]).unwrap()),
        "float64" => format!("{}", parse_float64(&data[..8]).unwrap()),
        "time" => format!("{}", parse_float64(&data[..4]).unwrap()),
        "duration" => format!("{}", parse_float64(&data[..4]).unwrap()),
        _ => "".to_string(),
    };
    // &var_name
    ""
}

#[derive(Debug)]
pub enum Error {
    MessageParsingError(String),
}

fn check_len(data_type: &str, data: &[u8], len: usize) -> Result<(), Error> {
    if data.len() != len {
        Err(Error::MessageParsingError(format!(
            "Cannot parse message of type: {}",
            data_type
        )))
    } else {
        Ok(())
    }
}

fn parse_bool(data: &[u8]) -> Result<bool, Error> {
    check_len("bool", data, 1).map(|_| data[0] != 0)
}

fn parse_int8(data: &[u8]) -> Result<i8, Error> {
    check_len("int8", data, 1).map(|_| i8::from_le_bytes(data.try_into().unwrap()))
}

fn parse_uint8(data: &[u8]) -> Result<u8, Error> {
    check_len("uint8", data, 1).map(|_| data[0])
}

fn parse_int16(data: &[u8]) -> Result<i16, Error> {
    check_len("int16", data, 2).map(|_| LE::read_i16(data))
}

fn parse_uint16(data: &[u8]) -> Result<u16, Error> {
    check_len("uint16", data, 2).map(|_| LE::read_u16(data))
}

fn parse_int32(data: &[u8]) -> Result<i32, Error> {
    check_len("int32", data, 4).map(|_| LE::read_i32(data))
}

fn parse_uint32(data: &[u8]) -> Result<u32, Error> {
    check_len("uint32", data, 4).map(|_| LE::read_u32(data))
}

fn parse_int64(data: &[u8]) -> Result<i64, Error> {
    check_len("int64", data, 8).map(|_| LE::read_i64(data))
}

fn parse_uint64(data: &[u8]) -> Result<u64, Error> {
    check_len("uint64", data, 8).map(|_| LE::read_u64(data))
}

fn parse_float32(data: &[u8]) -> Result<f32, Error> {
    check_len("float32", data, 4).map(|_| LE::read_f32(data))
}

fn parse_float64(data: &[u8]) -> Result<f64, Error> {
    check_len("float64", data, 8).map(|_| LE::read_f64(data))
}

fn parse_time(data: &[u8]) -> Result<u32, Error> {
    check_len("time", data, 4).map(|_| LE::read_u32(data))
}

fn parse_duration(data: &[u8]) -> Result<i32, Error> {
    check_len("duration", data, 4).map(|_| LE::read_i32(data))
}

fn parse_str(data: &[u8]) -> Result<String, Error> {
    match str::from_utf8(data) {
        Ok(parsed) => Ok(parsed.to_string()),
        Err(e) => Err(Error::MessageParsingError(format!(
            "Cannot parse message of type string: {}",
            e
        ))),
    }
}
