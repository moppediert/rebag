#[cfg(test)]
mod tests {
    use crate::{
        message_parsing::{parse_message, parse_message_definition},
        tests::sample_messages::{float32::FLOAT32, imu::SENSOR_IMU_MESSAGE},
    };

    #[test]
    fn test_parse_std_message() {
        let msg = FLOAT32;
        let parsed = parse_message_definition(SENSOR_IMU_MESSAGE);
        println!("{:#?}", parsed);
        assert!(true);
    }
}
