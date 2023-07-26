use ini::Ini;

pub struct Config {
    location: String,
    data: Ini,
}

impl Config {
    pub fn load(location: &str) -> Config {
        let mut config = Ini::new();

        // If parsing works, then load it
        if let Ok(loaded) = Ini::load_from_file(location) {
            config = loaded;
        }

        Config {
            location: location.to_string(),
            data: config,
        }
    }

    pub fn save(&self) {
        self.data
            .write_to_file(&self.location)
            .expect("Could not save config file.");
    }

    pub fn get_user_pass(&mut self) -> Option<(String, String)> {
        let username;
        let password;

        if let Some(x) = self.data.with_section(Some("user")).get("username") {
            username = x.to_string();
        } else {
            return None;
        }

        if let Some(x) = self.data.with_section(Some("user")).get("password") {
            password = x.to_string();
        } else {
            return None;
        }

        Some((username, password))
    }

    pub fn set_user_pass(&mut self, username: &str, password: &str) {
        self.data
            .with_section(Some("user"))
            .set("username", username)
            .set("password", password);
    }
}
