# Task 1: Hangman

## Description

Write a hangman game that runs in the terminal. The program picks a secret word, and the player guesses it one letter at a time. Every correct letter is revealed in the word, and every wrong one costs a life. The player wins by uncovering the whole word and loses when the lives run out.

**Chapters needed:** 1–5. The additional goals also use chapter 6.

## Requirements

1. The game is a new Cargo project, created with `cargo new`.
2. The secret word is picked at random from a list of words built into the program. Use the `rand` crate, as in chapter 1.
3. Before every guess, the program shows:
   - the word with unknown letters hidden, for example `_ o r r o _`
   - the number of lives left
   - the letters guessed so far
4. The player types one guess per turn on the keyboard.
5. The program checks the guess:
   - Empty input, more than one character, or a character that isn't a letter is rejected with a message, and it doesn't cost a life.
   - Uppercase and lowercase letters count as the same letter.
   - A letter that was already guessed is reported as such, and it doesn't cost a life.
6. A correct letter is revealed everywhere it appears in the word. A wrong letter costs one life. The player starts with 6 lives.
7. When every letter is revealed, the program prints a winning message. When no lives are left, it prints a losing message that shows the secret word.

## New for this task: reading keyboard input

The chapters didn't cover reading input. Here's all you need:

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read a line");
    let guess = input.trim();
    println!("You typed: {guess}");
}
```

- `read_line` waits for the player to press Enter, then appends the line to `input`.
- The line includes the Enter key as a newline character at the end. `trim()` removes it, along with any spaces around the text.
- Reading can fail in rare cases, so `read_line` returns a `Result`. `expect` stops the program with a message if that happens. `Result` is covered in chapter 6.
- To read again on the next turn, start from a new, empty `String`.

## Example session

The secret word here is `borrow`. The lines with a single letter, or `ab`, are what the player typed.

```text
Word: _ _ _ _ _ _
Lives: 6   Guessed:
Your guess:
e
No, it isn't there.

Word: _ _ _ _ _ _
Lives: 5   Guessed: e
Your guess:
o
Yes, it's there!

Word: _ o _ _ o _
Lives: 5   Guessed: e o
Your guess:
O
You already guessed that letter.

Word: _ o _ _ o _
Lives: 5   Guessed: e o
Your guess:
ab
Please type a single letter.

Word: _ o _ _ o _
Lives: 5   Guessed: e o
Your guess:
r
Yes, it's there!

Word: _ o r r o _
Lives: 5   Guessed: e o r
Your guess:
b
Yes, it's there!

Word: b o r r o _
Lives: 5   Guessed: e o r b
Your guess:
x
No, it isn't there.

Word: b o r r o _
Lives: 4   Guessed: e o r b x
Your guess:
w
Yes, it's there!

You won! The word was: borrow
```

Your messages don't have to match these word for word. The behavior does.

## Additional goals

When the requirements work, try some of these:

- Draw the gallows in ASCII art, adding one part for every life lost. Ready-made drawings are in the bonus section below.
- After a round, ask whether to play again, and keep a score of wins and losses.
- Let the player guess the whole word at once. A wrong word costs 2 lives.
- Add Polish words with letters like `ż`, `ó` and `ł`, and make sure they're revealed correctly.
- Add difficulty levels, for example easy, normal, and hard, which change the number of lives or the length of the words.
- Write unit tests for the game logic: hiding the word, handling a correct guess, a wrong guess, and a repeated guess (chapter 6).
- Read the word list from a text file instead of keeping it in the program.

## Bonus: ASCII gallows

If you do the gallows goal, copy these drawings into your program:

```rust
const GALLOWS: [&str; 7] = [
    r"
  +---+
  |   |
      |
      |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
      |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========",
    r"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========",
];
```

- `GALLOWS[0]` is the empty gallows and `GALLOWS[6]` is the whole figure: one stage for each life lost.
- Each drawing starts with a newline, so it begins on a line of its own when printed.
- `r"…"` is a **raw string**: a backslash in it is an ordinary character, which the figure's arms and legs need.

## Hints

- **Build it in small steps,** and run the program after each one:
  1. Show a fixed word, fully hidden.
  2. Handle a single guess and show the word again.
  3. Repeat guesses in a loop.
  4. Count lives, and end the game on a win or a loss.
  5. Pick the word at random last.
- **Keep the game state together.** A struct can hold the secret word, the guessed letters, and the lives. Methods on it can answer questions like "is the game won?" (chapter 5).
- **Describe what a guess did with an enum,** for example correct, wrong, already guessed, or invalid. A `match` on it in the main loop decides what to print (chapters 4 and 5).
- **Work with characters, not bytes.** `chars()` goes over the letters of a string, and a `Vec<char>` is a simple way to remember the guessed letters (chapter 5).
- **Ignoring case is easier if you lowercase the input first,** before you look at its characters.
- **Picking a random word** works like the dice roll in chapter 1, but with a range of indexes into your list of words.
- **Keep the game logic apart from reading and printing.** It makes the code easier to follow, and much easier to test if you do the testing goal.
- Run `cargo clippy` and `cargo fmt` from time to time. They catch mistakes and keep the code tidy.
