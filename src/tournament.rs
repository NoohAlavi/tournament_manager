mod player;
use std::{
    fs, 
    io::{
        self, 
        Write
    }
};
use colored::*;

pub struct Tournament {
    pub name: String,
    pub players: Vec<player::Player>,
    pub rounds: u8
}

impl Tournament {
    pub fn new(tourney_name: &str, num_of_rounds: u8) -> Tournament {
        println!("Tournament '{}' has been created!", tourney_name);
        Tournament {
            name: tourney_name.to_string(),
            players: Vec::new(),
            rounds: num_of_rounds
        }
    }

    pub fn register_player(&mut self, player_name: &str) {
        self.players.push(player::Player::new(player_name.to_string()));
        println!(
            "{}",
            format!("{} has been added to {}!", player_name, self.name).yellow()
        );
    }

    pub fn start_round(&mut self, round_num: u8) {
        cls();
        println!("{}",
            &format!("Current Round: {}/{}\n\n", round_num, self.rounds).blue()
        );
        self.print_standings();
        self.print_pairings();

        let mut new_score: String;
        
        for player in &mut self.players {
            new_score = String::new();

            println!(
                "{}",
                &format!("How many points did {} score this round?", 
                player.name).green()
            );
            io::stdin()
                .read_line(&mut new_score)
                .unwrap_or_else(|_| panic!("{}", "[ERROR] Could not read line.".red()));
            let new_score: f32 = new_score.trim()
                .parse()
                .unwrap_or_else(|_| panic!("{}", "[ERROR] Please enter a valid number.".red()));

            player.update_score(new_score);
        }

        if round_num < self.rounds {
            self.start_round(round_num + 1);
        } else {
            self.end_tournament();
        }
    }

    pub fn get_pairings(&self) -> String {
        String::from("<pairings (work in progress)>\n")
    }

    pub fn print_pairings(&self) {
        println!("{}", self.get_pairings().cyan());
    }

    pub fn get_standings(&self) -> String {
        let mut standings = format!("{} scores:\n\n", self.name);
        
        for player in &self.players {
            let score_str = format!("{}: {}", player.name, player.score);
            
            standings.push_str(&score_str);
            standings.push('\n');
        }
        standings
    }

    pub fn print_standings(&self) {
        println!("{}", self.get_standings().yellow())
    }

    pub fn save_standings(&self) -> io::Result<()> {
        let file_name = format!("{}_scores.txt", self.name);
        let mut file = fs::File::create(file_name)?;
        file.write_all(self.get_standings().as_bytes())?;

        Ok(())
    }

    pub fn end_tournament(&self) {
        cls();
        println!("{n} has ended! Standings have been saved in `{n}_scores.txt`\n", n = self.name);
        self.print_standings();
        self.save_standings().unwrap();
    }
}

// Clear the screen
pub fn cls() {
    println!("{esc}c",esc=27 as char);
}
