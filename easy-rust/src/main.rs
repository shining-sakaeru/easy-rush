use core::num;
use std::{mem::size_of};

fn main() {
    // one line comment
    /* inside comment */
    println!("Hello, world!");

    //integer
    let a :u8 = 100;
    let b = 50;
    let _c = a + b;

    //char and string
    let _my_number: u16 = 8;
    let _second_number: u8 = 10;
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
    let _my_name = "Noah".to_string(); // string
    let _other_name = String::from("Noah2");
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

    let _my_name = "Noah"; // &'static str
    unsafe {LOW_SCORE = 1};  // 비추천 다른방법 찾아보자

    // ownership - 소유권
    // & = reference

    let country = String::from("대한민국");
    let ref_one = &country;
    let _ref_two = &country;

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
    let _my_name = String::from("Noah");
    let my_city: String = "Seoul".into();

    println!("{}", my_city);

    let _my_vec3 = Vec::from([8,9,10]); // [i32; 3]

    // tuples
    // Vec<String>

    let my_tuple = ("Noah", 8, vec![8, 9, 10], 'b', [8, 9, 10], 7.7);
    println!(
        "
        First item: {}
        Second item: {}
        Third item: {:?}
        Fourth item: {}
        Fifth item: {:?}
        Sixth item: {}
        ",
        my_tuple.0,
        my_tuple.1,
        my_tuple.2,
        my_tuple.3,
        my_tuple.4,
        my_tuple.5,
    );

    // Vec<(&str, i32)>
    // Destructuring
    // Structure

    let _my_vec = vec![("Hey", 9), ("Hello", 23)];

    let str_tuple = ("one", "two", "three");
    let (a, _b, _c) = str_tuple;
    println!("Item is: {}", a);
 
    let str_array = ["one", "two", "three"];
    let [a, _b, _c] = str_array;
    println!("Item is: {}", a);

    // Control flow

    let my_number = 5;
    let my_second_number = 10;
    if my_number == 5 && my_second_number == 10 {
        println!("It's same");
    } else if my_number == 6 {
        println!("It's six");        
    } else {
        println!("It's a diff. number");
    }

    // rust style
    // expression-based language
    let my_number: u8 = 5;

    let second_number = match my_number {
        0 => 23,
        1 => 65,
        5 => 99,
        _ => 0 // _ "I don't care"
    };

    println!("The second number is: {}", second_number);

    // Match statements
    let sky = "cloudy";  // &str
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's note very nice"),
        ("clear", "warm") => println!("It's quite good"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure for today")
    }


    let children = 5;
    let married = true;
    
    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Something other")
    }

    // rgb

    fn match_colours(rbg: (u32, u32, u32)) {
        match rbg {
            (r, _, _) if r < 10 => println!("Not much red"),
            (_, b, _) if b < 10 => println!("Not much blue"),
            (_, _, g) if g < 10 => println!("Not much green"),
           _ => println!("Every colour is at least 10")
        }
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
    


    let my_number = 10;
    let some_variable = match my_number {
        10 => 8,
        _ => 000
    };
    println!("{}", some_variable);


    fn match_number(input: i32) {
        match input {
            number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),
            _ => println!("It's greater than ten")
        }
    }

    match_number(5);
    match_number(11);

    // structs

    // unit struct
    struct FileDirectory;  // byte를 사용하지 않음

    // tuple struct
    #[derive(Debug)]  // attribute
    struct Colour(u8, u8, u8);

    // named struct
    use std::mem::size_of_val;

    #[derive(Debug)]  // attribute
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
        all_populations: [u32; 5500]
    }


    fn takes_file_directory(_input: FileDirectory) {
        println!("I got a file directory");
    }

    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));

    let my_colour = Colour(20, 50, 100);
    println!("The second colour is {}", my_colour.1);
    println!("The second colour is {:?}", my_colour);
    /*    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudearu".to_string(),
        all_populations: [5499; 5500]
    };
    

    println!("The population is: {}\nThe capital is: {}", canada.population, canada.capital);

    println!("The country is: {:#?}", canada);
    */

    let population = 35_000_000;
    let capital = "Otawwa".to_string();
    let leader_name = "Justin".to_string();


    let my_country = Country {
        population,
        capital,
        leader_name,
        all_populations: [5490; 5500]
    };

    println!("Country is {} bytes in size", size_of_val(&my_country));

    struct Numbers {
        one: u8,
        two: u8,
        three: u8,
        four: u32
    }

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
        four: 30
    };

    println!("Size is: {}", size_of_val(&numbers));
    println!("Size is: {}", size_of_val(&my_country));
    
    // enum = enumeration
    // e = from + number

    // struct = and
    // enum = or
    
    enum ThingsInTheSky {
        Sun,  // 0
        Stars,  // 1
    }

    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Stars
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => println!("I can see the sun"),
            ThingsInTheSky::Stars => println!("I can see the stars"),
            
        }
    }

    let time = 20;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
    check_skystate(&create_skystate(12));

    // enums 2
    enum Mood {
        Happy,
        Sleepy,
        NotBad,
        Angry
    }

    fn match_mood(mood: &Mood) -> i32 {
        use Mood::*;  // pointer
        let happiness_level = match mood {
            Happy => 10,
            Sleepy => 6,
            NotBad => 7,
            Angry => 2,        
        };
        happiness_level
    }
    
    // same with upper function
    fn match_mood2(mood: &Mood) -> i32 {
        use Mood::*;  // pointer
        
        match mood {
            Happy => 10,
            Sleepy => 6,
            NotBad => 7,
            Angry => 2,        
        }
    }

    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    let my_mood2 = Mood::Happy;
    let happiness_level2 = match_mood(&my_mood2);
    println!("Out of 1 to 10, my happiness is {}", happiness_level2);

    enum Season {
        Spring,  // 0
        Summer,
        Autumn,
        Winter
    }

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];  // Vec<Season>
    for season in four_seasons {
        println!("The number is: {}", season as u32);
    }

    // enums 3
    enum Star {
        BrownDwarf = 10,
        RedDwarf = 50,
        YellowStar = 100,
        RedGiant = 1000,
        DeadStar
    }

    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedDwarf, DeadStar];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star: {}", size),
            size if size >= 80 => println!("Pretty big star: {}", size),
            _ => println!("Some other star")
        }
    }

    println!("What about DeadStar? It is: {}", DeadStar as u32);


    enum Number {
        U32(u32),
        I32(i32)
    }

    fn get_number(input: i32) -> Number {
        match input.is_positive() {
            true => Number::U32(input as u32),
            false => Number::I32(input),
        }
    }

    let my_vec = vec![get_number(-800), get_number(8)];  // Vec<Number>

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 value: {}", number),
            Number::I32(number) => println!("It's a i32 value: {}", number),
            
        }
    }

    // loops
    let mut counter = 0;
    let mut counter2 = 0;
    
    'first_loop: loop {  // ' tick
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            println!("Entering second loop");

            'second_loop: loop {
                println!("The second counter is: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }

    // while
    let mut counter = 0;

    while counter != 5 {
        counter += 1;
        println!("The counter is now: {}", counter);
    }

    // for
    for number in 0..3 {  // .. exclusive range, ..= inclusive range
        println!("The number is {}", number);
    }

    // break
    let mut counter = 5;

    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("my_number is now: {}", counter);

    // impl blocks

    #[derive(Debug)]
    struct Animal {
        age: u8,
        animal_type: AnimalType
    }
    
    #[derive(Debug)]
    enum AnimalType {
        Cat,
        Dog
    }

    impl Animal {
        fn new_cat(age: u8) -> Self {  // Self = Animal
            
            Self {
                age,
                animal_type: AnimalType::Cat
            }
        }
    }

    let my_animal = Animal::new_cat(10); 

    println!("I made a: {:?}", my_animal);












}
    