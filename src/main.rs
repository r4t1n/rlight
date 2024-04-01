use glob::glob;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let brightness: &String = &args[1];

    let backlight_devices: Vec<String> = get_backlight_devices();
    let backlight_brightness_path: String = format!("{}/{}", backlight_devices[0], "brightness");

    let _ = fs::write(backlight_brightness_path, brightness);
}

fn get_backlight_devices() -> Vec<String> {
    let sys_backlight_path: &str = "/sys/class/backlight/*";
    let mut backlight_devices: Vec<String> = Vec::new();

    for entry in glob(sys_backlight_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(backlight_device) => {
                backlight_devices.push(backlight_device.into_os_string().into_string().unwrap())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    backlight_devices
}
