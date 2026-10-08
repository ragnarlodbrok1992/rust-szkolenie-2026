# Zadanie 3: Edytor pixel artu (wyzwanie)

## Opis

Napisz edytor pixel artu z własnym oknem. Małe płótno jest pokazane w powiększeniu, a rysujesz na nim myszą: odręcznie, prostymi liniami, prostokątami i wypełnianiem. Kolory wybiera się z koła barw, więc możliwy jest każdy kolor RGB. Gotowy obrazek zapisuje się jako plik PNG.

To **zadanie z wyzwaniem**. Celowo wykracza poza kurs: używa zewnętrznego crate'a [`macroquad`](https://docs.rs/macroquad), a niektóre rzeczy trzeba będzie znaleźć w jego dokumentacji. Jest dla ciebie, jeśli znasz już podstawy albo szybko przeszedłeś przez rozdziały.

**Potrzebne rozdziały:** 1–6 oraz dokumentacja `macroquad`.

## Wymagania

1. Program otwiera okno z płótnem 32 × 32. Każdy piksel płótna jest rysowany jako powiększony kwadrat, łatwy do trafienia myszą. Płótno jest na początku białe, a siatkę między pikselami można włączać i wyłączać.
2. **Ołówek:** dopóki trzymasz lewy przycisk myszy, piksel pod kursorem jest malowany bieżącym kolorem. Szybki ruch myszą nie może zostawiać przerw: połącz poprzednią i bieżącą pozycję linią.
3. **Gumka:** prawy przycisk myszy działa jak ołówek, ale maluje na biało.
4. **Linia:** naciśnij lewy przycisk, przeciągnij i puść, żeby narysować prostą linię w **dowolnym kierunku**, wyznaczoną **algorytmem Bresenhama**. Podczas przeciągania podgląd pokazuje, gdzie będzie linia.
5. **Prostokąt:** naciśnij, przeciągnij i puść, żeby narysować obrys prostokąta, z podglądem podczas przeciągania.
6. **Wypełnianie:** kliknięcie zmienia kolor spójnego obszaru pikseli tego samego koloru co kliknięty (flood fill). Piksele stykają się tylko bokami, nie rogami.
7. **Pipeta:** kliknięcie na płótnie ustawia kolor tego piksela jako bieżący.
8. **Koło barw:**
   - Koło, w którym **kąt** to odcień (czerwony, żółty, zielony i tak dalej dookoła koła), a **odległość od środka** to nasycenie, od bieli w środku do pełnego koloru na brzegu.
   - Pod nim **suwak jasności**, od czerni do pełnej jasności.
   - Rząd kilku gotowych próbek kolorów.
   - Kliknięcie albo przeciąganie po kole lub suwaku, albo kliknięcie próbki, zmienia bieżący kolor. Znacznik pokazuje wybrany punkt na kole.
   - Bieżący kolor jest pokazany jako próbka z wartością `#RRGGBB`.
9. **Pasek stanu:** pokazuje bieżące narzędzie, bieżący kolor i współrzędne piksela płótna pod kursorem.
10. **Cofanie:** `Ctrl+Z` cofa ostatnią zmianę i można go naciskać wielokrotnie. Całe pociągnięcie ołówkiem, od naciśnięcia przycisku do jego puszczenia, liczy się jako jedna zmiana.
11. **Zapis:** `S` zapisuje płótno jako `drawing.png`, jeden piksel obrazka na jeden piksel płótna.
12. Używanie myszy poza płótnem nigdy niczego nie maluje i nigdy nie wysypuje programu.

## Sterowanie

| Wejście | Działanie |
|---|---|
| Lewy przycisk myszy | Użycie bieżącego narzędzia albo wybór koloru na kole, suwaku lub próbkach |
| Prawy przycisk myszy | Gumka |
| `P` | Ołówek |
| `L` | Linia |
| `R` | Prostokąt |
| `F` | Wypełnianie |
| `I` | Pipeta |
| `G` | Pokaż lub ukryj siatkę |
| `C` | Wyczyść płótno (można cofnąć) |
| `S` | Zapisz jako `drawing.png` |
| `Ctrl+Z` | Cofnij |

Układ okna zależy od ciebie.

## Nowe w tym zadaniu

### Dodanie macroquad

W nowym projekcie Cargo uruchom:

```sh
cargo add macroquad
```

W chwili pisania dodaje to wersję 0.4. Na **Windowsie** i **macOS** nic więcej nie trzeba. Na **Linuksie** najpierw zainstaluj kilka bibliotek systemowych, na przykład na Ubuntu albo Debianie:

```sh
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

### Pierwsze okno

```rust
use macroquad::prelude::*;

#[macroquad::main("First window")]
async fn main() {
    loop {
        clear_background(WHITE);

        let (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            draw_rectangle(mouse_x - 10.0, mouse_y - 10.0, 20.0, 20.0, RED);
        }
        draw_text("Hold the left mouse button", 20.0, 40.0, 30.0, BLACK);

        next_frame().await;
    }
}
```

- `#[macroquad::main(...)]` otwiera okno i uruchamia twoje `main`. Traktuj `async` i `.await` jako część konfiguracji macroquad: każda klatka kończy się na `next_frame().await`.
- `loop` wykonuje się raz na klatkę, około 60 razy na sekundę. W każdej klatce odczytujesz wejście i rysujesz wszystko od nowa.
- Współrzędne to piksele typu `f32`. `(0.0, 0.0)` to lewy górny róg okna, a `y` rośnie w dół.
- To, co narysujesz później, pojawia się na wierzchu tego, co narysowane wcześniej.

### Gdzie szukać informacji

Wszystko jest w [dokumentacji macroquad](https://docs.rs/macroquad). Oto funkcje potrzebne w tym zadaniu:

| Obszar | Funkcje i typy |
|---|---|
| Mysz | `mouse_position`, `is_mouse_button_down`, `is_mouse_button_pressed`, `is_mouse_button_released` |
| Klawiatura | `is_key_pressed`, `is_key_down`, `KeyCode` |
| Rysowanie | `draw_rectangle`, `draw_rectangle_lines`, `draw_line`, `draw_circle_lines`, `draw_text`, `Color::from_rgba` |
| Obrazy | `Image::gen_image_color`, `Image::set_pixel`, `Image::export_png`, `Texture2D::from_image`, `draw_texture` |
| Rozmiar okna | `Conf`, używane jako `#[macroquad::main(window_conf)]` z funkcją, która je zwraca |

`is_mouse_button_down` jest prawdziwe w każdej klatce, dopóki przycisk jest wciśnięty. `is_mouse_button_pressed` i `is_mouse_button_released` są prawdziwe tylko w tej jednej klatce, w której to się dzieje.

## Cele dodatkowe

Gdy wymagania już działają, spróbuj kilku z tych rozszerzeń:

- Napisz testy jednostkowe logiki, która nie potrzebuje okna: linii Bresenhama, wypełniania, konwersji kolorów i cofania (rozdział 6).
- Dodaj ponawianie cofniętej zmiany, na przykład pod `Ctrl+Y`.
- Dodaj narzędzie do rysowania okręgów, korzystając z algorytmu punktu środkowego (midpoint circle), „rodzeństwa” algorytmu Bresenhama.
- Wczytuj plik PNG na płótno.
- Pozwól wybrać rozmiar płótna oraz przybliżać i oddalać widok.
- Dodaj tryb lustrzany, który rysuje wszystko podwójnie, symetrycznie względem środka płótna.
- Obsłuż przezroczystość: wartość alfa dla kolorów i szachownicę pod przezroczystymi pikselami.
- Zaznacz prostokątny obszar i przesuń go albo skopiuj.
- Zapisuj w większej skali, tak żeby każdy piksel płótna był w PNG, powiedzmy, blokiem 8 × 8.

## Wskazówki

- **Oddziel obrazek od okna.** Struktura `Canvas` z polami `width`, `height` i `Vec` kolorów nic nie wie o macroquad. Osobna funkcja rysuje ją w każdej klatce. Logikę płótna możesz wtedy testować bez otwierania okna.
- **Trzymaj piksele w jednym `Vec`.** Piksel `(x, y)` jest pod indeksem `y * width + x`. Daj `Canvas` metody do odczytu i ustawiania piksela, które po prostu ignorują pozycje poza płótnem.
- **Pozycja myszy na piksel płótna:** odejmij położenie płótna w oknie, podziel przez rozmiar jednego powiększonego piksela i zaokrąglij w dół. Zwracaj `Option`, z `None`, gdy mysz nie jest nad płótnem.
- **Zrób z narzędzia enum** i zdecyduj przez `match`, co robi kliknięcie (rozdziały 4 i 5).
- **Linia i prostokąt to małe automaty stanów.** Zapamiętaj piksel początkowy, gdy przycisk zostanie naciśnięty. Dopóki jest trzymany, rysuj podgląd na wierzchu płótna, nie zmieniając płótna. Zatwierdź kształt, gdy przycisk zostanie puszczony.
- **Algorytm Bresenhama** idzie od jednego końca linii do drugiego, piksel po pikselu. Zawsze robi krok wzdłuż dłuższej osi i pamięta całkowity „błąd”, który mówi, kiedy zrobić krok także wzdłuż krótszej. Nie używa liczb zmiennoprzecinkowych. Ogólna wersja obsługuje wszystkie kierunki, także linie biegnące w górę albo w lewo. Dobry opis jest [na Wikipedii](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm). Przeczytaj fragment o obsłudze wszystkich przypadków.
- **Testuj algorytm Bresenhama przez właściwości,** a nie dokładne listy pikseli. Poprawna linia zawiera oba końce, nie ma przerw (każdy krok prowadzi do sąsiedniego piksela) i ma dokładnie tyle pikseli, ile wynosi dłuższy z jej wymiarów, plus jeden. Różne poprawne implementacje mogą inaczej wybierać między dwoma równie bliskimi pikselami.
- **Wypełnianie rób z `Vec` jako listą zadań** pozycji do sprawdzenia, a nie rekurencją. Funkcja wywołująca samą siebie dla każdego piksela może przepełnić stos na dużym obszarze.
- **Koło barw używa modelu kolorów HSV:** odcień to kąt od 0 do 360 stopni, a nasycenie i wartość (jasność) mają zakres od 0 do 1.
  - Kąt między środkiem koła a myszą daje `f32::atan2`.
  - Odległość daje twierdzenie Pitagorasa.
  - Zamiana HSV na RGB to znany wzór, który dzieli koło odcieni na sześć sektorów po 60 stopni. Wyszukaj „HSV to RGB”.
- **Narysuj koło raz, a nie w każdej klatce.** Liczenie tysięcy pikseli 60 razy na sekundę jest wolne. Wypełnij kołem `Image` raz na początku, zamień je na `Texture2D` i w każdej klatce rysuj teksturę.
- **Utrzymuj koło w zgodzie z kolorem.** Gdy pipeta albo próbka ustawia kolor, znacznik na kole powinien przesunąć się w odpowiednie miejsce. Do tego potrzebna jest konwersja w drugą stronę, z RGB na HSV.
- **Cofanie przez zrzuty stanu.** Zanim zacznie się zmiana, odłóż kopię płótna do `Vec`, a przy cofaniu zdejmij ją ze stosu. Kopię daje `#[derive(Clone)]` (rozdział 6). Rób zrzut, gdy przycisk myszy zostanie *naciśnięty*, a nie w każdej klatce, gdy jest trzymany.
- Co jakiś czas uruchamiaj `cargo clippy` i `cargo fmt`.
