use std::io;
struct rectangle{
    width: u32,
    height: u32,
}

// Method, function relating to a specific struct
impl rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn rect_area(){
    let rectangle1 = &rectangle{
        width: input_dim("Width"),
        height: input_dim("Height"),
    };
    println!("Area: {}", rectangle1.area());
    println!("Can Hold: {}", rectangle1.can_hold(&rectangle{width: 10, height: 10}));
}

fn input_dim(opt: &str) -> u32{
    let mut number = String::new();
    println!("Give me a {}: ",opt);
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: u32 = number.trim().parse().expect("Please type a number!");
    number
}

