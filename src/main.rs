use clap::Parser;
use std::env;
use std::fs;
use std::path;
use std::process;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "")]
    brightness: String,

    #[clap(short, long)]
    list: bool,
}

fn main() {
    let args: Args = Args::parse();
    let backlight_devices: Vec<String> = get_backlight_devices();

    if args.list {
        list_backlight_device_names(backlight_devices.clone());
    };

    let backlight_default_device: String = get_backlight_default_device();
    let backlight_device: String = if backlight_default_device.is_empty() {
        backlight_devices[0].to_string()
    } else {
        backlight_default_device
    };

    let backlight_brightness_path: String = format!("{}/brightness", backlight_device);
    let (backlight_brightness, backlight_max_brightness): (u32, u32) =
        get_backlight_brightness(backlight_brightness_path.clone(), backlight_device);

    if args.brightness.is_empty() {
        let brightness: f32 =
            (backlight_brightness as f32 / backlight_max_brightness as f32 * 100.0).round();
        println!("{}%", brightness);
        process::exit(0);
    };

    let operation: char = if args.brightness.starts_with('+') {
        '+'
    } else if args.brightness.starts_with('_') {
        '-'
    } else {
        ' '
    };

    let user_brightness: u8 = args
        .brightness
        .replace(['%', '+', '_'], "")
        .parse()
        .unwrap();

    if user_brightness > 100 {
        eprintln!("Error: can not set brightness to over 100%");
        process::exit(1);
    };

    let mut absolute_brightness: u32 = match operation {
        '+' => backlight_brightness + backlight_max_brightness * user_brightness as u32 / 100,
        '-' => backlight_brightness
            .saturating_sub(backlight_max_brightness * user_brightness as u32 / 100),
        _ => backlight_max_brightness * user_brightness as u32 / 100,
    };

    if absolute_brightness > backlight_max_brightness {
        absolute_brightness = backlight_max_brightness;
    };

    write_backlight_brightness(backlight_brightness_path, absolute_brightness);
}

fn get_backlight_devices() -> Vec<String> {
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

fn list_backlight_device_names(backlight_devices: Vec<String>) {
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

    process::exit(0);
}

fn get_backlight_default_device() -> String {
    let mut user_home_dir: String = String::new();
    match env::home_dir() {
        Some(home_dir) => {
            user_home_dir = home_dir
                .into_os_string()
                .into_string()
                .expect("Error: could not convert PathBuf to String")
        }
        None => eprintln!("Error: could not get user home directory"),
    };

    let backlight_default_device_path: String = format!("{}/.config/rlight", user_home_dir);
    let mut backlight_default_device: String = String::new();
    if path::Path::new(&backlight_default_device_path).exists() {
        backlight_default_device = fs::read_to_string(backlight_default_device_path)
            .unwrap()
            .trim_end()
            .to_string();
    };

    backlight_default_device
}

fn get_backlight_brightness(
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

fn write_backlight_brightness(backlight_brightness_path: String, brightness: u32) {
    match fs::write(backlight_brightness_path.clone(), brightness.to_string()) {
        Ok(t) => t,
        Err(e) => eprintln!(
            "Error writing to backlight device at '{backlight_brightness_path}': {e}\nHave you added yourself to the 'video' group?"
        ),
    };
}
