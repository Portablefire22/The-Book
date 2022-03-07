use colored::*;
use std::io;

/*
    Plan:

    Take input, change board
    Check if win and output winner
    Maybe use min-max algorithim to make it unbeatable.


*/


fn main() {
    println!("{}","Noughts And Crosses".truecolor(247,129,128));
    let mut board_state = [' ',' ',' ',' ',' ',' ',' ',' ',' '];
    let mut is_player1: bool = true;
    board_output(board_state);
    'GameLoop: loop{
        
        let take_input_results = take_input(is_player1, board_state); 
        is_player1 = take_input_results.0;
        board_state = take_input_results.1;
        //println!("{:?}", take_input_results.1);
        board_output(board_state);
        if check_board_state(board_state){
            break 'GameLoop
        }
        //clearscreen::clear().expect("Failed to clear"); // TODO Find a better place to put this
    }
    
}

fn take_input(mut is_player1: bool, mut board_state: [char; 9]) -> (bool, [char; 9]){
    let mut input: String = String::new();
    let mut letter: char;
    match is_player1 {
        true => letter = 'X',
        false => letter = 'O',
    };
    println!("Select position for {} (1-9)",letter);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: usize = match input
        .trim() // Trims whitespace
        .parse() { // Parses the string into a number
            Ok(num) => num,
            Err(_) => 10, 
        };
    match input {
        0..=8 => if board_state[input] != ' '{
            println!("Space already occupied by {}, try again.", board_state[input])
        } else{
            board_state[input] = letter;
            is_player1 = !is_player1;
        }
        _ => println!("Not within the board's range, try again.")
    }
    return (is_player1, board_state);
}

fn board_output(board_state: [char; 9]){
    let mut i = 0;
    
    while i <= board_state.len() - 1 {
        //println!("{}", i); // Debug
        println!("{} | {} | {}",board_state[i],board_state[i+1],board_state[i+2]);
        i = i + 3;
    }
    
}

fn check_board_state(board_state: [char; 9]) -> bool{
    let mut occupied: u32 = 0;
    let mut player1_victory: bool = false;
    let mut player2_victory: bool = false;
    for x in board_state { // Checks to see if board is full
        if x != ' '{
            occupied = occupied + 1;
        }
    }
    let mut victory: bool = false;
    // TODO finish conditions and check for diag
    for x in 0..8{ // Checks for winning conditions
        match x {
            0 => if board_state[x] == board_state[x+1] && (board_state[x+1] == board_state[x+2]) { 
                victory = true;
            },
            1 => if board_state[x] == board_state[x-1] && (board_state[x] == board_state[x+1]){
                victory = true;
            }
            _ => unreachable!()
        }
        if victory{
            match board_state[x] {
                'X' => player1_victory = true,
                'O' => player2_victory = true,
                _ => std::panic::panic_any("A victory condition was achieved without the victory being attributed to a player!"),
            }
        }
        break;
    }


    if player1_victory || player2_victory{
        let mut letter: char = ' ';
        if player1_victory{
            letter = 'X';
        } else if player2_victory {
            letter = 'O';
        }
        println!("Congratulations to {} for winning!", letter);
        return true;
    } else if occupied == 9 {
        println!("Tie.");
        return true
    } 
    else{
        return false
    }
}