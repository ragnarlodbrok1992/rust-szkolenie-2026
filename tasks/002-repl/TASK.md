# Task 2: Calculator REPL

## Description

Write an interactive calculator that runs in the terminal. A REPL (read–eval–print loop) shows a prompt, reads what the user typed, evaluates it, prints the result, and starts again. Your REPL evaluates mathematical expressions with the usual operator precedence, parentheses, built-in functions like `sqrt` and `sin`, and the constants `pi` and `e`. Mistakes in the input are reported as clear error messages, and the REPL keeps running.

**Chapters needed:** 1–6.

## Requirements

1. The program shows a `> ` prompt and waits for input on the same line. After each answer it shows the prompt again. `exit` or `quit` ends the program, and empty lines are ignored.
2. Numbers can be whole or decimal, for example `3` or `2.5`. All calculations use `f64`.
3. Expressions use `+`, `-`, `*` and `/` with the usual precedence: `*` and `/` before `+` and `-`. Parentheses change the order, and a minus sign can stand in front of a number or a parenthesis: `-3`, `-(2 + 1)`.
4. Spaces are optional: `2*(3+4)` and `2 * ( 3 + 4 )` give the same result.
5. The built-in functions take one argument in parentheses: at least `sqrt`, `sin`, `cos`, `tan`, `abs`, `ln`, and `exp`. Angles are in radians. The constants `pi` and `e` can be used anywhere a number can.
6. Each of these mistakes prints a message starting with `error:`, and the REPL keeps running:
   - a character that isn't part of an expression, like `$`
   - a malformed number, like `1.2.3`
   - a missing or an extra parenthesis
   - an unknown function or name
   - an incomplete expression, like `2 +`
   - division by zero
   - a result that isn't a real number, like `sqrt(-1)` or `ln(0)`
7. The program never panics, whatever the user types.
8. The errors are an `enum` of your own that implements `Display`, and the functions that can fail return `Result` (chapter 6).

## New for this task

### Reading a line

Use the same `read_line` snippet as in task 1.

### A prompt on the same line

`print!` doesn't end the line, so the prompt can stay where the user types. Terminals usually show text only once a line is complete, so you need to **flush** the output yourself:

```rust
use std::io::{self, Write};

fn main() {
    print!("> ");
    io::stdout().flush().expect("failed to show the prompt");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read a line");
    println!("You typed: {}", input.trim());
}
```

- `flush()` comes from the `Write` trait, which is why the `use` line brings in `Write` (traits are in chapter 6).

### Matching on slices

`match` can take a slice apart by its length and contents. That's handy when you need to look at the first few items of a `Vec`, for example tokens or words:

```rust
fn describe(words: &[&str]) -> String {
    match words {
        [] => String::from("nothing"),
        [single] => format!("one word: {single}"),
        [first, rest @ ..] => format!("{first} and {} more", rest.len()),
    }
}

fn main() {
    let words: Vec<&str> = "sqrt of sixteen".split_whitespace().collect();
    println!("{}", describe(&words));
    println!("{}", describe(&["pi"]));
    println!("{}", describe(&[]));
}
```

- `[]` matches an empty slice, and `[single]` matches exactly one item.
- `[first, rest @ ..]` matches one or more items. `..` stands for any number of items, and `rest @` gives them a name, as a slice.
- To match on a `Vec`, use `.as_slice()`, for example `match tokens.as_slice()`.

## Example session

The text after `> ` is what the user typed.

```text
> 2 * (3 + 4)
14
> 10 / 4
2.5
> 2+3*4
14
> -(2 + 1) * 2
-6
> sqrt(16) + abs(-2)
6
> sin(pi / 2)
1
> e
2.718281828459045
>
> 2 +
error: incomplete expression
> 1 / 0
error: division by zero
> sqrt(-1)
error: the result is not a real number
> (1 + 2
error: missing `)`
> 1 + 2)
error: unexpected `)`
> foo(2)
error: unknown function `foo`
> x + 1
error: unknown name `x`
> 3 $ 4
error: unexpected character `$`
> 1.2.3
error: invalid number `1.2.3`
> exit
```

Your messages don't have to match these word for word. The behavior does.

## Additional goals

When the requirements work, try some of these:

- Write unit tests for both stages: what the tokenizer produces, and what the evaluator returns for correct expressions and for each kind of error (chapter 6).
- Add variables: `let x = 2 * pi` stores a value, and `x` can be used in later expressions. Add `ans`, which always holds the last result.
- Add the power operator `^`. It binds tighter than `*` and groups from the right, so `2^3^2` is `2^9` = `512`.
- Add functions with two arguments, separated by a comma: `max(a, b)`, `min(a, b)`, `pow(a, b)`.
- Add a `help` command that lists the functions and constants, and a `history` command that shows the expressions typed so far.
- Add a degrees mode, switched with `deg` and `rad`, for `sin`, `cos`, and `tan`.
- Show at most, say, 10 decimal places, so that `0.1 + 0.2` prints `0.3` instead of `0.30000000000000004`.

## Hints

- **Work in two stages.** First **tokenize**: turn the text into a `Vec` of tokens, such as numbers, names, operators, and parentheses. Then **evaluate** the tokens. Each stage is easier to write and to test on its own.
- **Make the token an enum,** with variants like `Number(f64)`, `Name(String)`, `Plus`, and `LeftParen` (chapter 5).
- **Walk the input as a `Vec<char>` with an index.** You can then look at the current character and decide how many characters belong to the next token. For a number, collect its digits and dots into a `String` and call `parse::<f64>()`. For a name, collect its letters.
- **Precedence** has two well-known solutions. Pick one:
  - **Recursive descent:** write one function for each precedence level. The function for `+` and `-` calls the one for `*` and `/`, which calls the one for single values: a number, a constant, a function call, an expression in parentheses, or a minus sign in front of a value. That last function calls the first one again for the inside of parentheses.
  - **Shunting-yard:** go through the tokens once, keeping two `Vec`s as stacks, one for numbers and one for operators. An operator waits on its stack until the next operator has lower precedence.
- **`f64` already knows the math:** `.sqrt()`, `.sin()`, `.ln()`, and the rest are methods on `f64`, and `std::f64::consts::PI` is π.
- **Catch invalid results** with `is_nan()` and `is_infinite()`. Check for division by zero before you divide.
- **Let `?` carry errors upward.** When every step returns `Result`, one `?` passes the first error all the way up to the loop, which prints it.
- **Keep the loop thin:** read a line, call one function that evaluates it, and print either the value or the error.
- Run `cargo clippy` and `cargo fmt` from time to time.
