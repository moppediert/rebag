pub fn size(data_type: &str) -> u32 {
    match data_type {
        "bool" => 1,
        "int8" => 1,
        "uint8" => 1,
        "int16" => 2,
        "uint16" => 2,
        "int32" => 4,
        "uint32" => 4,
        "int64" => 8,
        "uint64" => 8,
        "float32" => 4,
        "float64" => 8,
        "string" => 64, // https://github.com/ROBOTIS-GIT/ros2xrcedds/blob/fe853ea88cce49d0974976bf857b6664090ef1b2/std_msgs/String.hpp#L33C8-L33C12
        "time" => 4,
        "duration" => 4,
        _ => panic!("Error parsing type: {}", type_def),
    }
}
