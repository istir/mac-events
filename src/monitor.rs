use std::process::Command;

use crate::config::Config;

pub struct Monitor {
    blackout: bool,
    input: String, // this can change so it goes into config
    monitor_name: String,
}

impl Monitor {
    pub fn new() -> Self {
        let monitor_name = Config::new().data.monitor_name;
        if monitor_name == "" {
            panic!("No monitor specified!");
        }
        let input = get_display_input(monitor_name.clone());
        return Self {
            blackout: false,
            input,
            monitor_name,
        };
    }
    pub fn turn_off(&mut self) {
        self.blackout = true;
        self.input = "HDMI1".to_string();
        self.set();
    }

    pub fn turn_on(&mut self) {
        self.blackout = false;
        self.input = "DisplayPort1".to_string();
        self.set();
    }

    fn set(&mut self) {
        let blackout = self.blackout;
        let input = self.input.clone();
        self.set_blackout(blackout);
        self.set_display_input(input);
    }

    fn set_display_input(&mut self, value: String) {
        Command::new("lunar")
            .arg("displays")
            .arg(self.monitor_name.clone())
            .arg("input")
            .arg(value)
            .output()
            .expect("Failed to execute command");
    }

    fn set_blackout(&mut self, value: bool) {
        let value_str = value.to_string();
        Command::new("lunar")
            .arg("displays")
            .arg(self.monitor_name.clone())
            .arg("blackout")
            .arg(&value_str)
            .output()
            .expect("Failed to execute command");
    }
}
fn get_display_input(monitor_name: String) -> String {
    let value = Command::new("lunar")
        .arg("displays")
        .arg(monitor_name)
        .arg("input")
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&value.stdout);
    let display_input = output.trim().to_string();
    let input = read_display_input_value(display_input);
    println!("{}", input);
    return input;
}

fn read_display_input_value(display_input: String) -> String {
    let lines: Vec<&str> = display_input.split('\n').collect();
    let value = if lines.len() > 1 {
        lines[1].split(":").nth(1).unwrap_or("").trim().to_string()
    } else {
        String::new()
    };
    return value.replace(" ", "");
}
