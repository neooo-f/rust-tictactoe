use std::{thread};
use std::time::Duration;
use crossterm::{execute, cursor};
use std::io;
use std::io::Write;

fn map_field(field: &u8) -> String {
    match field {
        0 => String::from(" "),
        1 => String::from("x"),
        2 => String::from("o"),
        _ => String::from("invalid")
    }
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // scrolls down (clears screen)
}

pub fn print_board(fields: &[u8]) {
    println!("{} | {} | {}", map_field(&fields[0]), map_field(&fields[1]), map_field(&fields[2]));
    println!("---------");
    println!("{} | {} | {}", map_field(&fields[3]), map_field(&fields[4]), map_field(&fields[5]));
    println!("---------");
    println!("{} | {} | {}", map_field(&fields[6]), map_field(&fields[7]), map_field(&fields[8]));
}

pub fn opponent_thinking() {
    print!("Opponent Thinking "); // Print "Loading" once
    io::stdout().flush().unwrap(); // Ensure "Loading" is printed immediately

    let num_dots = 4;
    let dot_duration = Duration::from_millis(500);

    // Hide the cursor
    execute!(io::stdout(), cursor::Hide).unwrap();

    for _ in 0..2 {
        // Go back to the beginning of the line after each cycle
        print!("\rOpponent Thinking ");

        // Print dots one after another
        for _ in 0..num_dots {
            print!(".");
            io::stdout().flush().unwrap(); // Ensure dot is printed immediately
            thread::sleep(dot_duration);
        }

        // Erase the dots by moving the cursor back and printing spaces
        print!("\rOpponent Thinking ");
        for _ in 0..num_dots {
            print!(" ");
        }
        io::stdout().flush().unwrap(); // Ensure spaces are printed immediately
        thread::sleep(Duration::from_secs(1)); // Wait for 1 second
    }

    // Clear the line completely after the animation is over
    print!("\r"); // Move cursor to the beginning of the line
    print!("                  "); // Print enough spaces to overwrite the line
    io::stdout().flush().unwrap(); // Ensure spaces are printed immediately

    // Show the cursor
    execute!(io::stdout(), cursor::Show).unwrap();
}