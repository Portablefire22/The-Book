use clearscreen::clear;
fn main() {
    clear().expect("Failed to clear screen");
    // Can't be bothered to dynamically create an array of 80 length and assign.
    let mut strings: [[char; 80]; 5] = [
        ['#',' ','#',' ','#','#','#',' ','#',' ',' ',' ','#',' ',' ',' ','#','#','#',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        ['#',' ','#',' ','#',' ',' ',' ','#',' ',' ',' ','#',' ',' ',' ','#',' ','#',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        ['#','#','#',' ','#','#','#',' ','#',' ',' ',' ','#',' ',' ',' ','#',' ','#',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        ['#',' ','#',' ','#',' ',' ',' ','#',' ',' ',' ','#',' ',' ',' ','#',' ','#',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        ['#',' ','#',' ','#','#','#',' ','#','#','#',' ','#','#','#',' ','#','#','#',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ']
    ];
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
