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
    print!("Opponent Thinking ");
    io::stdout().flush().unwrap(); // prints line above immediately

    let num_dots = 4;
    let dot_duration = Duration::from_millis(500);

    execute!(io::stdout(), cursor::Hide).unwrap(); // hides cursor

    for _ in 0..2 {
        print!("\rOpponent Thinking "); // goes back to the beginning of the line after each cycle

        for _ in 0..num_dots { // prints dots one after another
            print!(".");
            io::stdout().flush().unwrap(); // prints dot immediately
            thread::sleep(dot_duration);
        }

        print!("\rOpponent Thinking "); // erases the dots by moving the cursor back and printing spaces
        for _ in 0..num_dots {
            print!(" ");
        }
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    // clears the line completely after the animation is over
    print!("\r"); // moves cursor to the beginning of the line
    print!("                  "); // print enough spaces to overwrite the line
    io::stdout().flush().unwrap(); // prints spaces immediately

    execute!(io::stdout(), cursor::Show).unwrap();    // shows the cursor again
}