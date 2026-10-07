use std::collections::HashMap;
use std::f64::consts::PI;

fn main() {
    destructuring();
    slices();
    string_slices();
    vectors();
    strings();
    closures();
    iterators();
    hash_maps();
    structs();
    methods();
    lifetimes_in_structs();
    enums();
    options();
}

fn destructuring() {
    println!("--- Destructuring ---");

    let person = ("Alice", 30, 1.68);
    let (name, age, _) = person;
    println!("{name} is {age}");
    // let (name, age) = person;

    let (smallest, largest) = min_max(9, 4);
    println!("{smallest} {largest}");
}

fn min_max(first: i32, second: i32) -> (i32, i32) {
    if first < second {
        (first, second)
    } else {
        (second, first)
    }
}

fn slices() {
    println!("--- Slices ---");

    let primes = [2, 3, 5, 7, 11];
    let middle = &primes[1..4];
    println!("{middle:?} {}", middle.len());
    println!("{:?} {:?} {:?}", &primes[..2], &primes[3..], &primes[..]);

    println!("{}", sum(&primes));
    println!("{}", sum(&primes[..2]));
    // println!("{:?}", &primes[2..10]);
}

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}

fn string_slices() {
    println!("--- String slices ---");

    let text = String::from("hello world");
    let hello = &text[0..5];
    let world = &text[6..];
    println!("{hello} | {world}");
}

fn vectors() {
    println!("--- Vec ---");

    let mut scores = Vec::new();
    for round in 1..=3 {
        scores.push(round * 10);
    }
    let mut letters = vec!['a', 'b', 'c'];
    let last = letters.pop();
    println!("{scores:?} {letters:?} {last:?}");

    println!("{}", scores[1]);
    println!("{:?}", scores.get(1));
    println!("{:?}", scores.get(10));
    // println!("{}", scores[10]);

    for score in &scores {
        print!("{score} ");
    }
    for score in &mut scores {
        *score += 1;
    }
    println!("{scores:?}");
    println!("{}", sum(&scores));

    let mut numbers = vec![1, 2, 3];
    println!("len {} capacity {}", numbers.len(), numbers.capacity());
    // let first = &numbers[0];
    numbers.push(4);
    // println!("{first}");
    println!("len {} capacity {}", numbers.len(), numbers.capacity());
}

fn strings() {
    println!("--- String ---");

    let mut greeting = String::from("Hello");
    greeting.push(',');
    greeting.push_str(" world");
    let full = format!("{greeting}!");
    println!("{full}");

    let word = "zażółć";
    println!("{} {}", word.len(), word.chars().count());
    for letter in word.chars() {
        print!("{letter} ");
    }
    println!();
    // let first = word[0];
    // println!("{}", &word[0..3]);
}

fn closures() {
    println!("--- Closures ---");

    let double = |x| x * 2;
    let add = |a: i32, b: i32| a + b;
    let offset = 10;
    let shift = |x| x + offset;
    println!("{} {} {}", double(4), add(2, 3), shift(1));
}

fn iterators() {
    println!("--- Iterators ---");

    let numbers = [1, 2, 3, 4];
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    let total: i32 = numbers.iter().sum();
    let words = ["apple", "fig", "banana"];
    let long_count = words.iter().filter(|word| word.len() > 3).count();
    println!("{doubled:?} {total} {long_count}");
    for (index, word) in words.iter().enumerate() {
        println!("{index}: {word}");
    }

    // numbers.iter().map(|n| n * 2);
    // let doubled = numbers.iter().map(|n| n * 2).collect();
}

fn hash_maps() {
    println!("--- HashMap ---");

    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
    ages.insert("Bob", 25);
    println!("{:?}", ages.get("Alice"));
    println!("{:?}", ages.get("Carol"));
    // println!("{}", ages["Carol"]);

    let sentence = "the cat and the hat";
    let mut counts = HashMap::new();
    for word in sentence.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{}", counts["the"]);
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

struct Meters(f64);
struct Seconds(f64);

fn structs() {
    println!("--- Structs ---");

    let user = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("{} {}", user.name, user.age);
    // user.age += 1;
    println!("{user:?}");
    println!("{user:#?}");

    let mut changed = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    changed.age += 1;
    println!("{} {}", changed.age, changed.active);

    let built = new_user(String::from("Bob"), 25);
    println!("{} {}", built.name, built.age);

    let distance = Meters(5.0);
    let time = Seconds(2.0);
    println!("{}", distance.0 / time.0);
    // println!("{:?}", distance);
}

fn new_user(name: String, age: u32) -> User {
    User {
        name,
        age,
        active: true,
    }
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: String::from(name),
            age,
            active: true,
        }
    }

    fn greeting(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn into_name(self) -> String {
        self.name
    }
}

fn methods() {
    println!("--- Methods ---");

    let mut user = User::new("Alice", 30);
    user.birthday();
    println!("{} ({})", user.greeting(), user.age);

    let name = user.into_name();
    println!("{name}");
}

struct Excerpt<'a> {
    text: &'a str,
}

// struct ExcerptWithoutLifetime {
//     text: &str,
// }

fn lifetimes_in_structs() {
    println!("--- Lifetimes in structs ---");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = &novel[..16];
    let excerpt = Excerpt {
        text: first_sentence,
    };
    println!("{}", excerpt.text);
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Empty,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Empty => 0.0,
        }
    }
}

// fn area_without_empty(shape: &Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => PI * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

fn enums() {
    println!("--- Enums ---");

    let shapes = [Shape::Circle(1.0), Shape::Rectangle(2.0, 3.0), Shape::Empty];
    for shape in &shapes {
        println!("{:.2}", shape.area());
    }
}

fn options() {
    println!("--- Option ---");

    let values = [10, 20, 30];
    match values.get(5) {
        Some(value) => println!("found {value}"),
        None => println!("nothing there"),
    }

    if let Some(first) = values.first() {
        println!("first: {first}");
    }
    let bonus = 250_u8.checked_add(10).unwrap_or(0);
    let mut letters = vec!['a', 'b'];
    let fallback = letters.pop().unwrap_or('?');
    println!("bonus: {bonus}  fallback: {fallback:?}");
    // println!("{}", values.get(5).unwrap());
}
