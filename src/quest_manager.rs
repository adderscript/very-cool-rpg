use crate::utils::prompt;
use crate::enemy::EnemyType;
use crate::quest::Quest;

pub struct QuestManager {
    quests: Vec<Quest>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: vec!(
                Quest {
                    name: "A quest",
                    enemies: vec!(
                        Enemy::new(EnemyType::Slime),
                        Enemy::new(EnemyType::Skelly),
                    ),
                    reward_coins: 10,
                    completed: false,
                }
            )
        }
    }

    pub fn choose_quest(&self) {
        println!("\nHere are the currently available quests:")
    }
    
    pub fn start_quest(&self, quest_index: u32) {
        let quest = self.quests[quest_index-1];

        println!("\nQUEST STARTED");
        println!("ENEMY 1: SLIME");
    }
}