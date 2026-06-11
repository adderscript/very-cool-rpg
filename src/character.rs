use std::io::{self, Write};
use serde::{Serialize, Deserialize};

use crate::utils::prompt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    Solier,
    Archer,
    Wizard,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    class: Class,
    coins: u32,
}

impl Character {
    pub fn new(class: Class) -> Self {
        Self {
            class: class,
            coins: 0,
        }
    }

    pub fn creator() -> Self {
        // create character
        println!("\nCHARACTER CREATOR");

        println!("Class options:");
        println!("1. Archer");
        println!("2. Soldier");
        println!("3. Wizard");

        // take input
        let input = prompt("(1-3) ");

        // get class
        let class: Class;
        match input.trim() {
            "1" => class = Class::Archer,
            "2" => class = Class::Solier,
            "3" => class = Class::Wizard,
            _ => class = Class::Archer,
        }

        Self::new(class)
    }

    pub fn describe(&self) {
        println!("\nMY CHARACTER");
        println!("class: {:?}", self.class);
        println!("money: {}", self.coins);

        // continue...
        prompt("\nPress enter to continue...");
    }
}