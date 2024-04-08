use colored::ColoredString;
use colored::Colorize;
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
                "Error: failed to create config file at '{}': {}",
                backlight_default_device_path, err
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
        );
    }

    if backlight_devices.is_empty() {
        eprintln!("Error: no backlight devices found in '{sys_backlight_path}'");
        process::exit(1);
    };

    backlight_devices
}

pub fn list_backlight_device_names(
    backlight_devices: Vec<String>,
    backlight_default_device: String,
) {
    let mut backlight_device_list: Vec<String> = Vec::new();
    let mut index: u8 = 1;
    let index_text: ColoredString = format!("[{}]", index).white().bold(); // Needs to be set to white before bold, else it becomes blue for some reason
    let mut is_backlight_default_device: ColoredString = "".normal();

    for backlight_device in backlight_devices.iter() {
        if *backlight_device == backlight_default_device {
            is_backlight_default_device = "*".cyan().blink();
        };

        backlight_device_list.push(format!(
            "{} {} {}",
            index_text,
            backlight_device.replace("/sys/class/backlight/", ""),
            is_backlight_default_device
        ));

        index += 1;
    }

    for backlight_device_name in backlight_device_list.iter() {
        println!("{}", backlight_device_name)
    }
}

pub fn set_backlight_default_device(
    backlight_default_device_path: String,
    backlight_device: String,
) {
    let backlight_device_name: ColoredString = format!(
        "[{}]",
        backlight_device.replace("/sys/class/backlight/", "")
    )
    .white()
    .bold();
    println!(
        "{}: setting as the default backlight device",
        backlight_device_name
    );

    match fs::write(backlight_default_device_path.clone(), backlight_device) {
        Ok(t) => t,
        Err(err) => eprintln!(
            "Error writing to default backlight device file at '{}': {}",
            backlight_default_device_path, err
        ),
    };
}

pub fn write_backlight_brightness(backlight_brightness_path: String, brightness: u32) {
    match fs::write(backlight_brightness_path.clone(), brightness.to_string()) {
        Ok(t) => t,
        Err(err) => eprintln!(
            "Error writing to backlight device at '{}': {}\nHave you added yourself to the 'video' group?", backlight_brightness_path, err
        ),
    };
}
