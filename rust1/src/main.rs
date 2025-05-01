// use rand::Rng; (Old depricated version)
use rand::{Rng, rngs::ThreadRng};
use std::io;

#[derive(Debug)]
// enum => it is used to create a new type by selecting one of the predefined constant values
enum Status {
    ACTIVE,
    INACTIVE,
}

fn main() {
    // by default immutable hota h variable. 
    let x = 5;
    println!("The value of x is {}", x);

    // yadi hume kisi variable ko mutable bnana h toh hume mut keyword use krna hoga.
    let mut y = 10;
    println!("y is: {}", y);

    y = 20;
    println!("y is now: {}", y);

    // Data Types
    
    // 1) Integers
    let num:u8 = 5;
    println!("num: {}", num);

    // program 1) Swap 2 numbers
    let mut a:u8 = 5;
    let mut b:u8 = 10;
    let c:u8;
    println!("a before swap = {}", a);
    println!("b before swap = {}", b);
    c = a;
    a = b;
    b = c;
    println!("a after swap = {}", a);
    println!("b after swap = {}", b);


    // generate random number using rand package between 1 to 10.
    // if we use old depricated version
    // let random_number = rand::thread_rng().gen_range(1..=10);
    // println!("Generate Random Number between 1 and 10: {}", random_number);
    
    // if we use new version
    let mut rng: ThreadRng = rand::rngs::ThreadRng::default();
    let random_number = rng.random_range(1..=10);
    println!("Generate Random Number between 1 and 10: {}", random_number);

    // String Literal => &str (by default)
    let string_literal = "Hey, Yuvraj";
    println!("By default string literal: {}", string_literal);
    let str_literal: &str = "Hi, Yuvi";
    println!("String literal using &str: {}", str_literal);

    // String
    // method-1)
    let str1 = String::from("Hey, UV");
    println!("String1: {}", str1);
    // method-2)
    let str2: String = String::from("Hey, thunder");
    println!("String2: {}", str2);

    // Tuple
    let emp_info: (u8, &str) = (18, "Yuvraj");
    let emp_roll_no = emp_info.0;
    let emp_name = emp_info.1;
    println!("Student roll no: {}", emp_roll_no);
    println!("Student name: {}", emp_name);

    // destructing => make a tuple and extract value from that tuple
    let tup: (u64, String, &str) = (210010130018, String::from("Yuvi"), "A");
    let (r_no1, name1, batch1) = tup;
    println!("Roll No of first student: {}", r_no1);
    println!("name of first student: {}", name1);
    println!("Batch of first student: {}", batch1);

    // ---------------------User input-------------------- 

    // 1) Basic string input
    let mut input1 = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read");
    let name = input1.trim();
    println!("Hello, {}", name);

    // 2) Take Integer as Input(i32, u32 etc)
    let mut input2 = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read");
    let num: i32 = input2.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", num);

    // 3) Take Float input (f64, f32)
    let mut input3 = String::new();
    println!("Enter a decimal no:");
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read");

    let float_num: f64 = input3.trim().parse().expect("Invalid float");
    println!("You entered: {}", float_num);

    // 4) Input Mutable Values (space-separated)
    let mut input4 = String::new();
    println!("Enter two numbers separated by space:");
    io::stdin()
        .read_line(&mut input4)
        .expect("Failed to read");
    let parts: Vec<i32> = input4
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    if parts.len() == 2 {
        println!("Sum: {}", parts[0] + parts[1]);
    }
    else{
        println!("Enter exactly two number");
    }

    // 5) Input a character
    let mut input5 = String::new();
    println!("Enter a character: ");
    io::stdin()
        .read_line(&mut input5)
        .expect("Failed to read");

    let ch = input5.trim().chars().next().expect("No input");
    println!("Character: {}", ch);

    // Match => Similar to switch in other languages. But we didn't require the break statement.

    // case 1) when variable expression is integer
    let day: u8 = 3;
    match day {
        1 => println!("MONDAY"),
        2 => println!("TUESDAY"),
        3 => println!("WEDNESDAY"),
        4 => println!("THURSDAY"),
        5 => println!("FRIDAY"),
        6 => println!("SATURDAY"),
        7 => println!("SUNDAY"),
        _ => println!("INVALID DAY"),
    }

    // case 2) when variable expression is string
    let str = String::from("Welcome");
    // let str = "Welcome";

    // this line gives an error because String and literal type are not matched in rust.
    // as_str() => it will convert the String(heap allocated string) into a &str(string slice)
    // match str {
    match str.as_str() {
        "Welcome" => {
            println!("Matched");
        }
        _ => {
            println!("Not matched");
        }
    }


    let active = Status::ACTIVE;
    let inactive = Status::INACTIVE;
    println!("{:?}", active);
    println!("{:?}", inactive);

    // call function
    print_value();

    // passing arguments
    print_value5(5);

    // return value
    let a: u8 = ret_sum(5, 6);
    println!("a = {}", a);

}

fn print_value() {
    println!("Hello")
}

fn print_value5(val:u8){
    println!("{}", val);
}

fn ret_sum(a:u8, b:u8) -> u8 {
   return a + b;
}
