extern crate clap;
extern crate philipshue;
extern crate termion;
extern crate xdg;

mod args;
mod registeruser;
mod settings;

use clap::ArgMatches;
use philipshue::{Bridge, LightCommand};
use settings::Settings;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("lamplighter").unwrap();
    let config_path = xdg_dirs
        .place_config_file("bridge.cnf")
        .expect("Unable to create configuration file");

    match Settings::load_from(&config_path) {
        Ok(settings) => {
            do_command(&settings.into_bridge());
        }
        Err(e) => {
            println!("Couldn't load configuration: {}", e);
            println!("Starting bridge finding process...");
            match registeruser::register_user() {
                Some(bridge) => {
                    println!("Found a bridge");
                    println!("Username {}", bridge.get_username());
                    println!("IP {}", bridge.get_ip());
                    let settings =
                        Settings::new(bridge.get_username().to_owned(), bridge.get_ip().to_owned());
                    match settings.save_to(&config_path) {
                        Ok(_) => {
                            println!("Saved config");
                            do_command(&settings.into_bridge());
                        }
                        Err(e) => {
                            println!("Error saving config {}", e);
                        }
                    }
                }
                None => println!("Didn't find a bridge"),
            }
        }
    }
}

fn do_command(bridge: &Bridge) {
    let matches = args::make_app().get_matches();

    if let Some(matches) = matches.subcommand_matches("on") {
        handle_on(bridge, matches);
    }

    if let Some(matches) = matches.subcommand_matches("off") {
        handle_off(bridge, matches);
    }

    if let Some(matches) = matches.subcommand_matches("dim") {
        handle_dim(bridge, matches);
    }
}

fn handle_on(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    handle_on_lamp(bridge, matches).or_else(|| handle_on_group(bridge, matches))
}

fn handle_off(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    handle_off_lamp(bridge, matches).or_else(|| handle_off_group(bridge, matches))
}

fn handle_dim(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    handle_dim_lamp(bridge, matches)
}

fn handle_on_lamp(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    handle_lamp(bridge, matches, &LightCommand::default().on())
}

fn handle_off_lamp(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    handle_lamp(bridge, matches, &LightCommand::default().off())
}

fn handle_dim_lamp(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    let brightness = matches.value_of("brightness")?;
    let brightness = args::convert_brightness(brightness)?;
    handle_lamp(
        bridge,
        matches,
        &LightCommand::default().on().with_bri(brightness),
    )
}

fn handle_lamp(bridge: &Bridge, matches: &ArgMatches<'_>, command: &LightCommand) -> Option<()> {
    let lamp = matches.value_of("lamp")?;
    let lamp_id = identify_light_by_name(bridge, lamp)?;
    bridge.set_light_state(lamp_id, command).map(|_| ()).ok()
}

fn handle_on_group(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    let group = matches.value_of("group")?;
    println!("Turn on all in {}", group);
    Some(())
}

fn handle_off_group(bridge: &Bridge, matches: &ArgMatches<'_>) -> Option<()> {
    let group = matches.value_of("group")?;
    println!("Turn off all in {}", group);
    Some(())
}

fn identify_light_by_name(bridge: &Bridge, name: &str) -> Option<usize> {
    match bridge.get_all_lights() {
        Err(e) => eprintln!("Error getting lights {}", e),
        Ok(lights) => {
            for (id, light) in lights {
                if light.name.to_lowercase() == name.to_lowercase() {
                    return Some(id);
                }
            }
        }
    }
    None
}
