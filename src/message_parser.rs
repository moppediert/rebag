use std::collections::BTreeMap;

use regex::Regex;

#[derive(Debug)]
struct Field {
    field_name: String,
    field_type: String,
    field_repeat: u32,
}

const MESSAGE_SEPARATOR: &str =
    "================================================================================";
type FieldName<'a> = &'a str;
type FieldType<'a> = &'a str;

/// returns 0 for variable-length array
/// returns n for array of length n
/// returns 1 otherwise
pub fn match_repeat(field_def: &str) -> Option<(&str, u32)> {
    let re = Regex::new(
        r"^(?<type>[a-zA-Z]+(?:\/)?[a-zA-Z]+)(?<repeat_group>\[(?<repeat>[-]?[0-9]*)\])?$",
    )
    .unwrap();
    match re.captures(field_def) {
        Some(matched) => {
            println!("matched? {:#?}", matched);
            match matched.name("type") {
                Some(field_type) => match matched.name("repeat_group") {
                    Some(_) => match matched.name("repeat") {
                        Some(repeat) => match str::parse(repeat.as_str()) {
                            Ok(repeat) => Some((field_type.as_str(), repeat)),
                            Err(_) => Some((field_type.as_str(), 0)),
                        },
                        None => {
                            // println!("---- {:#?}", field_type.as_str());
                            Some((field_type.as_str(), 0))
                        }
                    },
                    None => Some((field_type.as_str(), 1)),
                },
                None => None,
            }
        }
        None => None,
    }
}

pub fn parse_message_definition(definition: &str) {
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
                type_def.insert(field_type.unwrap(), BTreeMap::new());
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
                        if let Some((sub_field_type, repeat)) = match_repeat(sub_field_type) {
                            let sub_field = Field {
                                field_name: sub_field_name.to_string(),
                                field_type: sub_field_type.to_string(),
                                field_repeat: repeat,
                            };
                            type_def.entry(ft).and_modify(|fields| {
                                fields.insert(sub_field_name, sub_field);
                            });
                        }
                    }
                    None => panic!(
                        "Message type has to be the first line in a section, beginning with MSG"
                    ),
                },
                None => panic!("Invalid message definition line: {}", raw_line),
            }
        }
    }
    println!("fields: {:#?}", fields);
    println!("def: {:#?}", type_def);
}
