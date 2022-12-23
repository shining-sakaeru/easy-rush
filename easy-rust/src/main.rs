use std::{mem::size_of};

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


    // functions
    fn give_number(one: i32, two: i32) -> i32 {
        let multiplied_by_ten =  {
            let first_number = 10;
            first_number * one * two
        };
        multiplied_by_ten
    }
    
    let my_number = give_number(9, 1);
    println!("{}", my_number);

    // mutability
    // shadowing

    let mut _my_number = 10;
    _my_number = 9;

    let my_variable = 10;
    println!("{}", my_variable);
    {
        let my_variable = "My variable";
        println!("{}", my_variable);  // only use in code blocks
    }
    println!("{}", my_variable);

    // fancy printing
    println!(r#"C:\mydrive"#);
    println!(
"many
stories
exist"
    );

    let p_variable = &9;
    println!("{:p}", p_variable);
    let h_variable = 9000;
    println!("{:X}", h_variable);
    let b_variable = 9000;
    println!("{:b}", b_variable);

    let title = "Today's news";
    println!("{:-^30}", title);  // formatting
    
    // string and &str
    let my_name = "Noah".to_string(); // string
    let other_name = String::from("Noah2");
    // growable + shrinkable
    let mut my_other_name = "Noah3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);

    // string method
        // .capacity
        // .push
        // .push_str
        // .pop
        // with_capacity
        // allocation
    let mut my_name =  String::with_capacity(26);
    println!("Length {} capa {}", my_name.len(), my_name.capacity());
    my_name.push_str("Noah!");
    println!("Length {} capa {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul ");
    println!("Length {} capa {}", my_name.len(), my_name.capacity());
    my_name.push('a');
    println!("Length {} capa {}", my_name.len(), my_name.capacity());

    // const
    // static
    const HIGH_SCORE: i32 = 20;  // global scopei
    static mut LOW_SCORE: i32 = 0;  // unsafe
    // 'static lifetime


    fn print_high_score() {
        println!("The high score is {}", HIGH_SCORE);
    }

    print_high_score();

    let my_name = "Noah"; // &'static str
    unsafe {LOW_SCORE = 1};  // 비추천 다른방법 찾아보자














}
    