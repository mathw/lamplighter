use philipshue::bridge::{self, Bridge};
use philipshue::errors::{BridgeError, HueError, HueErrorKind};
use std::thread::sleep;
use std::time::Duration;

pub fn register_user() -> Option<Bridge> {
    let mut bridge = None;
    // Discover a bridge
    let bridge_ip = bridge::discover().unwrap().pop().unwrap().into_ip();
    let devicetype = "my_hue_app#homepc";

    // Keep trying to register a user
    loop {
        match bridge::register_user(&bridge_ip, devicetype) {
            // A new user has succesfully been registered and the username is returned
            Ok(username) => {
                bridge = Some(Bridge::new(bridge_ip, username));
                break;
            }
            // Prompt the user to press the link button
            Err(HueError(
                HueErrorKind::BridgeError {
                    error: BridgeError::LinkButtonNotPressed,
                    ..
                },
                _,
            )) => {
                println!("Please, press the link on the bridge. Retrying in 5 seconds");
                sleep(Duration::from_secs(5));
            }
            // Some other error happened
            Err(e) => {
                println!("Unexpected error occured: {:?}", e);
                break;
            }
        }
    }
    bridge
}
