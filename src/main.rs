use clap::Parser;
use std::process;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "")]
    brightness: String,

    #[clap(short, long, default_value_t = 0)]
    device: u8,

    #[clap(short, long)]
    list: bool,

    #[clap(short, long, default_value_t = 0)]
    set: u8,
}

fn main() {
    let args: Args = Args::parse();

    if args.list {
        rlight::list_backlight_device_names(rlight::get_backlight_devices());
        process::exit(0);
    };

    let backlight_default_device_path: String = rlight::get_backlight_default_device_path();

    if args.set != 0 {
        let backlight_devices: Vec<String> = rlight::get_backlight_devices();
        println!(
            "[{}]: setting as the default backlight device",
            backlight_devices[(args.set - 1) as usize].replace("/sys/class/backlight/", "")
        );
        rlight::set_backlight_default_device(
            backlight_default_device_path.clone(),
            backlight_devices[(args.set - 1) as usize].to_owned(),
        );
        process::exit(0);
    };

    let backlight_device: String;

    if args.device != 0 {
        let backlight_devices: Vec<String> = rlight::get_backlight_devices();
        backlight_device = backlight_devices[(args.device - 1) as usize].to_owned();
    } else {
        let backlight_default_device: String =
            rlight::get_backlight_default_device(backlight_default_device_path.clone());

        if backlight_default_device.is_empty() {
            let backlight_devices: Vec<String> = rlight::get_backlight_devices();
            backlight_device = backlight_devices[0].to_owned();
            rlight::set_backlight_default_device(
                backlight_default_device_path,
                backlight_devices[0].to_owned(),
            );
        } else {
            backlight_device = backlight_default_device;
        }
    }

    let backlight_brightness_path: String = format!("{}/brightness", backlight_device);
    let (backlight_brightness, backlight_max_brightness): (u32, u32) =
        rlight::get_backlight_brightness(backlight_brightness_path.clone(), backlight_device);

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

    rlight::write_backlight_brightness(backlight_brightness_path, absolute_brightness);
}
