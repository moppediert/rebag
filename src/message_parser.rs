use std::collections::BTreeMap;

use regex::Regex;

const MESSAGE_SEPARATOR: &str =
    "================================================================================";

#[derive(Debug)]
pub struct Field {
    field_name: String,
    field_type: String,
    field_repeat: Repeated,
}
#[derive(Debug, PartialEq)]
pub enum Repeated {
    None,
    Fixed(u32),
    Variable,
}

pub fn match_repeat(field_def: &str) -> Option<(&str, Repeated)> {
    let re = Regex::new(
        r"^(?<type>[a-zA-Z]+[a-zA-Z0-9_]*(?:\/)?[a-zA-Z]+[a-zA-Z0-9_]*)(?<repeat_group>\[(?<repeat>[-]?[0-9]*)\])?$",
    )
    .unwrap();
    match re.captures(field_def) {
        Some(matched) => {
            match matched.name("type") {
                Some(field_type) => match matched.name("repeat_group") {
                    Some(_) => match matched.name("repeat") {
                        Some(repeat) => match str::parse::<u32>(repeat.as_str()) {
                            Ok(repeat) => Some((field_type.as_str(), Repeated::Fixed(repeat))), // float[1]
                            Err(_) => Some((field_type.as_str(), Repeated::Fixed(0))), // float[-1]
                        },
                        None => Some((field_type.as_str(), Repeated::Variable)), // float[]
                    },
                    None => Some((field_type.as_str(), Repeated::None)), // float
                },
                None => None,
            }
        }
        None => None,
    }
}

pub fn parse_message_definition(definition: &str) -> (Vec<Field>, BTreeMap<String, Vec<Field>>) {
    let mut fields: Vec<Field> = vec![];
    let mut type_def = BTreeMap::new();

    let mut sections = definition.split(MESSAGE_SEPARATOR);

    // At least one section is always present in a message definition
    let main_section = sections.next().unwrap();
    for line in main_section.split('\n') {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        // Discard inline comments
        let raw_line = match line.split_once('#') {
            Some((raw, _comment)) => raw.trim(),
            None => line.trim(),
        };

        match raw_line.split_once(' ') {
            Some((field_type, field_name)) => {
                if let Some((field_type, repeat)) = match_repeat(field_type) {
                    fields.push(Field {
                        field_name: field_name.to_string(),
                        field_type: field_type.to_string(),
                        field_repeat: repeat,
                    })
                } else {
                    panic!("Cannot parse message")
                }
            }
            None => panic!("Invalid message definition line: {}", raw_line),
        }
    }

    for section in sections {
        let mut field_type = None;

        for line in section.split('\n') {
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            if line.starts_with("MSG: ") {
                field_type = Some(line.trim_start_matches("MSG: "));
                type_def.insert(field_type.unwrap().to_string(), vec![]);
                continue;
            }

            // Discard inline comments
            let raw_line = match line.split_once('#') {
                Some((raw, _comment)) => raw.trim(),
                None => line.trim(),
            };

            match raw_line.split_once(' ') {
                Some((sub_field_type, sub_field_name)) => match field_type {
                    Some(ft) => match match_repeat(sub_field_type) {
                        Some((sub_field_type, repeat)) => {
                            let sub_field = Field {
                                field_name: sub_field_name.to_string(),
                                field_type: sub_field_type.to_string(),
                                field_repeat: repeat,
                            };
                            type_def.entry(ft.to_string()).and_modify(|fields| {
                                fields.push(sub_field);
                            });
                        }
                        None => panic!("Cannot parse message"),
                    },
                    None => panic!(
                        "Message type has to be the first line in a section, beginning with MSG"
                    ),
                },
                None => panic!("Invalid message definition line: {}", raw_line),
            }
        }
    }

    (fields, type_def)
}
