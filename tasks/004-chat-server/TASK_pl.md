# Zadanie 4: Serwer czatu (wyzwanie)

## Opis

Napisz tekstowy czat dla kilku osób. Program **serwera** przyjmuje wiele połączeń jednocześnie. Każda osoba uruchamia mały program **klienta**, łączy się z serwerem i wybiera pseudonim. Wszystko, co wpisze, trafia do wszystkich pozostałych połączonych osób, a kilka poleceń pozwala wypisać użytkowników, zmienić pseudonim i wysyłać prywatne wiadomości.

To **zadanie z wyzwaniem**. Celowo wykracza poza kurs: używa sieci i wątków z biblioteki standardowej Rusta. Wszystko, czego potrzebujesz, przedstawia sekcja „Nowe w tym zadaniu”, a zewnętrzne crate'y nie są potrzebne. Jest dla ciebie, jeśli znasz już podstawy albo szybko przeszedłeś przez rozdziały.

**Potrzebne rozdziały:** 1–6 oraz sekcja „Nowe w tym zadaniu”.

## Wymagania

1. Serwer i klient to dwa programy w **jednym** projekcie Cargo: `src/bin/server.rs` i `src/bin/client.rs`.
2. Serwer nasłuchuje na `127.0.0.1:7878`. W swoim terminalu zapisuje, co się dzieje: nowe połączenia, wybrane pseudonimy, wyjścia użytkowników i błędy.
3. Wielu klientów może być połączonych jednocześnie. Wolny albo zepsuty klient nigdy nie blokuje pozostałych.
4. Po połączeniu użytkownik jest proszony o pseudonim. Pseudonim nie może być pusty, zawierać spacji ani być już zajęty. W przeciwnym razie serwer wyjaśnia dlaczego i pyta ponownie.
5. Linia wpisana przez użytkownika trafia do wszystkich **pozostałych** użytkowników jako `[nick] text`.
6. Wszyscy pozostali dowiadują się, gdy użytkownik dołącza (`* anna joined`) i wychodzi (`* anna left`). Klient, który rozłączy się bez `/quit`, na przykład przez zamknięcie okna, też liczy się jako wychodzący.
7. Linie zaczynające się od `/` to polecenia:
   - `/list` pokazuje pseudonimy wszystkich obecnych.
   - `/nick <new>` zmienia twój pseudonim, a pozostali dostają o tym informację.
   - `/msg <nick> <text>` wysyła prywatną wiadomość, którą widzi tylko ten użytkownik.
   - `/quit` rozłącza.
   - Nieznane polecenie, brakujący argument albo nieznany użytkownik dają komunikat o błędzie, wysłany tylko do użytkownika, który go wpisał.
8. Nic, co zrobi klient, nie może wysypać serwera: rozłączenie w połowie wiadomości, wysłanie tekstu, który nie jest poprawnym UTF-8, wysłanie bardzo długich linii. Błędy są obsługiwane przez `Result` i dotyczą tylko połączenia tego klienta.
9. **Klient** łączy się z serwerem i wysyła każdą linię wpisaną z klawiatury. Wiadomości od serwera są wypisywane **od razu po nadejściu**, nawet gdy użytkownik jest w trakcie pisania. Gdy serwer zamknie połączenie, klient informuje o tym i kończy działanie.

## Nowe w tym zadaniu

### Dwa programy w jednym projekcie

Każdy plik `.rs` w katalogu `src/bin/` staje się osobnym programem, nazwanym tak jak plik:

```text
chat/
├── Cargo.toml
└── src/
    └── bin/
        ├── server.rs
        └── client.rs
```

Uruchamiasz je poleceniami `cargo run --bin server` i `cargo run --bin client`, każde w osobnym terminalu. Kod potrzebny obu programom może trafić do `src/lib.rs`, ale w tym zadaniu w porządku jest trzymanie każdego programu w jego własnym pliku.

### Przyjmowanie połączenia

Serwer wiąże `TcpListener` z adresem i czeka na klientów. Ten przyjmuje jedno połączenie, wita je i odczytuje jedną linię:

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

- `accept()` czeka na jedno połączenie. `for stream in listener.incoming()` czeka na kolejne połączenia, bez końca.
- `TcpStream` to oba kierunki połączenia. `try_clone()` daje drugi uchwyt do tego samego połączenia, więc przez jeden możesz czytać, a przez drugi pisać.
- `writeln!` zapisuje linię do strumienia, tak jak `println!` wypisuje ją w terminalu (wymaga `use std::io::Write`).
- `BufReader::new(stream).lines()` czyta strumień linia po linii. `next()` czeka, aż nadejdzie cała linia, i zwraca `None`, gdy druga strona się rozłączyła.
- Prawie każda operacja sieciowa może się nie udać, więc wszystkie zwracają `Result`.

### Łączenie z serwerem

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

Uruchom najpierw poprzedni program, a potem ten w drugim terminalu.

### Wątki

`thread::spawn` uruchamia domknięcie (rozdział 5) równolegle z resztą programu:

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

- `move` przekazuje domknięciu **własność** zmiennych, których używa (rozdział 3). Jest wymagane: nowy wątek może działać dłużej niż funkcja, która go uruchomiła, więc nie może ich tylko pożyczyć.
- `join()` czeka, aż wątek się zakończy. Serwer zwykle pozwala wątkom klientów działać samodzielnie.
- `std::process::exit(0)` natychmiast kończy cały program, z dowolnego wątku.

### Współdzielenie danych między wątkami

Kilka wątków może współdzielić jedną wartość przez `Arc<Mutex<…>>`:

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

- `Arc` pozwala kilku wątkom **posiadać** tę samą wartość. `Arc::clone` tworzy kolejnego właściciela, a nie kopię wartości.
- `Mutex` pozwala w danej chwili **używać** wartości tylko jednemu wątkowi. `lock()` czeka na swoją kolej i zwraca strażnika (guard), który działa jak referencja modyfikowalna (`*value`).
- Blokada zostaje zwolniona, gdy strażnik zostanie usunięty (rozdział 3). Trzymaj go jak najkrócej.
- `lock()` zawodzi tylko wtedy, gdy inny wątek spanikował, trzymając blokadę. Dlatego komunikat w `expect` mówi właśnie o tym.

## Przykładowa sesja

Serwer i dwóch klientów, anna i ben, każde w osobnym terminalu. W terminalach klientów między liniami od serwera widać to, co wpisał użytkownik: pseudonim, tekst czatu i polecenia. Liczby po `127.0.0.1:` w dzienniku serwera są przy każdym uruchomieniu inne.

**Terminal anny:**

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

**Terminal bena** (jako pierwszy pseudonim wpisał pustą linię):

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

**Terminal serwera**, który na końcu został zatrzymany, więc klient anny zgłasza rozłączenie:

```text
listening on 127.0.0.1:7878
connection from 127.0.0.1:60845
127.0.0.1:60845 is anna
connection from 127.0.0.1:60846
127.0.0.1:60846 is ben
ben is now benny
benny left
```

Twoje komunikaty nie muszą być identyczne, ale zachowanie programu tak.

## Cele dodatkowe

Gdy wymagania już działają, spróbuj kilku z tych rozszerzeń:

- Napisz testy jednostkowe parsowania poleceń i sprawdzania pseudonimów (rozdział 6).
- Dodaj pokoje: `/join <room>` przenosi cię do pokoju, wiadomości docierają tylko do osób w tym samym pokoju, a `/rooms` wypisuje używane pokoje.
- Wysyłaj 10 ostatnich wiadomości użytkownikowi, który właśnie dołączył, żeby wiedział, o czym mowa.
- Dodaj godzinę do każdej wiadomości, na przykład `[12:03] [anna] hi`. Biblioteka standardowa poda bieżący czas, ale sformatowanie go jako godziny wymaga crate'a takiego jak `chrono`.
- Ogranicz wiadomości do, powiedzmy, 500 znaków i ogranicz, ile wiadomości użytkownik może wysłać na sekundę.
- Przyjmuj adres i port jako argumenty wiersza poleceń (`std::env::args`) zamiast stałego `127.0.0.1:7878`.
- Dodaj administratora: pierwszy użytkownik, który dołączy, może wyrzucać innych poleceniem `/kick <nick>`.
- Ostateczny boss: przepisz serwer z użyciem crate'a [`tokio`](https://tokio.rs), z zadaniami `async` zamiast wątków.

## Wskazówki

- **Buduj etapami** i testuj każdy etap, zanim przejdziesz dalej:
  1. Serwer echa, który odsyła każdą linię jednemu klientowi.
  2. Jeden wątek na klienta, żeby można było połączyć kilku klientów.
  3. Wspólna lista połączonych klientów.
  4. Wysyłanie każdej wiadomości do wszystkich pozostałych klientów.
  5. Pseudonimy, a potem polecenia.
  6. Program klienta. Dopóki go nie ma, pomogą dwa terminale z rozbudowanym fragmentem z sekcji „Łączenie z serwerem”.
- **Wspólny stan:** `HashMap<String, TcpStream>` z pseudonimu na uchwyt do zapisu tego klienta (z `try_clone()`), za `Arc<Mutex<…>>`. Każdy wątek klienta dostaje własny `Arc::clone`.
- **Trzymaj blokady krótko.** Zablokuj mapę, zrób szybką pracę i pozwól strażnikowi zniknąć. Nigdy nie czekaj na linię z sieci, trzymając blokadę: wszystkie inne wątki też by czekały.
- **Nieudany zapis oznacza, że klienta już nie ma.** Usuń go z mapy zamiast wywoływać `unwrap`.
- **Sprzątaj w jednym miejscu.** Cokolwiek kończy sesję klienta, `/quit`, rozłączenie czy błąd, klient musi zostać usunięty z mapy, a pozostali powiadomieni. Ułóż wątek klienta tak, żeby sprzątanie zawsze się wykonało, na przykład wywołując pętlę czatu jako funkcję zwracającą `Result` i sprzątając po niej, niezależnie od tego, co zwróciła.
- **Polecenia jako enum.** Zamieniaj każdą linię na enum `Command`, używając `split_once` albo dopasowania wycinków z zadania 2, a potem rób na nim `match` (rozdziały 4 i 5). `splitn(3, ' ')` dzieli `/msg anna see you later` na dokładnie 3 części.
- **Niepoprawne UTF-8** przychodzi z `lines()` jako `Err` rodzaju `std::io::ErrorKind::InvalidData`. Możesz na nie odpowiedzieć i czytać dalej, zamiast odłączać klienta.
- **Klient potrzebuje dwóch wątków:** główny czyta klawiaturę i wysyła każdą linię, a uruchomiony osobno czyta linie od serwera i je wypisuje. Gdy serwer się rozłączy, wątek czytający kończy program przez `std::process::exit`.
- **Testuj ręcznie** z serwerem i dwoma lub trzema klientami w osobnych terminalach. Spróbuj zamknąć terminal klienta w połowie rozmowy.
- Co jakiś czas uruchamiaj `cargo clippy` i `cargo fmt`.
