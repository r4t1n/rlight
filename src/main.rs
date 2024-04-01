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
    let sys_backlight_path = fs::read_dir("/sys/class/backlight").unwrap();
    let mut backlight_devices: Vec<String> = Vec::new();

    for backlight_device in sys_backlight_path {
        backlight_devices.push(
            backlight_device
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
    }

    backlight_devices
}
