const MAX_POINTS: u32 = 100_000;
const SECONDS_IN_HOUR: u32 = 60 * 60;
// const BONUS_POINTS = 100_000;

fn main() {
    mutability();
    constants();
    shadowing();
    if_expressions();
    loops();
    matching();
    using_macros();
    formatting();
    writing_macros();
}

fn mutability() {
    println!("--- Mutability ---");

    let mut score = 0;
    println!("{score}");
    score = 10;
    score += 5;
    println!("{score}");

    // let mut total = 10;
    // println!("{total}");

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("{spaces}");
}

fn constants() {
    println!("--- Constants ---");

    println!("{MAX_POINTS} {SECONDS_IN_HOUR}");
}

fn shadowing() {
    println!("--- Shadowing ---");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let level = 1;
    {
        let level = level * 2;
        println!("inner: {level}");
    }
    println!("outer: {level}");
}

fn if_expressions() {
    println!("--- if expressions ---");

    let temperature = 18;
    if temperature < 10 {
        println!("cold");
    } else if temperature < 20 {
        println!("mild");
    } else {
        println!("warm");
    }

    let number = 3;
    if number != 0 {
        println!("number is not zero");
    }
    // if number {
    //     println!("number was three");
    // }

    let label = if temperature < 20 { "cool" } else { "warm" };
    println!("{label}");
    // let label = if temperature < 20 { "cool" } else { 20 };

    println!("{} {} {}", describe(-3), describe(0), describe(8));
}

fn describe(number: i32) -> &'static str {
    if number < 0 {
        return "negative";
    }
    if number == 0 {
        return "zero";
    }
    "positive"
}

fn loops() {
    println!("--- Loops ---");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("liftoff!");

    for number in 1..=3 {
        print!("{number} ");
    }
    for number in (1..=3).rev() {
        print!("{number} ");
    }
    let primes = [2, 3, 5, 7, 11];
    for prime in primes {
        print!("{prime} ");
    }
    println!();

    for number in 0..3 {
        print!("{number} ");
    }
    println!();

    for number in 1..=6 {
        if number % 2 == 0 {
            continue;
        }
        print!("{number} ");
    }
    println!();

    'outer: for row in 1..=3 {
        for column in 1..=3 {
            if row * column == 4 {
                break 'outer;
            }
            println!("{row} x {column} = {}", row * column);
        }
    }
}

fn matching() {
    println!("--- match ---");

    let dice = 4;
    let message = match dice {
        1 => "one",
        2 | 3 => "two or three",
        4..=6 => "four to six",
        _ => "not a dice value",
    };
    println!("{message}");

    // let message = match dice {
    //     1 => "one",
    //     2 | 3 => "two or three",
    //     4..=6 => "four to six",
    // };
}

fn using_macros() {
    println!("--- Using macros ---");

    // println!("{} {}", 1);

    let name = "Ferris";
    let greeting = format!("Hello, {name}!");
    eprintln!("warning: {greeting}");
    let doubled = dbg!(21 * 2);
    println!("{doubled}");

    println!("{}", next_level(1));
    // println!("{}", next_level(3));
    // println!("{}", next_level(7));
}

fn next_level(level: u32) -> u32 {
    match level {
        1 | 2 => level + 1,
        3 => todo!("the final level"),
        _ => unreachable!("only levels 1 to 3 exist"),
    }
}

fn formatting() {
    println!("--- Formatting ---");

    let short = "ab";
    println!("[{:>6}] [{:<6}] [{:^6}]", short, short, short);

    let measured = 9.87654;
    println!("{:.2}", measured);

    let first = "a";
    let second = "b";
    println!("{0} {1} {0}", first, second);

    let large = 255;
    let small = 5;
    println!("{:x} {:b}", large, small);

    let pair = (1, "two");
    println!("{:#?}", pair);

    println!("{{}}");
}

macro_rules! square {
    ($value:expr) => {
        $value * $value
    };
}

macro_rules! sum {
    ($($value:expr),*) => {
        0 $(+ $value)*
    };
}

fn writing_macros() {
    println!("--- Writing macros ---");

    println!("{}", square!(7));
    println!("{}", square!(1 + 2));

    println!("{}", sum!(1, 2, 3));
    println!("{}", sum!());
}
