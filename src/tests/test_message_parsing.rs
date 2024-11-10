#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{
        message_parser::{match_repeat, parse_message_definition, Repeated},
        tests::sample_messages::{float32::FLOAT32, imu::SENSOR_IMU_MESSAGE},
    };

    #[test]
    fn test_parse_std_message() {
        let msg = FLOAT32;
        let parsed = parse_message_definition(SENSOR_IMU_MESSAGE);
        // println!("{:#?}", parsed);
        assert!(true);
    }

    #[test]
    fn test_match_repeat() {
        let fixtures = [
            ("float64", Some(("float64", Repeated::None))),
            ("float[1]", Some(("float", Repeated::Fixed(1)))),
            ("float[]", Some(("float", Repeated::Fixed(0)))),
            ("float[999]", Some(("float", Repeated::Fixed(999)))),
            ("float[-1]", Some(("float", Repeated::Fixed(0)))),
            ("ns/SomeMessage", Some(("ns/SomeMessage", Repeated::None))),
            (
                "ns_xyz/Some_Message",
                Some(("ns_xyz/Some_Message", Repeated::None)),
            ),
            (
                "ns_xyz/SomeMessage",
                Some(("ns_xyz/SomeMessage", Repeated::None)),
            ),
            (
                "ns/SomeMessage[1]",
                Some(("ns/SomeMessage", Repeated::Fixed(1))),
            ),
            (
                "ns/SomeMessage[]",
                Some(("ns/SomeMessage", Repeated::Fixed(0))),
            ),
            ("ns/Some/Message[]", None),
            ("", None),
            ("1", None),
            ("11[]", None),
            ("11[1]", None),
        ];
        for fixture in fixtures.iter() {
            assert_eq!(match_repeat(fixture.0), fixture.1);
        }
    }
}
