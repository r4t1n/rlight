use std::env;
use std::fs;
use std::path;
use std::process;

pub fn get_backlight_brightness(
    backlight_brightness_path: String,
    backlight_device: String,
) -> (u32, u32) {
    let backlight_max_brightness_path: String = format!("{}/max_brightness", backlight_device);

    let backlight_brightness: u32 = fs::read_to_string(backlight_brightness_path.clone())
        .unwrap()
        .trim_end()
        .parse()
        .unwrap();

    let backlight_max_brightness: u32 = fs::read_to_string(backlight_max_brightness_path.clone())
        .unwrap()
        .trim_end()
        .parse()
        .unwrap();

    (backlight_brightness, backlight_max_brightness)
}

pub fn get_backlight_default_device(backlight_default_device_path: String) -> String {
    let mut backlight_default_device: String = String::new();
    if path::Path::new(&backlight_default_device_path).exists() {
        backlight_default_device = fs::read_to_string(backlight_default_device_path)
            .unwrap()
            .trim_end()
            .to_string();
    } else {
        match fs::File::create(backlight_default_device_path.clone()) {
            Ok(t) => t,
            Err(err) => panic!(
                "Error: failed to create config file at '{}': {err}",
                backlight_default_device_path
            ),
        };
    };

    backlight_default_device
}

pub fn get_backlight_default_device_path() -> String {
    let mut user_home_dir: String = String::new();

    // Using env::home_dir() is fine since this is only for Unix based systems ;3
    #[allow(deprecated)]
    match env::home_dir() {
        Some(home_dir) => {
            user_home_dir = home_dir
                .into_os_string()
                .into_string()
                .expect("Error: could not convert PathBuf to String")
        }
        None => eprintln!("Error: could not get the user home directory"),
    };

    let backlight_default_device_path: String = format!("{}/.config/rlight", user_home_dir);

    backlight_default_device_path
}

pub fn get_backlight_devices() -> Vec<String> {
    let sys_backlight_path: &str = "/sys/class/backlight";
    let backlight_dir = fs::read_dir(sys_backlight_path).unwrap();
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
    };

    backlight_devices
}

pub fn list_backlight_device_names(backlight_devices: Vec<String>) {
    let mut backlight_device_names: Vec<String> = Vec::new();
    let mut index: u8 = 1;
    for backlight_device in backlight_devices.iter() {
        backlight_device_names.push(format!(
            "[{}] {}",
            index,
            backlight_device.replace("/sys/class/backlight/", "")
        ));

        index += 1;
    }

    for backlight_device_name in backlight_device_names.iter() {
        println!("{}", backlight_device_name)
    }
}

pub fn write_backlight_brightness(backlight_brightness_path: String, brightness: u32) {
    match fs::write(backlight_brightness_path.clone(), brightness.to_string()) {
        Ok(t) => t,
        Err(err) => eprintln!(
            "Error writing to backlight device at '{backlight_brightness_path}': {err}\nHave you added yourself to the 'video' group?"
        ),
    };
}
