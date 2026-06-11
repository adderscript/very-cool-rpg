use std::io::{self, Write};

mod character;
mod enemy;
mod quest;
mod quest_manager;
mod save_manager;
mod utils;

use crate::character::Character;
use crate::quest_manager::QuestManager;
use crate::save_manager::SaveManager;
use crate::utils::prompt;

fn main() {
    println!("\nVERY COOL RPG GAME");
    println!("1. New Game");
    println!("2. Load Game");
    println!("3. Quit");

    let input = prompt("(1-3) ");

    // match action
    match input.trim() {
        "1" => {
            let character = Character::creator();
            run(character);
        },
        "2" => {
            let character = SaveManager::load();
            run(character);
        },
        _ => return,
    }
}

fn run(character: Character) {
    let quest_manager = QuestManager::new();

    loop {
        // prompt
        println!("\nHere are your options:");
        println!("1. Go Questing");
        println!("2. My Character");
        println!("3. Quit");

        let input = prompt("(1-3) ");

        // match action
        match input.trim() {
            "1" => quest_manager.choose_quest(),
            "2" => character.describe(),
            "3" => {
                quit(&character);
                return;
            },
            _ => continue,
        }
    }
}

fn quit(character: &Character) {
    println!("\nSaving...");
    SaveManager::save(&character);
}