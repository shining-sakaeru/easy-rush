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
    //let third_number = my_number + second_number as u16; // simple casting

    println!("Size of a char: {} bytes", size_of::<char>());
    println!("Size of a string : {}", "a".len());
    println!("Size of a string : {}", "abcde".len());

    let slice = "Hello";
    println!("Slice is {} bytes, {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice is {} bytes, {} characters.", slice2.len(), slice2.chars().count());

    //float
    //let float_number = 9_u8;
    let my_num_f = 9.6;
    let other_num_f = 9;
    println!("{}", my_num_f + other_num_f as f64);

    //println!
    fn give_age() -> i32 {
        36
    }

    let my_name = "Noah";
    //let my_age = 36;
    println!("My name is {} and my age is {}", my_name, give_age());

    // string interpolation

    // expression
    // 1 + 5

    let city = "Seoul";
    let year = 2002;
    let population = 9_987_987;
    println!("The city of {city} in {year} had a population of {population}. I love {city}!");

    // () - empty tuple, unit type (void)
    // expression-base language
    fn empty_tuple() {

    }

    // Display {}
    // Debug print
    let tuple = empty_tuple();
    println!("{:?}",tuple);


}
    