use anyhow::Result;
use byteorder::{ByteOrder, LE};
use core::{fmt, str};
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

fn parse_primivate_data(type_definition: &str, data: &[u8]) -> Result<String, MessageParsingError> {
    // https://wiki.ros.org/msg
    match type_definition {
        "bool" => Ok(bool::parse(data)?.to_string()),
        "int8" => Ok(i8::parse(data)?.to_string()),
        "uint8" => Ok(u8::parse(data)?.to_string()),
        "int16" => Ok(i16::parse(data)?.to_string()),
        "uint16" => Ok(u16::parse(data)?.to_string()),
        "int32" => Ok(i32::parse(data)?.to_string()),
        "uint32" => Ok(u32::parse(data)?.to_string()),
        "int64" => Ok(i64::parse(data)?.to_string()),
        "uint64" => Ok(u64::parse(data)?.to_string()),
        "float32" => Ok(f32::parse(data)?.to_string()),
        "float64" => Ok(f64::parse(data)?.to_string()),
        "string" => Ok(String::parse(data)?.to_string()),
        "time" => Ok(u32::parse(data)?.to_string()),
        "duration" => Ok(i32::parse(data)?.to_string()),
        _ => panic!("Invalid primitive type: {}", type_definition),
    }
}

#[derive(Debug)]
struct MessageParsingError;

impl fmt::Display for MessageParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing message definition")
    }
}

trait PrimitiveParser {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError>
    where
        Self: std::marker::Sized;
}

impl PrimitiveParser for bool {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 1);
        Ok(data[0] != 0)
    }
}

impl PrimitiveParser for i8 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 1);
        Ok(i8::from_le_bytes([data[0]]))
    }
}

impl PrimitiveParser for u8 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 1);
        Ok(data[0])
    }
}

impl PrimitiveParser for i16 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 2);
        Ok(LE::read_i16(data))
    }
}

impl PrimitiveParser for u16 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 2);
        Ok(LE::read_u16(data))
    }
}

impl PrimitiveParser for i32 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 4);
        Ok(LE::read_i32(data))
    }
}
impl PrimitiveParser for u32 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 4);
        Ok(LE::read_u32(data))
    }
}
impl PrimitiveParser for i64 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 8);
        Ok(LE::read_i64(data))
    }
}
impl PrimitiveParser for u64 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 8);
        Ok(LE::read_u64(data))
    }
}
impl PrimitiveParser for f32 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 4);
        Ok(LE::read_f32(data))
    }
}
impl PrimitiveParser for f64 {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        assert!(data.len() == 8);
        Ok(LE::read_f64(data))
    }
}
impl PrimitiveParser for String {
    fn parse(data: &[u8]) -> Result<Self, MessageParsingError> {
        match String::from_utf8(data.to_vec()) {
            Ok(result) => Ok(result),
            Err(_) => Err(MessageParsingError),
        }
    }
}
