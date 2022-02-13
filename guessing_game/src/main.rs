use std::io;
use rand::Rng;
use std::cmp::Ordering;
// The book: https://doc.rust-lang.org/book/title-page.html

/*
    Going to actually try and make this look good.
    Will probably try to follow the conventions of the rust language.
*/

fn main() {
    println!("Guessing Game");
    
    
    /* 
        let - creates variables 
        :: - indicates what it is associated with, String::new = new empty string 
        new - associated function
        Line creates an empty mutable string.
        mut - Mutable, idk what this means currently 11/02/22
    */
    
    
    

    /*
        gen_range() - Range of generation. Includes lower bound, excluded upper bound. 1 <= x < 101
    */
    let secret_number = rand::thread_rng().gen_range(1..101); // Generates a random number local to the current thread. 
    //println!("The secret number is {}", secret_number);       // Not really that secret tbh
    

    loop { // Simple infinite loop :)

        println!("Input your guess: ");
        let mut guess = String::new();


        // ------------------Receiving User Input------------------

        /*
            io - shortened std::io, std::io::stdin() would work if 'use std::io;' is not present
            read_line - Self explanatory tbh
            (&mut guess) - The string the input is APPENDED to. 
                - APPENDS THE DATA, DOES NOT OVERWRITE.
                - Variable must be mutable to allow for changing of the string's content
            & - reference, allows multiple parrts of the code access the same data without copying data to memory.
                - References, like variables, are immutable by default and must have '&mut <variable>' to make it mutable/changeable.

            Enums (From a retarded perspective):
                - These have a set of possibilities called varriants
                - Often used with 'match', a conditional which allows different code to execute based on the variant of the enum during evaluation.
            read_line's variants are: 
                - Ok  : Successful
                - Err : Contain info on how or why it failed
        */
        io::stdin()
            .read_line(&mut guess)              // read_line returns a value (io::Result), these are enums (????).
            .expect("Failed to read line");     // crashes the program if read_line returns 'Err', providing the error message in terminal

        /*
            Converts guess into an unsigned 32 bit integer.
            'Shadows' the old value with a new one. 
                - Essentially removing the old value, allowing us to re-use the name for conversion
            .trim() is required as pressing ENTER to complete the input will append a '\n' to the string. trim removes this
                - '\r' is also added, resulting in '\r\n', if the system is running Windows
        */
        
        /*
            This now uses enum matching to check if the value is a number. If ok, go ahead with program. If Err, start loop again.
        */
        let guess: u32 = match guess            // 2nd guess refers to the original string
            .trim()                             // Eliminates whitespace at beginning or end, ensures only numbers/letters remain.
            .parse() {                          // Parses a string into a number, : u32 required to let Rust know what type we want from parse.
                Ok(num) => num,
                Err(_) => continue,
            };
            //.expect("Please type a number!");   // If parse() fails to convert to a number, outputs "Please type a number!"

        println!("You guessed: {}", guess);

        // ------------------Comparing------------------

        /*
            Not going to lie, this shit is cool
            
            Ordering is an enum with the variants: 'Less', 'Greater' and 'Equal'. cmp compares two variables and returns it with a variant of Ordering.
            match is then used to decide what to do based on the enum's variant returned by cmp.
            Functions like an if, elif. Ends comparing variant to options when it finds a match.
        */
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        /*
            Okay, I'm not going to follow these fucking guidelines, "indent with four spaces"; fuck off.

            TIL VScode converts tabs to spaces. :)
        */
    }

    /* 
        This likely won't be read by anyone and I just want to write this down somewhere to get it off of my chest.
        If you know me in real life then fuck off.
        I will likely just write this as I think. Welcome to my thoughts. They are non-sensical and fucked.

        For the past few years I have questioned my gender identity and honestly, idk what to fucking think of it.
        Since 2019/2020 I have been questioning (as far as I remember) and it kinda comes in waves.
        Every night I stay awake in bed for atleast 1 hour, I no longer track the time of it but it would sometimes reach 3 hours of just
        laying in bed and being in a make believe world. 
        This had allowed me to experiment with various concepts, how I feel about people, who I am to myself and my future. 
        I don't know when, I dont know why but after a while my thoughts of the future always began to include one thing.
        
        "I wish I become a girl"
        I don't know why.

        Am I comfortable with who I am right now? 
            I guess, I haven't experienced any different so how am I meant to know?
            The main problem I have with my body is weight, I hate it but I'm not even overweight.
        
        I likely won't start improving on myself until uni, I guess to myself this feels like a fresh start.
        My friends are unlikely to go the same university as me, I haven't even met with the people I speak to most and my friends from
        college dont exactly achieve the same as me so they are limited to universities that im not interested in.
        Come to think of it, I don't know why i said 'college friends'. It is literally one person that I speak to and I only started speaking to 
        him because I was forced to. 

        So that leads to university, what is currently a fresh start, I hate speaking to people I don't know (hell, I hate making eye contact or speaking to people
        I know and trust if they initiate the conversation). I currently plan to get a studio accomodation in university, this would help me with privacy during
        'experimentation'.
        I've already come to the conclusion that I will probably cross-dress (is it even crossdressing if I question if I'm even male?)
        Fuck.
        I've thought about this for 2-3 years and I still haven't come to a conclusion for my gender?
        I
        I hate making decisions that impact my life, I assume everyone does too but most people I see already have their shit planned.
        I'm currently studying engineering with plans to go onto electrical and electronic engineering but is this what I want to do?
        Fucking taking engineering for college was an impulse decision, I had planned to go into computer science and just do programming but years of trying
        to one up myself during secondary school (age 11-16) and showcasing these 'projects' that I basically stole from the internet made me feel like an
        imposter.
        I felt like I was just being fake. 
        I have the knowledge of programming I think
        I can absolutely ace programming exams in college to the point where my microelectronics teacher lets me independantely learrn in the corner of the room,
        but I never felt it to be 'right' for me. I guess I just got burned out. 
        I know if I put effort into something I could easilly learn it but I can't put the effort into something unless I like the subject.

        My affinity for computers probably comes from being introduced to them, and being given a personal laptop, at age 7. Giving internet access to a 7 year old kid
        was probably not the best idea but it gave me my computer skills that I use every day. 
        Ngl, I wasn't even interested in programming until one day my friend showed me some shit he did in scratch. At this point he was always better than me in everything
        and so, determined to be better than him in something, tried scratch out for the first time. 
        I was a natural at it, when it eventually got taught in primary school (year 6, around age 10) I was able to flex my knowledge and skills. 
        At that age being good at a subject was 'cool' so I rode that high until it declined in year 8 when people stopped caring. 
        I would make daft games in scratch like a maze game with a shrinking/expansion mechanic so you had to backtrack and complete the maze in a certain order so you
        would be the corrrect size to reach the end.
        Fun concept, I might make it some time. Could be a future project if I actually go anywhere with Rust. 

        In school the classes in computer science were given 'programming tasks'. This was just a collection of 100+ programming activities. 
        They ranged from classic 'Hello, world' to supposably difficult tasks of converting a number to binary and then the binary into hex. 
        Fun tasks, they provided the knowledge required to get a grasp of python to the point where I can use it to make shitty programs of anything I need,
        but that's the issue. They are all just 'shitty'. I don't know if this is just self deprecation or just me knowing the value of my work but I am not
        satisfied with the quality of my work, ever.

        I try many times to get back into programming, I try javascript, I try java, I try Ruby on rails and I get bored. I have 0 motivation for anything that
        requires effort. I guess that's because I've never really had to put effort into anything meaningful.
        https://imgur.com/a/EEgD08s
        This link above just shows that even in 2020 I recognised my lack of motivation, and this was just after following a Ruby on Rails tutorial to create
        a 'twitter clone' : http://twitterc.herokuapp.com/
        I don't feel like 'I' made it. 
        I simply followed instructions, *I didn't do anything*

        So, as a last ditch effort to prove to myself that **I** can learn programming without following shitty tutorials and stealing code I'm going to learn Rust.
        I need to learn to not put myself down. 
        I need to learn who I am and build upon that
        I can't tell if it is the sleep deprivation (it is currently 02:07am UTC), or if something is wrong with me but I feel like crying while writing this.

        In the previously mentioned 'fantasy world' I've settled on a name I like for when im in fantasy. It's probably just the egg cracking but I kinda like the 
        name 'Lilith' :). I basically only refer to myself as that whenever I'm in that mood for the fantasy or whenever I want to escape reality and pretend that
        I'm a girl that isn't a fuck-up like me. 

        It's getting late so I'm going to stop, laying in bed might help me with pondering this situation. 
        Also, I am now just recognising that I'm falling into the stereotype of a gender-questioning programmer learning Rust. 

        Not even sure if I'm going to commit this or if im just going to stick it into a txt file and .gitignore it

        Farewell for now to whoever reads this,
                ~ L
*/
}
