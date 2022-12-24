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

    // ownership - 소유권
    // & = reference

    let country = String::from("대한민국");
    let ref_one = &country;
    let ref_two = &country;

    println!("Country is: {}", ref_one);
    /*
    fn return_it() -> &'static String {
        let country = String::from("대한민국");
        &country  // return &String
    }

    let my_country = return_it();
    */

    // & immutable reference / shared reference
    // &mut mutable reference / unique reference

    let mut my_number = 9;
    let num_ref = &mut my_number;

    *num_ref = 10;
    
    println!("Number is now {}", my_number);

    // shadowing
    let country = "Korean";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);

    // OWNERSHIP
    // move semantics
    fn print_country(country_name: &String){
        println!("My country is {}", country_name);
    }

    let country = "Korean".to_string();
    print_country(&country);
    print_country(&country);
    print_country(&country);

    // mutable references and mut in functions
    fn add_is_great(country_name: &mut String){
        country_name.push_str(" is great");
        println!("Now it says: {}", country_name);
    }

    let mut my_country = "Korean".to_string();
    add_is_great(&mut my_country);  // by mutable reference
    add_is_great(&mut my_country);  // by mutable reference

    fn add_is_great2(mut country_name: String) -> String {  // take by value, declare as mutable
        country_name.push_str(" is great");
        println!("Now it says: {}", country_name);
        country_name
    }

    let my_country2 = "Korean".to_string();
    add_is_great2(my_country2);

    // It's trivial to copy the bytes
    // Ownership and copy types
    fn prints_number(number: i32) {
        println!("{}", number);
    }

    fn prints_string(input: String) {
        println!("{}", input);
    }

    // copy - copy types
    // clone - non-copy types

    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Korea".to_string();
    prints_string(my_country.clone());
    prints_string(my_country);

    // uninitialzed variable
    // control flow

    // possibly uninitialized = maybe doesn't have a value yet    

    fn loop_then_return(mut counter: i32) -> i32 {
        loop {
            counter += 1;
            if counter % 50 == 0 {
                break;
            }
        }
        counter
    }

    let my_number;

    {
        // 복잡한 것들
        let x = loop_then_return(43);
        my_number = x
    }

    println!("{}", my_number);

    // Collection types
    // Array

    // buffer
    let array = ["One", "Two"];
    let array2 = ["One", "Twoos"];
    let array3 = [0; 5];
    println!("Is array the same as array2? {}", array == array2);
    // array.ffwewf();   type 확인가능
    println!("{:?}", array3);
    println!("{:?}", array2[1]);
    println!("{:?}", array2.get(3));
    // Some None Option enum

    // Slices
    // Vecs

    // dynamically sized type

    let seasons = ["봄", "여름", "가을", "겨울", "봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[0..2]);
    println!("{:?}", &seasons[0..=2]);
    println!("{:?}", &seasons[..]);
    println!("{:?}", &seasons[..=4]);

    // Vec = Vector
    // Vec<String>
    // Vec<u8>
    // T = some type
    let name1 = String::from("WIndy");
    let name2 = String::from("Gomesy");
    
    let mut my_vec = Vec::new();
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name1.clone());
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name2.clone());
    println!("Space for my_vec: {}", my_vec.capacity());

    println!("My cats are {:?}", my_vec);  // :? = debug print

    let my_vec2 = vec![name1, name2];  // vec! macro
    println!("My cats are {:?}", my_vec2);  // :? = debug print

    // trait = 초능력

    // From, Into
    let my_name = String::from("Noah");
    let my_city: String = "Seoul".into();

    println!("{}", my_city);

    let my_vec3 = Vec::from([8,9,10]); // [i32; 3]

}
    