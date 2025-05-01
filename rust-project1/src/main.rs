fn main() {
    println!("Hello, world!");

    // variables

    // 1) Integers
    let x: i32 = -5;
    let y: u32 = 5;
    let z: f32 = 3.14;

    print!("x: {}", x);
    print!("\ny: {}", y);
    println!("\nz: {}", z);

    print!("x: {}, y: {}, z: {}", x, y, z);

    // if we didn't specify the type then it will automatically detect the  by-deafult type of variable.
    let a = 5;
    println!("\na: {}", a);
    let a: i8 = 6;
    println!("\na: {}", a);

    // 2) Booleans
    let is_male: bool = true;
    let is_above_18: bool = true;
    if is_male{
        println!("You are a male")
    }
    else{
        println!("You are not a male")
    }
    if is_male && is_above_18 {
        print!("You are a male and above 18");
    }

    // 3) Strings
    let name: String = String::from("Yuvraj Rajput");
    println!();
    println!("{}", name);

    // Type safety k liye ye char1 ko Option<char> bna dega. Iska mtlab ye h ki ye confirm nhi h ki name.chars().nth(0) 1 character value hi dega ya nhi.
    let char1: Option<char> = name.chars().nth(0);
    let char1000: Option<char> = name.chars().nth(1000);

    // Method-1) Print value at any index of the string
    match char1{
        Some(c) => println!("{}", c),
        None => println!("No character at index")
    }

    match char1000{
        // yadi character mil jata h toh ye run hoga.
        Some(c) => println!("{}", c),
        // yadi character nhi milta toh ye run hoga.
        None => println!("No character at index 1000"),
    }

    // Method-2) Print value at any index of the string
    // character mil jata h us index pr toh vo character print kr dega aur yadi character nhi milta toh error aa jaiye ga.
    println!("char1: {}", char1.unwrap());

    // println!("char1000: {}", char1000.unwrap());

    // Conditional Statements
    
    // 1) if-else
    let x: i32 = 99;
    let is_even: bool = is_even(x);
    if x == 0 {
        println!("{} is 0", x);
    }
    else if is_even {
        println!("{} is even", x);
    }
    else {
        println!("{} is odd", x);
    }

    // loops

    // for loop
    for i in 0..11 {
        println!("{}", i);
    }

    // Traverse a string and print only first word of sentence.

    let sentence: String = String::from("Yuvraj is my name");

    let first_word: String = get_first_word(sentence);

    println!("First word is {}", first_word);
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
     }
     return ans;
}

