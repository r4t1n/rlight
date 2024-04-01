use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: one argument not provided");
        process::exit(1);
    }

    let user_brightness: &String = &args[1];
    let user_brightness: u8 = match user_brightness.replace('%', "").parse::<u8>() {
        Ok(t) => t,
        Err(e) => panic!("Error parsing user brightness: {e}"),
    };

    if user_brightness > 100 {
        eprintln!("Error: can not set brightness to over 100%");
        process::exit(1);
    }

    let backlight_devices: Vec<String> = get_backlight_devices();
    let backlight_brightness_path: String = format!("{}/brightness", backlight_devices[0]);
    let backlight_max_brightness_path: String = format!("{}/max_brightness", backlight_devices[0]);

    let backlight_max_brightness: u32 = fs::read_to_string(backlight_max_brightness_path.clone())
        .unwrap()
        .trim_end()
        .parse()
        .unwrap();

    let brightness: u32 = backlight_max_brightness * user_brightness as u32 / 100;
    write_brightness(backlight_brightness_path, brightness)
}

fn get_backlight_devices() -> Vec<String> {
    let sys_backlight_path: &str = "/sys/class/backlight";
    let backlight_dir = fs::read_dir("/sys/class/backlight").unwrap();
    let mut backlight_devices: Vec<String> = Vec::new();

    for backlight_device in backlight_dir {
        backlight_devices.push(
            backlight_device
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
    }

    if backlight_devices.is_empty() {
        eprintln!("Error: no backlight devices found in '{sys_backlight_path}'");
        process::exit(1);
    }

    backlight_devices
}

fn write_brightness(backlight_brightness_path: String, brightness: u32) {
    match fs::write(backlight_brightness_path.clone(), brightness.to_string()) {
        Ok(t) => t,
        Err(e) => eprintln!(
            "Error writing to backlight device at '{backlight_brightness_path}': {e}\nHave you added yourself to the 'video' group?"
        ),
    }
}
