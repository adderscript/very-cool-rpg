#[derive(Clone)]
pub enum EnemyType {
    Slime,
    Skelly,
    EvilWizard,
}

pub struct Enemy {
    hp: u32,
    damage: u32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Slime => Self {
                hp: 3,
                damage: 1,
            },
            EnemyType::Skelly => Self {
                hp: 2,
                damage: 3
            },
            EnemyType::EvilWizard => Self {
                hp: 5,
                damage: 2,
            },
        }
    }
}