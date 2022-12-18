use std::mem::size_of;

fn main() {
    // one line comment
    /* inside comment */
    println!("Hello, world!");

    //integer
    let a :u8 = 100;
    let b = 50;
    let c = a + b;

    //char and string
    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16; // simple casting

    println!("Size of a char: {} bytes", size_of::<char>());
    println!("Size of a string : {}", "a".len());
    println!("Size of a string : {}", "abcde".len());

    let slice = "Hello";
    println!("Slice is {} bytes, {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice is {} bytes, {} characters.", slice2.len(), slice2.chars().count());


}
    