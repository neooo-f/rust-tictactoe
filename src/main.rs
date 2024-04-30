mod display_utils;
mod logic;

use std::io;
use std::io::Write;

fn main() {
    println!("Game Starts!");
    println!();

    loop {
        let mut board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        display_utils::print_board(&board);
        println!();

        loop {
            println!();
            print!("Enter next position: (number 1 - 9): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let num: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a valid number between 1 and 9.");
                    continue;
                }
            };

            if num < 1 || num > 9 {
                println!("Invalid input. Please enter a number only between 1 and 9.");
                continue;
            }

            let new_pos_index: usize = num as usize;
            if board[new_pos_index - 1] != 0 {
                println!("Position already taken. Please choose another position.");
                continue;
            }

            display_utils::clear_console();
            board[new_pos_index - 1] = 1;

            if logic::check_winner(&board) {
                println!("You win!");
                println!();
                display_utils::print_board(&board);
                println!();
                break;
            }

            if board.iter().all(|&x| x != 0) {
                println!("It's a draw!");
                break;
            }

            display_utils::clear_console();
            println!();
            display_utils::opponent_thinking();
            logic::make_move(&mut board);

            println!();
            display_utils::print_board(&board);

            if logic::check_winner(&board) {
                println!();
                println!("Opponent wins!");
                break;
            }

            if board.iter().all(|&x| x != 0) {
                println!("It's a draw!");
                break;
            }
        }

        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read input");
        let play_again = play_again.trim().to_lowercase();

        if play_again != "yes" {
            break;
        }

        display_utils::clear_console();
    }
}