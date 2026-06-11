use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::path::Path;

const SAVEFILE: &str = "save.json";

use crate::character::Character;

pub struct SaveManager;

impl SaveManager {
    pub fn save(character: &Character) {
        let json = serde_json::to_string_pretty(&character).unwrap();
        let path = Path::new(SAVEFILE);
        let display = path.display();

        // open file in write-only mode
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the json to file
        match file.write_all(json.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Successfully saved to {}", display),
        }
    }

    pub fn load() -> Character {
        let json = read_to_string(SAVEFILE).unwrap();

        serde_json::from_str(&json).unwrap()
    }
}