use colored::*;
use std::io;
use std::time::{Instant}; // https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html
use std::cmp;
/*
    Plan:

    Take input, change board
    Check if win and output winner
    Maybe use min-max algorithim to make it unbeatable.


*/


fn main() {
    let start = Instant::now();
    println!("{}","Noughts And Crosses".truecolor(247,129,128));
    let mut board_state = [' ',' ',' ',' ',' ',' ',' ',' ',' '];
    let mut is_player1: bool = true;
    let mut choice: String = String::new();
    println!("Play against AI (0) or Player (1)");
    let mut i: bool = true;
    let mut ai: bool = false;
    let mut choice_num: i8 = 2;
    while i{ 
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        choice_num = match choice
            .trim() // Trims whitespace
            .parse() { // Parses the string into a number
                Ok(num) => {i = false; num},
                Err(_) => 10, 
        };
    }
    match choice_num{
        0 => ai = true,
        1 => ai = false,
        _ => println!("Only input 0 or 1")
    }
    
    board_output(board_state);
    'GameLoop: loop{
        
        let take_input_results = take_input(is_player1, board_state, ai); 
        is_player1 = take_input_results.0;
        board_state = take_input_results.1; 
        if ai { board_state = best_move(board_state); }
        board_output(board_state);
        if check_board_state(board_state,ai).0{
            break 'GameLoop
        }
        //clearscreen::clear().expect("Failed to clear"); // TODO Find a better place to put this
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}",duration);
    
}

fn take_input(mut is_player1: bool, mut board_state: [char; 9], ai: bool) -> (bool, [char; 9]){
    let mut input: String = String::new();
    let letter: char;
    match is_player1 {
        true => letter = 'X',
        false => letter = 'O',
    };
    println!("Select position for {} (0-8)",letter);
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
            if !ai { is_player1 = !is_player1; }
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

fn check_board_state(board_state: [char; 9], ai: bool) -> (bool, char){
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
    //for x in 0..8{ // Checks for winning conditions
        
        /*match x {
            0 => if (board_state[x] == board_state[x+1] && (board_state[x+1] == board_state[x+2])) || (board_state[x] == board_state[x+4] && board_state[x+4] == board_state[x+8]) { 
                victory = true;
            },
            1 => if board_state[x] == board_state[x-1] && (board_state[x] == board_state[x+1]){
                victory = true;
            }
            _ => unreachable!()
        }*/
    //}
    // Checking made using https://stackoverflow.com/a/2670776
    let mut value_check_horz: i32;
    let mut value_check_down: i32;
    let mut i: usize = 0;
    while i < board_state.len().try_into().unwrap(){
        value_check_horz = 0;
        for j in 0..3{
            //println!("{}", i+j);
            match board_state[i + j]{
                'X' => value_check_horz = value_check_horz + 1,
                'O' => value_check_horz = value_check_horz - 1,
                ' ' => i=i,
                _ => unreachable!()
            };
        }
        if i == 0{
            for z in 0..3{
                value_check_down = 0;
                let mut c: u32 = 0;
                while c < board_state.len().try_into().unwrap(){
                    match board_state[(z + c) as usize]{
                        'X' => value_check_down = value_check_down + 1,
                        'O' => value_check_down = value_check_down - 1,
                        ' ' => i=i,
                        _ => unreachable!()
                    };
                    c += 3;
                }
                if value_check_down.abs() == 3 || value_check_down.abs() == 3{
                    victory = true;
                }
                
                if victory && !(player1_victory || player2_victory){
                    //println!("{}", value_check_down);
                    match value_check_down {
                        3 => {
                            player1_victory = true;
                            break;
                        },
                        -3 => {
                            player2_victory = true;
                            break;
                        }
                        _ => std::panic::panic_any("A victory condition was achieved without the victory being attributed to a player!"),
                    }
                }
            }
            
        }
        // I can't be fucked to program this one like the others so I'm just going to hard code this solution
        if board_state[0] == board_state[4] && board_state[4] == board_state[8] && board_state[0] != ' '{
            victory = true;
            match board_state[0]{
                'X' => player1_victory = true,
                'O' => player2_victory = true,
                _ => std::panic::panic_any("A victory condition was achieved without the victory being attributed to a player!"),
            }
        } else if (board_state[2] == board_state[4] && board_state[4] == board_state[6]) && board_state[2] != ' '{
            victory = true;
            match board_state[2]{
                'X' => player1_victory = true,
                'O' => player2_victory = true,
                _ => std::panic::panic_any("A victory condition was achieved without the victory being attributed to a player!"),
            }
        }

        if value_check_horz.abs() == 3 || value_check_horz.abs() == 3{
            victory = true;
        }
        if victory && !(player1_victory || player2_victory){
            match value_check_horz {
                3 => {
                    player1_victory = true;
                    break;
                },
                -3 => {
                    player2_victory = true;
                    break;
                }
                _ => std::panic::panic_any("A victory condition was achieved without the victory being attributed to a player!"),
            }
        }
        i += 3;
    }

    if player1_victory || player2_victory{
        let mut letter: char = ' ';
        if player1_victory{
            letter = 'X';
        } else if player2_victory {
            letter = 'O';
        }
        if !ai { println!("Congratulations to {} for winning!", letter); }
        return (true, letter);
    } else if occupied == 9 {
        if !ai { println!("Tie."); }
        return (true, 'T');
    } 
    else{
        return (false, ' ');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check(){
        let test_board = ['X','O',' ',
                          'X','O','O',
                          'X',' ',' '];
        assert_eq!(check_board_state(test_board, false),(true,'X'),);
    }
    #[test]
    fn test_check_two(){
        let test_board = ['X','O','X',
                          'O','O','O',
                          'O','X','X'];
        assert_eq!(check_board_state(test_board, false),(true,'O'));
    }
    #[test]
    fn test_check_tie(){
        let test_board = ['X','O','X',
                          'X','O','O',
                          'O','X','X'];
        assert_eq!(check_board_state(test_board, false),(true,'T'));
    }
    
    #[test]
    fn test_check_fail(){
        let test_board = ['X','O','X',
                          'X',' ','O',
                          'O','X','X'];
        assert_eq!(check_board_state(test_board, false),(false,' '));
    }
    
}


// Trying my best to convert CodingTrain's minimax algorithm to Rust
// IDK what this thing is doing currently, might fix later idk
// https://github.com/CodingTrain/website/blob/main/CodingChallenges/CC_154_Tic_Tac_Toe_Minimax/P5/minimax.js

fn best_move(mut board_state: [char; 9]) -> [char; 9]{
    let mut best_score = (-f64::INFINITY) as i32; // This actually isnt infinity but it is enough to satisfy the requirement
    let mut mve = [0,0]; // Nested arrays are used in the original so these must be modified to correctly identify the position ( 1st pos * 3 + 2nd pos)
    for i in 0..3 {
        for j in 0..3 {
            if board_state[((i * 3)+j) as usize] == ' '{
                board_state[((i * 3)+j) as usize] = 'O';
                let r = minimax(board_state, 0, false);
                let score = r.0;
                board_state = r.1;
                board_state[((i*3) + j) as usize] = ' ';
                if score > (best_score as i32){
                    best_score = score;
                    mve = [i,j];
                }
            }
        }
    }
    board_state[((mve[0] * 3) + mve[1])] = 'O';
    board_state
}

fn minimax(mut board_state: [char; 9], depth: u32, is_maximizing: bool) -> (i32,[char; 9]){
    let result = check_board_state(board_state,true);
    if result.0{
        match result.1{
            'X' => return (-10,board_state),
            'O' => return (10,board_state),
            'T' => return (0,board_state),
            _ => unreachable!()
        }
    }
    if is_maximizing{
        let mut best_score = -(f64::INFINITY) as i32; // This fucking minus sign has caused me to go mentally insane, do not remove this. When removed the algorithm fucks itself.
        for i in 0..3{
            for j in 0..3{
                if(board_state[(i*3)+j]) == ' '{
                    board_state[(i*3)+j] = 'O';
                    let r = minimax(board_state, depth + 1, false);
                    let score = r.0;
                    board_state = r.1;
                    board_state[(i*3)+j] = ' ';
                    best_score = cmp::max(score,best_score);
                }
            }
        }
        (best_score,board_state)
    } else{
        let mut best_score = f64::INFINITY as i32;
        for i in 0..3{
            for j in 0..3{
                if(board_state[(i*3)+j]) == ' '{
                    board_state[(i*3)+j] = 'X';
                    let r = minimax(board_state, depth + 1, true);
                    let score = r.0;
                    board_state = r.1;
                    board_state[(i*3)+j] = ' ';
                    best_score = cmp::min(score,best_score);
                }
            }
        }
        (best_score,board_state)
    }
}