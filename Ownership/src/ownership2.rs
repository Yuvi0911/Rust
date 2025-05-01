pub fn ownership2() {
    let s1: String = get_string(); // s1 becomes the owner of hello
    println!("s1 = {}", s1);

}

fn get_string() -> String {
    let new_string = String::from("hello");
    return new_string;
}
