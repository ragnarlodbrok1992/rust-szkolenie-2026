# Zadanie 2: Kalkulator REPL

## Opis

Napisz interaktywny kalkulator działający w terminalu. REPL (read–eval–print loop, czyli pętla „wczytaj, oblicz, wypisz”) pokazuje znak zachęty, wczytuje to, co wpisał użytkownik, oblicza to, wypisuje wynik i zaczyna od nowa. Twój REPL oblicza wyrażenia matematyczne z typową kolejnością działań, nawiasami, wbudowanymi funkcjami takimi jak `sqrt` i `sin` oraz stałymi `pi` i `e`. Błędy we wpisanym tekście są zgłaszane czytelnymi komunikatami, a REPL działa dalej.

**Potrzebne rozdziały:** 1–6.

## Wymagania

1. Program pokazuje znak zachęty `> ` i czeka na dane w tej samej linii. Po każdej odpowiedzi pokazuje znak zachęty ponownie. `exit` albo `quit` kończy program, a puste linie są pomijane.
2. Liczby mogą być całkowite albo dziesiętne, na przykład `3` albo `2.5`. Wszystkie obliczenia używają `f64`.
3. Wyrażenia używają `+`, `-`, `*` i `/` z typową kolejnością działań: `*` i `/` przed `+` i `-`. Nawiasy zmieniają kolejność, a minus może stać przed liczbą albo nawiasem: `-3`, `-(2 + 1)`.
4. Spacje są opcjonalne: `2*(3+4)` i `2 * ( 3 + 4 )` dają ten sam wynik.
5. Wbudowane funkcje przyjmują jeden argument w nawiasach: co najmniej `sqrt`, `sin`, `cos`, `tan`, `abs`, `ln` i `exp`. Kąty są w radianach. Stałych `pi` i `e` można używać wszędzie tam, gdzie liczby.
6. Każdy z tych błędów wypisuje komunikat zaczynający się od `error:`, a REPL działa dalej:
   - znak, który nie należy do wyrażenia, np. `$`
   - niepoprawna liczba, np. `1.2.3`
   - brakujący albo nadmiarowy nawias
   - nieznana funkcja albo nazwa
   - niekompletne wyrażenie, np. `2 +`
   - dzielenie przez zero
   - wynik, który nie jest liczbą rzeczywistą, np. `sqrt(-1)` albo `ln(0)`
7. Program nigdy nie panikuje, cokolwiek wpisze użytkownik.
8. Błędy to twój własny `enum`, który implementuje `Display`, a funkcje, które mogą się nie udać, zwracają `Result` (rozdział 6).

## Nowe w tym zadaniu

### Odczyt linii

Użyj tego samego fragmentu z `read_line` co w zadaniu 1.

### Znak zachęty w tej samej linii

`print!` nie kończy linii, więc znak zachęty może zostać tam, gdzie użytkownik pisze. Terminal zwykle pokazuje tekst dopiero po zakończeniu linii, dlatego trzeba samemu **opróżnić bufor** (flush):

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

- `flush()` pochodzi z traitu `Write`, dlatego linia `use` udostępnia `Write` (traity są w rozdziale 6).

### Dopasowanie wycinków

`match` potrafi rozłożyć wycinek według jego długości i zawartości. Przydaje się to, gdy trzeba spojrzeć na kilka pierwszych elementów `Vec`, na przykład tokenów albo słów:

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

- `[]` pasuje do pustego wycinka, a `[single]` do dokładnie jednego elementu.
- `[first, rest @ ..]` pasuje do jednego elementu lub więcej. `..` oznacza dowolną liczbę elementów, a `rest @` nadaje im nazwę, jako wycinek.
- Żeby dopasować `Vec`, użyj `.as_slice()`, na przykład `match tokens.as_slice()`.

## Przykładowa sesja

Tekst po `> ` to to, co wpisał użytkownik.

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

Twoje komunikaty nie muszą być identyczne, ale zachowanie programu tak.

## Cele dodatkowe

Gdy wymagania już działają, spróbuj kilku z tych rozszerzeń:

- Napisz testy jednostkowe obu etapów: tego, co zwraca tokenizer, i tego, co zwraca obliczanie dla poprawnych wyrażeń i dla każdego rodzaju błędu (rozdział 6).
- Dodaj zmienne: `let x = 2 * pi` zapisuje wartość, a `x` można użyć w kolejnych wyrażeniach. Dodaj `ans`, które zawsze przechowuje ostatni wynik.
- Dodaj operator potęgowania `^`. Wiąże mocniej niż `*` i grupuje od prawej, więc `2^3^2` to `2^9` = `512`.
- Dodaj funkcje dwuargumentowe z argumentami oddzielonymi przecinkiem: `max(a, b)`, `min(a, b)`, `pow(a, b)`.
- Dodaj polecenie `help`, które wypisuje funkcje i stałe, oraz polecenie `history`, które pokazuje wpisane dotąd wyrażenia.
- Dodaj tryb stopni, przełączany poleceniami `deg` i `rad`, dla `sin`, `cos` i `tan`.
- Pokazuj najwyżej, powiedzmy, 10 miejsc po przecinku, żeby `0.1 + 0.2` wypisywało `0.3` zamiast `0.30000000000000004`.

## Wskazówki

- **Pracuj w dwóch etapach.** Najpierw **tokenizuj**: zamień tekst na `Vec` tokenów, takich jak liczby, nazwy, operatory i nawiasy. Potem **obliczaj** na tokenach. Każdy etap łatwiej napisać i przetestować osobno.
- **Zrób z tokenu enum** z wariantami takimi jak `Number(f64)`, `Name(String)`, `Plus` i `LeftParen` (rozdział 5).
- **Przechodź po wejściu jako `Vec<char>` z indeksem.** Możesz wtedy spojrzeć na bieżący znak i zdecydować, ile znaków należy do następnego tokenu. Dla liczby zbierz jej cyfry i kropki do `String` i wywołaj `parse::<f64>()`. Dla nazwy zbierz jej litery.
- **Kolejność działań** ma dwa znane rozwiązania. Wybierz jedno:
  - **Zstępowanie rekurencyjne** (recursive descent): napisz jedną funkcję na każdy poziom pierwszeństwa. Funkcja dla `+` i `-` wywołuje tę dla `*` i `/`, a ta wywołuje funkcję dla pojedynczej wartości: liczby, stałej, wywołania funkcji, wyrażenia w nawiasach albo minusa przed wartością. Ta ostatnia wywołuje znowu pierwszą funkcję dla wnętrza nawiasów.
  - **Algorytm stacji rozrządowej** (shunting-yard): przejdź raz po tokenach, trzymając dwa `Vec` jako stosy, jeden na liczby i jeden na operatory. Operator czeka na swoim stosie, aż następny operator będzie miał niższe pierwszeństwo.
- **`f64` zna już matematykę:** `.sqrt()`, `.sin()`, `.ln()` i reszta to metody `f64`, a `std::f64::consts::PI` to π.
- **Wyłapuj niepoprawne wyniki** za pomocą `is_nan()` i `is_infinite()`. Dzielenie przez zero sprawdź przed dzieleniem.
- **Niech `?` przenosi błędy w górę.** Gdy każdy etap zwraca `Result`, jedno `?` przekaże pierwszy błąd aż do pętli, która go wypisze.
- **Pętla niech będzie cienka:** wczytaj linię, wywołaj jedną funkcję, która ją oblicza, i wypisz wartość albo błąd.
- Co jakiś czas uruchamiaj `cargo clippy` i `cargo fmt`.
