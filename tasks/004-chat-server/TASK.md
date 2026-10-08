# Task 4: Chat server (challenge)

## Description

Write a text chat for several people. A **server** program accepts many connections at the same time. Each person starts a small **client** program, connects to the server, and picks a nickname. Everything they type is sent to everyone else who is connected, and a few commands let them list users, change their nickname, and send private messages.

This is a **challenge task**. It goes beyond the course on purpose: it uses networking and threads from Rust's standard library. The "New for this task" section introduces everything you need, and no external crates are required. It suits you if you already know the basics, or got through the chapters quickly.

**Chapters needed:** 1–6, plus the "New for this task" section.

## Requirements

1. The server and the client are two programs in **one** Cargo project: `src/bin/server.rs` and `src/bin/client.rs`.
2. The server listens on `127.0.0.1:7878`. In its own terminal, it logs what happens: new connections, chosen nicknames, users leaving, and errors.
3. Many clients can be connected at the same time. A slow or broken client never blocks the others.
4. After connecting, a user is asked for a nickname. A nickname can't be empty, contain spaces, or be in use already. Otherwise the server explains why and asks again.
5. A line typed by a user is sent to all **other** users as `[nick] text`.
6. Everyone else is told when a user joins (`* anna joined`) and leaves (`* anna left`). A client that disconnects without `/quit`, for example because its window was closed, counts as leaving too.
7. Lines starting with `/` are commands:
   - `/list` shows the nicknames of everyone online.
   - `/nick <new>` changes your nickname, and the others are told about it.
   - `/msg <nick> <text>` sends a private message that only that user sees.
   - `/quit` disconnects.
   - An unknown command, a missing argument, or an unknown user gets an error message, sent only to the user who typed it.
8. Nothing a client does can crash the server: disconnecting in the middle of a message, sending text that isn't valid UTF-8, sending very long lines. Errors are handled with `Result`, and only that client's connection is affected.
9. **The client** connects to the server and sends every line typed on the keyboard. Messages from the server are printed **as soon as they arrive**, even while the user is in the middle of typing. When the server closes the connection, the client says so and exits.

## New for this task

### Two programs in one project

Every `.rs` file in the `src/bin/` folder becomes a separate program, named after the file:

```text
chat/
├── Cargo.toml
└── src/
    └── bin/
        ├── server.rs
        └── client.rs
```

Start them with `cargo run --bin server` and `cargo run --bin client`, each in its own terminal. Code that both programs need can go into `src/lib.rs`, but for this task it's fine to keep each program in its own file.

### Accepting a connection

A server binds a `TcpListener` to an address and waits for clients. This one accepts a single connection, greets it, and reads one line:

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let (stream, address) = listener.accept()?;
    println!("connection from {address}");

    let mut writer = stream.try_clone()?;
    writeln!(writer, "Hello! Say something:")?;

    let mut lines = BufReader::new(stream).lines();
    if let Some(line) = lines.next() {
        println!("received: {}", line?);
    }
    Ok(())
}
```

- `accept()` waits for one connection. `for stream in listener.incoming()` waits for connections one after another, forever.
- A `TcpStream` is both directions of the connection. `try_clone()` gives you a second handle to the same connection, so you can read through one and write through the other.
- `writeln!` writes a line into the stream, like `println!` writes to the terminal (it needs `use std::io::Write`).
- `BufReader::new(stream).lines()` reads the stream line by line. `next()` waits until a whole line has arrived, and returns `None` when the other side has disconnected.
- Nearly every network operation can fail, so they all return `Result`.

### Connecting to a server

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut lines = BufReader::new(stream.try_clone()?).lines();

    if let Some(greeting) = lines.next() {
        println!("server says: {}", greeting?);
    }
    writeln!(stream, "hi there")?;
    Ok(())
}
```

Run the previous program first, then this one in a second terminal.

### Threads

`thread::spawn` runs a closure (chapter 5) at the same time as the rest of the program:

```rust
use std::thread;

fn main() {
    let name = String::from("worker");
    let handle = thread::spawn(move || {
        println!("hello from {name}");
    });
    handle.join().expect("the thread finished");
}
```

- `move` gives the closure **ownership** of the variables it uses (chapter 3). It's required: the new thread may run longer than the function that started it, so it can't just borrow them.
- `join()` waits until the thread finishes. A server usually lets its client threads run on their own instead.
- `std::process::exit(0)` ends the whole program immediately, from any thread.

### Sharing data between threads

Several threads can share one value through `Arc<Mutex<…>>`:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().expect("no thread panicked");
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("the thread finished");
    }
    println!("counter: {}", counter.lock().expect("no thread panicked"));
}
```

- `Arc` lets several threads **own** the same value. `Arc::clone` makes one more owner, not a copy of the value.
- `Mutex` lets only one thread at a time **use** the value. `lock()` waits for its turn and gives back a guard that works like a mutable reference (`*value`).
- The lock is released when the guard is dropped (chapter 3). Keep it alive for as short a time as possible.
- `lock()` fails only if another thread panicked while holding the lock. That's why the `expect` message says so.

## Example session

The server and two clients, anna and ben, each in their own terminal. In the client terminals, every line from the server comes with what the user typed in between: their nickname, chat text, and commands. The numbers after `127.0.0.1:` in the server log differ on every run.

**anna's terminal:**

```text
Welcome! Choose a nickname:
anna
Hello, anna! Commands: /list, /nick, /msg, /quit
* ben joined
hi ben!
[ben] hello anna
/msg ben psst, a secret
[anna -> ben] psst, a secret
* ben is now benny
/dance
error: unknown command /dance
/msg carl hi
error: nobody called carl is online
/nick
error: usage: /nick <new nickname>
* benny left
/list
Online: anna
Disconnected from the server.
```

**ben's terminal** (the first nickname he typed was an empty line):

```text
Welcome! Choose a nickname:

A nickname can't be empty or contain spaces. Try again:
anna
The nickname anna is taken. Try again:
ben
Hello, ben! Commands: /list, /nick, /msg, /quit
[anna] hi ben!
hello anna
/list
Online: anna, ben
[anna -> ben] psst, a secret
/nick benny
You are now benny
/quit
Disconnected from the server.
```

**The server's terminal**, which was stopped at the end, so anna's client reports the disconnect:

```text
listening on 127.0.0.1:7878
connection from 127.0.0.1:60845
127.0.0.1:60845 is anna
connection from 127.0.0.1:60846
127.0.0.1:60846 is ben
ben is now benny
benny left
```

Your messages don't have to match these word for word. The behavior does.

## Additional goals

When the requirements work, try some of these:

- Write unit tests for parsing commands and checking nicknames (chapter 6).
- Add rooms: `/join <room>` moves you to a room, messages only reach people in the same room, and `/rooms` lists the rooms in use.
- Send the last 10 messages to a user who has just joined, so they know what's going on.
- Add the time to every message, for example `[12:03] [anna] hi`. The standard library can give you the current time, but formatting it as a clock time takes a crate like `chrono`.
- Limit messages to, say, 500 characters, and limit how many messages a user can send per second.
- Take the address and port as command-line arguments (`std::env::args`) instead of a fixed `127.0.0.1:7878`.
- Add an admin: the first user to join can `/kick <nick>` others.
- The final boss: rewrite the server with the [`tokio`](https://tokio.rs) crate, using `async` tasks instead of threads.

## Hints

- **Build it in stages,** and test each one before moving on:
  1. An echo server that sends every line back to a single client.
  2. One thread per client, so several clients can be connected.
  3. A shared list of connected clients.
  4. Sending every message to all the other clients.
  5. Nicknames, then commands.
  6. The client program. Until it exists, two terminals with the `Connecting to a server` snippet, extended, can help.
- **Shared state:** a `HashMap<String, TcpStream>` from nickname to that client's write handle (from `try_clone()`), behind `Arc<Mutex<…>>`. Every client thread gets its own `Arc::clone`.
- **Keep locks short.** Lock the map, do the quick work, and let the guard drop. Never wait for a line from the network while holding the lock: every other thread would wait too.
- **A failed write means the client is gone.** Remove it from the map instead of calling `unwrap`.
- **Clean up in one place.** Whatever ends a client's session, whether `/quit`, a disconnect, or an error, the client must be removed from the map and the others told. Structure the client's thread so that the cleanup always runs, for example by calling the chat loop as a function that returns `Result`, and doing the cleanup after it, whatever it returned.
- **Commands as an enum.** Parse each line into a `Command` enum, using `split_once` or slice patterns from task 2, then `match` on it (chapters 4 and 5). `splitn(3, ' ')` splits `/msg anna see you later` into exactly 3 parts.
- **Invalid UTF-8** arrives from `lines()` as an `Err` with the kind `std::io::ErrorKind::InvalidData`. You can answer it and keep reading, instead of dropping the client.
- **The client needs two threads:** the main thread reads the keyboard and sends each line, and a spawned thread reads lines from the server and prints them. When the server disconnects, the reading thread ends the program with `std::process::exit`.
- **Test by hand** with the server and two or three clients in separate terminals. Try closing a client's terminal in the middle of a conversation.
- Run `cargo clippy` and `cargo fmt` from time to time.
