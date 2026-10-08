# Zadanie 1: Wisielec

## Opis

Napisz grę w wisielca działającą w terminalu. Program losuje tajne słowo, a gracz zgaduje je litera po literze. Każda trafiona litera zostaje odkryta w słowie, a każda chybiona kosztuje jedno życie. Gracz wygrywa, gdy odkryje całe słowo, i przegrywa, gdy skończą mu się życia.

**Potrzebne rozdziały:** 1–5. Cele dodatkowe korzystają też z rozdziału 6.

## Wymagania

1. Gra to nowy projekt Cargo, utworzony poleceniem `cargo new`.
2. Tajne słowo jest losowane z listy słów wbudowanej w program. Użyj crate'a `rand`, tak jak w rozdziale 1.
3. Przed każdą próbą program pokazuje:
   - słowo z ukrytymi nieznanymi literami, na przykład `_ o r r o _`
   - liczbę pozostałych żyć
   - litery zgadnięte do tej pory
4. Gracz wpisuje z klawiatury jedną próbę na turę.
5. Program sprawdza próbę:
   - Puste wejście, więcej niż jeden znak albo znak, który nie jest literą, zostaje odrzucony z komunikatem i nie kosztuje życia.
   - Wielkie i małe litery liczą się jako ta sama litera.
   - Litera, która już była zgadywana, zostaje tak oznaczona i nie kosztuje życia.
6. Trafiona litera zostaje odkryta wszędzie, gdzie występuje w słowie. Chybiona litera kosztuje jedno życie. Gracz zaczyna z 6 życiami.
7. Gdy wszystkie litery są odkryte, program wypisuje komunikat o wygranej. Gdy skończą się życia, wypisuje komunikat o przegranej i pokazuje tajne słowo.

## Nowe w tym zadaniu: odczyt z klawiatury

Rozdziały nie omawiały odczytu danych od użytkownika. Oto wszystko, czego potrzebujesz:

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

- `read_line` czeka, aż gracz naciśnie Enter, a potem dopisuje linię do `input`.
- Linia zawiera na końcu Enter jako znak nowej linii. `trim()` go usuwa, razem ze spacjami wokół tekstu.
- Odczyt może się w rzadkich przypadkach nie udać, więc `read_line` zwraca `Result`. `expect` zatrzymuje wtedy program z komunikatem. `Result` omawiamy w rozdziale 6.
- Żeby odczytać kolejną próbę w następnej turze, zacznij od nowego, pustego `String`.

## Przykładowa rozgrywka

Tajne słowo to `borrow`. Linie z pojedynczą literą albo z `ab` to to, co wpisał gracz.

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

Twoje komunikaty nie muszą być identyczne, ale zachowanie programu tak.

## Cele dodatkowe

Gdy wymagania już działają, spróbuj kilku z tych rozszerzeń:

- Rysuj szubienicę w ASCII art, dodając jeden element za każde stracone życie. Gotowe rysunki są w sekcji bonusowej poniżej.
- Po rundzie zapytaj, czy grać dalej, i licz wygrane oraz przegrane.
- Pozwól zgadywać od razu całe słowo. Błędne słowo kosztuje 2 życia.
- Dodaj polskie słowa z literami takimi jak `ż`, `ó` i `ł` i upewnij się, że są poprawnie odkrywane.
- Dodaj poziomy trudności, na przykład łatwy, normalny i trudny, które zmieniają liczbę żyć albo długość słów.
- Napisz testy jednostkowe logiki gry: ukrywania słowa, obsługi trafionej, chybionej i powtórzonej próby (rozdział 6).
- Wczytuj listę słów z pliku tekstowego zamiast trzymać ją w programie.

## Bonus: szubienica w ASCII

Jeśli realizujesz cel z szubienicą, skopiuj te rysunki do swojego programu:

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

- `GALLOWS[0]` to pusta szubienica, a `GALLOWS[6]` to cała postać: jeden etap na każde stracone życie.
- Każdy rysunek zaczyna się od znaku nowej linii, więc po wypisaniu zaczyna się w osobnej linii.
- `r"…"` to **surowy napis** (raw string): ukośnik wsteczny jest w nim zwykłym znakiem, a potrzebują go ręce i nogi postaci.

## Wskazówki

- **Buduj grę małymi krokami** i uruchamiaj program po każdym z nich:
  1. Pokaż stałe słowo, całe ukryte.
  2. Obsłuż jedną próbę i pokaż słowo ponownie.
  3. Powtarzaj próby w pętli.
  4. Licz życia i zakończ grę wygraną albo przegraną.
  5. Losowanie słowa dodaj na końcu.
- **Trzymaj stan gry w jednym miejscu.** Struktura może przechowywać tajne słowo, zgadnięte litery i życia. Jej metody mogą odpowiadać na pytania w rodzaju „czy gra jest wygrana?” (rozdział 5).
- **Opisz wynik próby enumem,** na przykład trafiona, chybiona, już zgadywana albo niepoprawna. `match` na nim w głównej pętli decyduje, co wypisać (rozdziały 4 i 5).
- **Pracuj na znakach, nie na bajtach.** `chars()` przechodzi po literach napisu, a `Vec<char>` to prosty sposób na zapamiętanie zgadniętych liter (rozdział 5).
- **Ignorowanie wielkości liter jest łatwiejsze, gdy najpierw zamienisz wejście na małe litery,** zanim zaczniesz analizować jego znaki.
- **Losowanie słowa** działa jak rzut kostką z rozdziału 1, tylko z zakresem indeksów do twojej listy słów.
- **Oddziel logikę gry od odczytu i wypisywania.** Kod jest wtedy czytelniejszy i dużo łatwiejszy do testowania, jeśli zrealizujesz cel z testami.
- Co jakiś czas uruchamiaj `cargo clippy` i `cargo fmt`. Wyłapują błędy i utrzymują porządek w kodzie.
