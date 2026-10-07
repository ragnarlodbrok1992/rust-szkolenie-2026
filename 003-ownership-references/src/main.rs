fn main() {
    strings();
    ownership_rules();
    move_copy_clone();
    ownership_and_functions();
    references();
    mutable_references();
    lifetimes();
}

fn strings() {
    println!("--- String and &str ---");

    let number = 5;
    let literal: &str = "hello";
    let mut owned = String::from("hello");
    owned.push_str(", world");
    println!("{number}");
    println!("{literal} / {owned} / {}", owned.len());
}

fn ownership_rules() {
    println!("--- Ownership rules ---");

    {
        let text = String::from("hello");
        println!("{text}");
    }
    // println!("{text}");
}

fn move_copy_clone() {
    println!("--- Move, copy, clone ---");

    let first = String::from("hello");
    let second = first;
    println!("{second}");
    // println!("{first} {second}");

    let x = 5;
    let y = x;
    println!("{x} {y}");

    let original = String::from("hello");
    let copy = original.clone();
    println!("{original} {copy}");
}

fn ownership_and_functions() {
    println!("--- Ownership and functions ---");

    let text = String::from("hello");
    take_ownership(text);
    // println!("{text}");

    let returned = give_back(String::from("hi"));
    println!("{returned}");
}

fn take_ownership(value: String) {
    println!("{value}");
}

fn give_back(value: String) -> String {
    value
}

fn references() {
    println!("--- References ---");

    let text = String::from("hello");
    let borrowed = &text;
    println!("{borrowed} {text}");

    let length = length_of(&text);
    println!("{text} has {length} bytes");
    println!("{}", length_of("a literal works too"));
}

fn length_of(value: &str) -> usize {
    value.len()
}

// fn add_world_read_only(value: &String) {
//     value.push_str(", world");
// }

fn mutable_references() {
    println!("--- Mutable references ---");

    let mut text = String::from("hello");
    add_world(&mut text);
    println!("{text}");

    let reader = &text;
    println!("{reader}");
    let writer = &mut text;
    writer.push_str(", again");
    println!("{text}");

    // let first = &mut text;
    // let second = &mut text;
    // println!("{first} {second}");

    // let a = &text;
    // let b = &mut text;
    // println!("{a} {b}");
}

fn add_world(value: &mut String) {
    value.push_str(", world");
}

// fn make_greeting() -> &String {
//     let greeting = String::from("hello");
//     &greeting
// }

fn lifetimes() {
    println!("--- Lifetimes ---");

    // let reference;
    // {
    //     let text = String::from("hello");
    //     reference = &text;
    // }
    // println!("{reference}");

    let outer = String::from("long text");
    let short_text = String::from("short");
    println!("{}", longer(&outer, &short_text));
    println!("{}", longer("hi", "hello"));

    // let result;
    // {
    //     let inner = String::from("short");
    //     result = longer(&outer, &inner);
    // }
    // println!("{result}");

    println!("[{}]", trimmed("  padded  "));

    let greeting: &'static str = "hello";
    println!("{greeting}");
}

// fn longer_without_lifetimes(first: &str, second: &str) -> &str {
//     if first.len() >= second.len() {
//         first
//     } else {
//         second
//     }
// }

fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

fn trimmed(text: &str) -> &str {
    text.trim()
}
