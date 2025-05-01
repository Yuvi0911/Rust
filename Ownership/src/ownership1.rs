
pub fn ownership1() {
    println!("Ownership");

    // by default str1 and str2 ka type string literal hoga. Is case me memory stack me allocate hoti h aur fixed hoti h.
    let str1 = "Hello";
    // str2 k liye ek nayi memory bnegi stack me. Is case me ownership transfer nhi hogi.
    // Ownership mainly heap allocated types pr focus krti h.
    let str2 = str1;

    println!("str1: {}", str1); //Hello
    println!("str2: {}", str2); //Hello

    // s1 aur s2 ko memory heap me allocate hogi.
    let s1:String = String::from("Yuvi");
    // is case me ownership transfer ho jaiye gi s1 se s2 pr. Ab Yuvi ka owner s1 se s2 bn jaiye ga.
    let s2:String = s1;

    // first println will give error because the ownership is transfered from s1 to s2 and there will be no s1 present into the memory because it is removed by the rust compiler.
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // clone method => it make the deep copy of the heap data.
    let d1:String = String::from("UV");
    // is case me ek clone ban jaye ga, ownership transfer nhi hogi.
    let d2 = d1.clone();
    println!("d1: {}", d1);
    println!("d2: {}", d2); 


    // Ownership and functions ------------------------
    let x:String = String::from("Yuvraj"); // x is the owner of Yuvraj
    // x se ownership le kr y k paas chli gyi h aur rust ab x ko hta dega heap se.
    process_string(x); // Transfer of ownership
    // ab hum x ko access krege toh error aa jaiye ga kyoki ownership transfer ho chuki h aur x memory se clear ho chuka h.
    // println!("x = {}", x);

}

fn process_string(x:String) {
    println!("x: {}", x);
}
