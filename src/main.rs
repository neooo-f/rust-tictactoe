mod display_utils;
mod bot;

use std::io;
use std::io::Write;
use std::{thread};
use std::time::Duration;

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
                    continue; // Continue to the next iteration of the inner loop
                }
            };

            if num < 1 || num > 9 {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue; // Continue to the next iteration of the inner loop
            }

            let new_pos_index: usize = num as usize;
            if board[new_pos_index - 1] != 0 {
                println!("Position already taken. Please choose another position.");
                continue; // Continue to the next iteration of the inner loop
            }

            display_utils::clear_console();
            board[new_pos_index - 1] = 1;

            println!();
            display_utils::print_board(&board);

            /* if check_winner(&board) {
                println!("You win!");
                break; // Break out of the inner loop, game over
            } */

            if board.iter().all(|&x| x != 0) {
                println!("It's a draw!");
                break; // Break out of the inner loop, game over
            }

            display_utils::clear_console();
            println!();
            display_utils::opponent_thinking();
            bot::make_move(&mut board);

            println!();
            display_utils::print_board(&board);

            /* if check_winner(&board) {
                println!("Opponent wins!");
                break; // Break out of the inner loop, game over
            } */

            if board.iter().all(|&x| x != 0) {
                println!("It's a draw!");
                break; // Break out of the inner loop, game over
            }
        }

        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read input");
        let play_again = play_again.trim().to_lowercase();

        if play_again != "yes" {
            break; // Break out of the outer loop, game over
        }
    }
}

/* fn main() {
    let mut board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    println!("Game Starts!");
    println!();

    display_utils::print_board(&board);
    println!();

    println!();
    print!("Enter next position: (number 1 - 9): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let num: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number between 1 and 9.");
            return; // Exit the program
        }
    };

    if num < 1 || num > 9 {
        println!("Invalid input. Please enter a number between 1 and 9.");
    }

    display_utils::clear_console();

    let new_pos_index: usize = input.trim().parse().unwrap();
    board[new_pos_index - 1] = 1;

    println!();
    display_utils::print_board(&board);

    // sleep 1 sec
    display_utils::clear_console();
    thread::sleep(Duration::from_millis(2000));

    // simulate opponent thinking
    display_utils::opponent_thinking();

    bot::make_move(&mut board);

    println!();
    display_utils::print_board(&board);
} */

/* when user inputs a number (1 - 9), on this filed should be placed a x (only if field isn't already in use) */
/* mapped field values are 0 = not used, 1 = x, 2 = o  */
