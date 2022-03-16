use clearscreen::clear;
use std::io;
fn main() {
    
    clear().expect("Failed to clear screen");
    // Can't be bothered to dynamically create an array of 80 length and assign.
    let mut strings: [[char; 80]; 5] = [
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ']
    ];
    println!("Input your string (20 Char max): ");
        let mut stri = String::from("") ;
    while stri.len() > 20 || stri == ""{
        stri = String::from("");
        io::stdin()
        .read_line(&mut stri)              // read_line returns a value (io::Result), these are enums (????).
        .expect("Failed to read line");     // crashes the program if read_line returns 'Err', providing the error message in terminal
    } 
    strings = lookup_char(strings, stri.trim().to_string());
    loop {
        clear().expect("Failed to clear screen");
        let mut x: usize = 0;
        let mut temp: char;
        let mut z: i32;
        for current_row in strings{
            temp = current_row[0]; // Store first value
            z = 1;
            for _current_char in strings[x]{
                if z != 80{ // Stops indexing error, seems to work without any problems
                    strings[x][(z - 1) as usize] = strings[x][z as usize]; // Current character gets assigned to previous character
                    z = z + 1;
                }
            }
            strings[x][(strings[x].len() - 1) as usize] = temp; // Last value in lists becomes the original first value (Sends first value to the last position)
            for current_char in strings[x]{ // Outputs characters onto screen
                print!("{}",current_char)
            }
            println!("");
            x = x + 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    
}

// Convert text to giant ASCII text
fn small_to_big() -> [[char; 80]; 5]{
    
    [
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ']
    ]
}

fn lookup_char(mut txt: [[char; 80]; 5], input_str: String) -> [[char; 80]; 5]{
    
    let mut pos: u8 = 0;
    
    for i in txt {
        let mut current_pos: u8 = 0;
        for chr in i{
            if chr == '#' && current_pos > pos{
                pos = current_pos;
            }
            current_pos = current_pos + 1;
        }    
    }
    if pos != 0 {
        pos = pos + 2;
    }
    for input_chr in input_str.chars(){
        match input_chr.to_ascii_uppercase(){
            'A' => {
                txt[0][(pos+1) as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';

                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
            },
            'B' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';
                
                txt[0][(pos+1) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

                txt[1][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
            },
            'C' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';

                txt[0][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

                txt[0][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
            },
            'D' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';
                
                txt[0][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

                txt[2][(pos+2) as usize] = '#';
                
                txt[1][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
            },
            'E' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';

                txt[0][(pos+1) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';

                txt[2][(pos+1) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';

                txt[4][(pos+1) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
            },
            'F' => {
                txt[0][pos as usize] = '#'; txt[0][(pos+1) as usize] = '#'; txt[0][(pos+2) as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#'; txt[2][(pos+1) as usize] = '#'; txt[2][(pos+2) as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';
            },
            'G' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';

                txt[0][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

                txt[0][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';

            },
            'H' => {
                txt[0][pos as usize] = '#';
                txt[1][pos as usize] = '#';
                txt[2][pos as usize] = '#';
                txt[3][pos as usize] = '#';
                txt[4][pos as usize] = '#';

                txt[2][(pos+1) as usize] = '#';
                
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
            },
            'I' => {
                txt[0][(pos+1) as usize] = '#';
                txt[1][(pos+1) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[3][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

                txt[0][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';

                txt[4][(pos) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';


            },
            'J' => {
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';

                txt[4][(pos+1) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[3][(pos) as usize] = '#';
            },
            'K' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';

                txt[4][(pos+2) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
            },
            'L' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'M' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[1][(pos+1) as usize] = '#';
            },
            'N' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
            },
            'O' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'P' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
            },
            'Q' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'R' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';

                txt[3][(pos+1) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
            },
            'S' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
                txt[4][(pos) as usize] = '#';

            },
            'T' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+1) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[3][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'U' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'V' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';

            },
            'W' => {
                txt[0][(pos) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[2][(pos) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+2) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[3][(pos+1) as usize] = '#';
            },
            'X' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[3][(pos) as usize] = '#';
                txt[3][(pos+2) as usize] = '#';
                txt[4][(pos) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
            },
            'Y' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';
                txt[3][(pos+1) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
            },
            'Z' => {
                txt[0][(pos) as usize] = '#';
                txt[0][(pos+1) as usize] = '#';
                txt[0][(pos+2) as usize] = '#';
                txt[1][(pos+2) as usize] = '#';
                txt[2][(pos+1) as usize] = '#';

                txt[3][(pos) as usize] = '#';
                txt[4][(pos+2) as usize] = '#';
                txt[4][(pos+1) as usize] = '#';
                txt[4][(pos) as usize] = '#';
            },
            ' ' => pos = pos,
            _ => std::panic::panic_any("String contains invalid characters! (ASCII only).")
        }
        pos = pos + 4;
    }
    println!("{}", pos);
    return txt
}