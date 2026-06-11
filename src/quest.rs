use crate::enemy::Enemy;

struct Quest {
    name: String,
    enemies: Vec<Enemy>,
    reward_coins: u32,
    completed: bool,
}