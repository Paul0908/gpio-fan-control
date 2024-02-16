use std::{fs, io, thread};
use std::time::Duration;
use gpio::GpioOut;
use gpio::sysfs::SysFsGpioOutput;
use log::{error, info};
use simple_logger::SimpleLogger;

const TEMP_FILE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";
const GPIO_PIN: u16 = 73;
const CHECK_INTERVAL: Duration = Duration::from_secs(5);
const TEMP_THRESHOLD: f32 = 50.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    let mut gpio_fan_pin = SysFsGpioOutput::open(GPIO_PIN).unwrap();
    info!("fan control initialized");
    loop {
        match read_temp(TEMP_FILE_PATH) {
            Ok(temp) if temp >= TEMP_THRESHOLD => {
                if let Err(e) = gpio_fan_pin.set_high() {
                    error!("Failed to set GPIO pin high: {} at temperature: {}", e, temp);
                }
            }
            Ok(temp) => {
                if let Err(e) = gpio_fan_pin.set_low() {
                    error!("Failed to set GPIO pin low: {} at temperature: {}", e, temp);
                }
            }
            Err(e) => {
                error!("Failed to read temperature: {}", e);
                thread::sleep(CHECK_INTERVAL);
                continue;
            }
        }
        thread::sleep(CHECK_INTERVAL);
    }
}

fn read_temp(file_path: &str) -> Result<f32, io::Error> {
    let content = fs::read_to_string(file_path)?;
    content.trim().parse::<f32>()
        .map(|temp_milli| temp_milli / 1000.0)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Could not parse the temperature"))
}
