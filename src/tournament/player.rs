pub struct Player {
    pub name: String,
    pub score: f32,
    pub last_paired_player_name: String,
    pub is_bye: bool
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0.0,
            last_paired_player_name: String::new(),
            is_bye: false
        }
    }

    pub fn update_score(&mut self, updated_score: f32) {
        self.score += updated_score;
    }
}
