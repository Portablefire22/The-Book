use colored::*;
use std::time::{Instant}; // https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html
// Not exactly the most elegant solution
fn main(){
    let start = Instant::now();
    println!("Flag Test");
    println!("Created using {}{}{}{}{}{}{}","c".red(),"o".yellow(),"l".green(),"o".blue(),"r".magenta(),"e".cyan(),"d".red());

    // loop 3 times
    for _i in 1..4{
        println!("{}","#############################################################################".truecolor(131,170,247));
    }
    for _i in 1..4{
        println!("{}","#############################################################################".truecolor(247,129,128));
    }
    for _i in 1..4{
        println!("{}","#############################################################################".truecolor(255,255,255));
    }
    for _i in 1..4{
        println!("{}","#############################################################################".truecolor(247,129,128));
    }
    for _i in 1..4{
        println!("{}","#############################################################################".truecolor(131,170,247));
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}",duration);
    //test2();
}

// Time for some over-engineering or ,simply, improving how well it scales :)

fn test2() { // On average 1 to 0.8 ms quicker
    let start = Instant::now();
    println!("Different Method");
    println!("Created using {}{}{}{}{}{}{}","c".red(),"o".yellow(),"l".green(),"o".blue(),"r".magenta(),"e".cyan(),"d".red());

    for j in 1..6{

        // https://doc.rust-lang.org/book/ch06-02-match.html
        let col;
        col = match j {
            1 | 5 => (131,170,247), // 1 or 5
            2 | 4 => (247,129,128), // 2 or 4 
            3 => (255,255,255),
            _ => unreachable!() // _ : Any value not included are unreachable.
        };
        // loop 3 times
        for _i in 1..4{
            println!("{}","#############################################################################".truecolor(col.0,col.1,col.2));
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}",duration);
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn avg_time(){ // The two programs are consistently similar in speed but the fastest varies with each run
        // From my tests, consistently, test2 is faster than main
        
        let mut totals = [0.0,0.0];
        for _i in 0..1000{
            let start = std::time::Instant::now();
            assert_eq!(main(),());
            let duration = start.elapsed();
            totals[0] += duration.as_millis() as f32;
        }

        for _i in 0..1000{
            let start = std::time::Instant::now();
            assert_eq!(test2(),());
            let duration = start.elapsed();
            totals[1] += duration.as_millis() as f32;
        }
        println!("Five loop version avg time(ms): {}",totals[0]/1000.0);
        println!("Nested loop version avg time(ms){}",totals[1]/1000.0);
    }
}