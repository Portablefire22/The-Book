use colored::*;

// Not exactly the most elegant solution
fn main() {
    println!("Flag Test");
    println!("Created using {}{}{}{}{}{}{}","c".red(),"o".yellow(),"l".green(),"o".blue(),"r".magenta(),"e".cyan(),"d".red());

    // loop 3 times
    for i in 1..4{
        println!("{}","#############################################################################".truecolor(131,170,247));
    }
    for i in 1..4{
        println!("{}","#############################################################################".truecolor(247,129,128));
    }
    for i in 1..4{
        println!("{}","#############################################################################".truecolor(255,255,255));
    }
    for i in 1..4{
        println!("{}","#############################################################################".truecolor(247,129,128));
    }
    for i in 1..4{
        println!("{}","#############################################################################".truecolor(131,170,247));
    }
    test2();
}

// Time for some over-engineering or ,simply, improving how well it scales :)

fn test2() {
    println!("Different Method");
    println!("Created using {}{}{}{}{}{}{}","c".red(),"o".yellow(),"l".green(),"o".blue(),"r".magenta(),"e".cyan(),"d".red());

    for j in 1..6{

        // https://doc.rust-lang.org/book/ch06-02-match.html
        let mut col = (0,0,0);
        col = match j {
            1 => (131,170,247),
            2 => (247,129,128),
            3 => (255,255,255),
            4 => (247,129,128),
            5 => (131,170,247),
            _ => unreachable!() // _ : Any value not included are unreachable.
        };
        // loop 3 times
        for _i in 1..4{
            println!("{}","#############################################################################".truecolor(col.0,col.1,col.2));
        }
    }
    
}