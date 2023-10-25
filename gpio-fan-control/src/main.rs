use std::fs;


fn main() {
    const TEMP_FILE_NAME: &str = "/sys/class/thermal/thermal_zone0/temp";
    const GPIO_PIN_FAN: i8 = 2;

    let content = fs::read_to_string(TEMP_FILE_NAME)
        .expect("Unable to read temp from file");

    println!("File contents: {}, {}", content, GPIO_PIN_FAN)
}