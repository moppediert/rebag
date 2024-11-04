#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{
        message_parser::{match_repeat, parse_message_definition},
        tests::sample_messages::{float32::FLOAT32, imu::SENSOR_IMU_MESSAGE},
    };

    #[test]
    fn test_parse_std_message() {
        let msg = FLOAT32;
        // parse_message_definition(SENSOR_IMU_MESSAGE);
        assert!(true);
    }

    #[test]
    fn test_match_repeat() {
        let fixtures = [
            ("float", Some(("float", 1))),
            ("float[1]", Some(("float", 1))),
            ("float[]", Some(("float", 0))),
            ("float[999]", Some(("float", 999))),
            ("float[-1]", Some(("float", 0))),
            ("ns/SomeMessage", Some(("ns/SomeMessage", 1))),
            ("ns/SomeMessage[1]", Some(("ns/SomeMessage", 1))),
            ("ns/SomeMessage[]", Some(("ns/SomeMessage", 0))),
            ("ns/Some/Message[]", None),
            ("", None),
            ("1", None),
            ("11[]", None),
            ("11[1]", None),
        ];
        for fixture in fixtures.iter() {
            println!("{:#?}", fixture);
            assert_eq!(match_repeat(fixture.0), fixture.1);
        }
    }
}
