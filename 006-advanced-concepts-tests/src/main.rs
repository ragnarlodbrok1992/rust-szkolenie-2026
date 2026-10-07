use std::fmt;
use std::num::ParseIntError;

fn largest_i32(items: &[i32]) -> &i32 {
    let mut largest = &items[0];
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest_without_bound<T>(items: &[T]) -> &T {
//     let mut largest = &items[0];
//     for item in items {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut largest = &items[0];
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generics() {
    println!("--- Generics ---");

    let numbers = [34, 50, 25, 100, 65];
    let letters = ['y', 'm', 'a', 'q'];
    println!("{}", largest_i32(&numbers));
    println!("{} {}", largest(&numbers), largest(&letters));

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("{} {}", integer_point.x(), integer_point.y);
    println!("{} {}", float_point.x(), float_point.y);
    // let mixed = Point { x: 5, y: 4.0 };
}

trait Describe {
    fn describe(&self) -> String;

    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

struct Dog {
    name: String,
}

impl Describe for Dog {
    fn describe(&self) -> String {
        format!("{} the dog", self.name)
    }
}

enum Weather {
    Sunny,
    Rainy,
}

impl Describe for Weather {
    fn describe(&self) -> String {
        match self {
            Weather::Sunny => String::from("a sunny day"),
            Weather::Rainy => String::from("a rainy day"),
        }
    }
}

// struct Robot;
// impl Describe for Robot {}

fn print_description(item: &impl Describe) {
    println!("{}", item.describe());
}

fn print_twice<T: Describe>(item: &T) {
    println!("{} / {}", item.describe(), item.shout());
}

fn print_and_copy<T>(item: &T) -> T
where
    T: Describe + Clone,
{
    println!("{}", item.describe());
    item.clone()
}

#[derive(Clone)]
struct Note {
    text: String,
}

impl Describe for Note {
    fn describe(&self) -> String {
        format!("note: {}", self.text)
    }
}

struct Temperature {
    degrees: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°C", self.degrees)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    volume: u8,
    muted: bool,
}

fn traits() {
    println!("--- Traits ---");

    let dog = Dog {
        name: String::from("Rex"),
    };
    println!("{}", dog.describe());
    println!("{}", Weather::Rainy.shout());
    // println!("{dog}");

    print_description(&dog);
    print_description(&Weather::Sunny);
    print_twice(&Weather::Rainy);
    let note = Note {
        text: String::from("buy milk"),
    };
    let copy = print_and_copy(&note);
    println!("{}", copy.text);

    let temperature = Temperature { degrees: 21.456 };
    println!("{temperature}");
    let as_text = temperature.to_string();
    println!("{as_text}");

    let original = Settings::default();
    let copy = original.clone();
    println!("{copy:?} {}", copy == original);

    let items: Vec<Box<dyn Describe>> = vec![
        Box::new(Dog {
            name: String::from("Rex"),
        }),
        Box::new(Weather::Rainy),
    ];
    for item in &items {
        println!("{}", item.describe());
    }
}

#[derive(Debug)]
enum AgeError {
    NotANumber,
    TooOld(u32),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber => write!(f, "age must be a number"),
            AgeError::TooOld(age) => write!(f, "{age} is too old"),
        }
    }
}

fn parse_age(input: &str) -> Result<u32, AgeError> {
    let age = input.parse::<u32>().map_err(|_| AgeError::NotANumber)?;
    if age > 150 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}

fn add_numbers(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let a = first.parse::<i32>()?;
    let b = second.parse::<i32>()?;
    Ok(a + b)
}

fn error_handling() {
    println!("--- Error handling ---");

    let inputs = ["42", "4x2"];
    for input in inputs {
        match input.parse::<i32>() {
            Ok(number) => println!("number: {number}"),
            Err(error) => println!("error: {error}"),
        }
    }

    // let number = "4x2".parse::<i32>().unwrap();
    // let age = "4x2".parse::<i32>().expect("age must be a number");

    println!("{:?}", add_numbers("2", "40"));
    println!("{:?}", add_numbers("2", "x"));

    for input in ["30", "abc", "200"] {
        match parse_age(input) {
            Ok(age) => println!("age: {age}"),
            Err(error) => println!("error: {error}"),
        }
    }

    let empty: Option<i32> = None;
    let as_result: Result<i32, &str> = empty.ok_or("no value");
    println!("{as_result:?}");
    let as_option: Option<u32> = parse_age("abc").ok();
    println!("{as_option:?}");
}

fn main() {
    generics();
    traits();
    error_handling();
    // let number = "42".parse::<i32>()?;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_finds_biggest_number() {
        assert_eq!(*largest(&[3, 9, 4]), 9);
    }

    #[test]
    fn parse_age_accepts_valid_age() {
        assert_eq!(parse_age("30").unwrap(), 30);
    }

    #[test]
    fn parse_age_rejects_text() {
        assert!(matches!(parse_age("abc"), Err(AgeError::NotANumber)));
    }

    #[test]
    fn parse_age_rejects_age_over_150() {
        let input = "200";

        let result = parse_age(input);

        assert!(result.is_err(), "expected an error for 200, got {result:?}");
    }

    #[test]
    fn temperature_displays_one_decimal() {
        let temperature = Temperature { degrees: 21.456 };
        assert_ne!(temperature.to_string(), "21.456°C");
        assert_eq!(temperature.to_string(), "21.5°C");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn largest_of_empty_slice_panics() {
        largest::<i32>(&[]);
    }

    #[test]
    fn adds_valid_numbers() -> Result<(), ParseIntError> {
        assert_eq!(add_numbers("2", "40")?, 42);
        Ok(())
    }

    // #[test]
    // fn largest_of_letters_is_wrong_on_purpose() {
    //     assert_eq!(*largest(&['a', 'z', 'k']), 'k');
    // }
}
