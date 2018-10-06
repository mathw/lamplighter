use philipshue::Bridge;
use std::fs;
use std::io;
use std::path::Path;

pub struct Settings {
    username: String,
    bridge_ip: String,
}

impl Settings {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn bridge_ip(&self) -> &str {
        &self.bridge_ip
    }

    pub fn load_from(filename: impl AsRef<Path>) -> io::Result<Settings> {
        let content = fs::read_to_string(filename)?;
        let lines = content.lines().collect::<Vec<_>>();
        if lines.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "config file format invalid",
            ));
        }
        let username = lines[0];
        let ip = lines[1];
        Ok(Settings {
            username: username.to_owned(),
            bridge_ip: ip.to_owned(),
        })
    }

    pub fn save_to(&self, filename: impl AsRef<Path>) -> io::Result<()> {
        let contents = format!("{}\n{}", self.username, self.bridge_ip);
        fs::write(filename, &contents)
    }

    pub fn new(username: String, bridge_ip: String) -> Settings {
        Settings {
            username: username,
            bridge_ip: bridge_ip,
        }
    }

    pub fn into_bridge(&self) -> Bridge {
        Bridge::new(self.bridge_ip(), self.username())
    }
}
