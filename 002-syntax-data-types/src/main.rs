fn main() {
    syntax_basics();
    scalar_types();
    types_in_practice();
    compound_types();
    functions();
}

fn syntax_basics() {
    println!("--- Syntax basics ---");

    let greeting = "Hello";
    println!("{greeting}, Rust!");

    // let diceRoll = 6;
    // println!("{diceRoll}");

    let age = 30;
    let height: f64 = 1.75;
    let name = "Ferris";
    // age = 31;

    println!("{}", age);
    println!("{age} years, {height} m");
    println!("{:?}", (age, height));
    println!("{name}");
}

fn scalar_types() {
    println!("--- Scalar types ---");

    println!("i8: {} to {}", i8::MIN, i8::MAX);
    println!("u8: {} to {}", u8::MIN, u8::MAX);
    println!("i32: {} to {}", i32::MIN, i32::MAX);

    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    let with_suffix = 42u8;
    let thousand = 1_000_i64;
    println!("{decimal} {hexadecimal} {octal} {binary} {byte} {with_suffix} {thousand}");

    let price = 2.5;
    let ratio: f32 = 0.75;
    println!("{price} {ratio}");
    println!("{}", 0.1 + 0.2);

    let is_ready = true;
    let has_errors: bool = false;
    println!("{is_ready} {has_errors}");

    let letter = 'R';
    let digit: char = '7';
    let crab = '🦀';
    println!("{letter} {digit} {crab}");
    // let letter: char = "a";
}

fn types_in_practice() {
    println!("--- Types in practice ---");

    let count = 10;
    let average = 2.5;
    let small: u8 = 10;
    let tiny = 10_u8;
    println!("{count} {average} {small} {tiny}");
    // let whole: i32 = 3.5;
    // let total = 5 + 2.5;

    println!("7 + 2 = {}", 7 + 2);
    println!("7 - 2 = {}", 7 - 2);
    println!("7 * 2 = {}", 7 * 2);
    println!("7 / 2 = {}", 7 / 2);
    println!("-7 / 2 = {}", -7 / 2);
    println!("7 % 2 = {}", 7 % 2);
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    let whole = 7;
    let half = whole as f64 / 2.0;
    let truncated = 3.99_f64 as i32;
    let big: i32 = 300;
    let wrapped = big as u8;
    let letter_code = 'A' as u8;
    println!("{half} {truncated} {wrapped} {letter_code}");

    // let x: u8 = 256;
    // let x: u8 = 255 + 1;
    println!("{}", add_one(41));
    // println!("{}", add_one(255));

    println!("{}", 250_u8.wrapping_add(10));
    println!("{}", 250_u8.saturating_add(10));
    println!("{:?}", 250_u8.checked_add(3));
    println!("{:?}", 250_u8.checked_add(10));
}

fn add_one(value: u8) -> u8 {
    value + 1
}

fn compound_types() {
    println!("--- Compound types ---");

    let person: (&str, i32, f64) = ("Alice", 30, 1.68);
    println!("{}", person.0);
    println!("{}", person.1);
    println!("{:?}", person);
    // println!("{}", person.3);

    let primes = [2, 3, 5, 7, 11];
    let zeros = [0; 5];
    let first: i32 = primes[0];
    println!("{} {}", first, primes.len());
    println!("{:?}", zeros);
    // println!("{}", primes[10]);
    print_element(primes, 2);
    // print_element(primes, 10);

    let name = "Ferris";
    let greeting: &str = "Hello, world!";
    let initial = 'F';
    println!("{greeting} {name} {initial}");
}

fn print_element(numbers: [i32; 5], position: usize) {
    println!("{}", numbers[position]);
}

fn functions() {
    println!("--- Functions ---");

    print_sum(3, 4);
    println!("{}", square(5));
    println!("{}", square_with_return(5));

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let nothing = print_sum(3, 4);
    println!("{:?}", nothing);
}

fn print_sum(first: i32, second: i32) {
    println!("{first} + {second} = {}", first + second);
}

fn square(number: i32) -> i32 {
    number * number
}

fn square_with_return(number: i32) -> i32 {
    return number * number;
}

// fn square_with_semicolon(number: i32) -> i32 {
//     number * number;
// }
