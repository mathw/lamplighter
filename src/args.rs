use clap::{App, AppSettings, Arg, SubCommand};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Lamplighter")
        .version("0.1")
        .author("Matthew Walton")
        .about("Controls Philips Hue lights from the command line")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("on")
                .about("turns on a light or room")
                .arg(
                    Arg::with_name("lamp")
                        .value_name("LAMP")
                        .help("lamp to turn on")
                        .conflicts_with("group")
                        .required_unless("group"),
                ).arg(
                    Arg::with_name("group")
                        .short("g")
                        .long("group")
                        .takes_value(true)
                        .help("group to turn on all lamps within")
                        .conflicts_with("lamp")
                        .required_unless("lamp"),
                ),
        ).subcommand(
            SubCommand::with_name("off")
                .about("turns off a light or room")
                .arg(
                    Arg::with_name("lamp")
                        .value_name("LAMP")
                        .help("lamp to turn off")
                        .conflicts_with("group")
                        .required_unless("group"),
                ).arg(
                    Arg::with_name("group")
                        .short("g")
                        .long("group")
                        .takes_value(true)
                        .help("group to turn off all lamps within")
                        .conflicts_with("lamp")
                        .required_unless("lamp"),
                ),
        ).subcommand(
            SubCommand::with_name("dim")
                .about("change brightness")
                .arg(
                    Arg::with_name("lamp")
                        .help("lamp to adjust brightness of")
                        .takes_value(true)
                        .required(true),
                ).arg(
                    Arg::with_name("brightness")
                        .help("brightness to set (0-255 or 0-100%)")
                        .takes_value(true)
                        .required(true)
                        .validator(validate_brightness),
                ),
        )
}

fn validate_brightness(s: String) -> Result<(), String> {
    if let Some('%') = s.chars().last() {
        parse_percentage(&s)
            .map(|_| ())
            .map_err(|_| "Brightness percentage must be 0-100%".to_owned())
    } else {
        u8::from_str(&s)
            .map(|_| ())
            .map_err(|_| "Brightness must be 0-255".to_owned())
    }
}

pub fn convert_brightness(s: &str) -> Option<u8> {
    if is_percentage(s) {
        parse_percentage(s)
            .map(|p| (255.0 * (p as f32 / 100.0)) as u8)
            .ok()
    } else {
        u8::from_str(s).ok()
    }
}

fn is_percentage(s: &str) -> bool {
    match s.chars().last() {
        Some('%') => true,
        _ => false,
    }
}

fn parse_percentage(s: &str) -> Result<u8, ParseIntError> {
    u8::from_str(&s[0..s.len() - 1])
}
