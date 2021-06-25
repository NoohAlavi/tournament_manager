mod tournament;
use std::io;
use colored::*;

fn main() {
    tournament::cls(); 

    let mut tourney_name = String::new();
    let mut num_of_rounds = String::new();
    let mut input: String;
    
    println!("{}", "-Tournament Manager-".green().bold());
    println!("{}", "created by nooh alavi\n".green().bold());
    println!("{}", "Contribute to the development at https://github.com/NoohAlavi/tournament_manager\n".green().bold());


    println!("{}", "Please enter a tournament name: ".cyan().bold());
    io::stdin()
        .read_line(&mut tourney_name)
        .unwrap_or_else(|_| panic!("{}", "[ERROR] Could not read line.".red().bold()));
    tourney_name = tourney_name.trim().to_string();

    println!("{}", "Please enter a number of rounds: ".cyan().bold());
    io::stdin()
        .read_line(&mut num_of_rounds)
        .unwrap_or_else(|_| panic!("{}", "[ERROR] Could not read line.".red().bold()));
    let num_of_rounds: u8 = num_of_rounds.trim()
        .parse()
        .unwrap_or_else(|_| panic!("{}", "[ERROR] Please enter a valid number.".red().bold()));

    let mut tourney = tournament::Tournament::new(&tourney_name, num_of_rounds);
    
    loop {
        println!("{}", "Enter a player name to register them to the tournament. Type ':start' or ':s' to begin: ".cyan().bold());

        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(|_| panic!("{}", "[ERROR] Could not read line.".red().bold()));
        input = input.trim().to_string();

        if input == ":start" || input == ":s" {
            tourney.start_round(1);
            break;
        } else if input.is_empty() {
            println!("{}", "Please enter a valid player name!".red().bold());
        } 
        else {
            tourney.register_player(&input);
        }
    }
}
