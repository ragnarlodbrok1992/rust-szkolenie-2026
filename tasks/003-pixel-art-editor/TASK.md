# Task 3: Pixel-art editor (challenge)

## Description

Write a pixel-art editor with its own window. A small canvas is shown magnified, and you draw on it with the mouse: freehand, straight lines, rectangles, and flood fill. Colors come from a color wheel, so any RGB color is possible. The finished picture is saved as a PNG file.

This is a **challenge task**. It goes beyond the course on purpose: it uses an external crate, [`macroquad`](https://docs.rs/macroquad), and you'll need to look things up in its documentation. It suits you if you already know the basics, or got through the chapters quickly.

**Chapters needed:** 1–6, plus the `macroquad` documentation.

## Requirements

1. The program opens a window with a 32 × 32 canvas. Every canvas pixel is drawn as an enlarged square, so it's easy to hit with the mouse. The canvas starts white, and a grid between the pixels can be switched on and off.
2. **Pencil:** while the left mouse button is held, the pixel under the cursor is painted in the current color. Moving the mouse quickly must not leave gaps: connect the previous and the current position with a line.
3. **Eraser:** the right mouse button works like the pencil, but paints white.
4. **Line tool:** press the left button, drag, and release to draw a straight line in **any direction**, computed with **Bresenham's line algorithm**. While you drag, a preview shows where the line will be.
5. **Rectangle tool:** press, drag, and release to draw the outline of a rectangle, with a preview while dragging.
6. **Fill tool:** a click recolors the connected area of pixels that have the same color as the clicked one (flood fill). Pixels touch only through their sides, not their corners.
7. **Eyedropper:** a click on the canvas makes that pixel's color the current color.
8. **Color wheel:**
   - A circle where the **angle** is the hue (red, yellow, green, and so on around the circle) and the **distance from the center** is the saturation, from white in the middle to full color at the edge.
   - Below it, a **brightness slider** from black to full brightness.
   - A row of a few ready-made color swatches.
   - Clicking or dragging on the wheel or the slider, or clicking a swatch, changes the current color. A marker shows the chosen point on the wheel.
   - The current color is shown as a swatch with its `#RRGGBB` value.
9. **Status bar:** shows the current tool, the current color, and the canvas coordinates of the pixel under the cursor.
10. **Undo:** `Ctrl+Z` undoes the last change, and can be pressed repeatedly. A whole pencil stroke, from pressing the button to releasing it, counts as one change.
11. **Saving:** `S` saves the canvas as `drawing.png`, one image pixel per canvas pixel.
12. Using the mouse outside the canvas never paints anything and never crashes the program.

## Controls

| Input | Action |
|---|---|
| Left mouse button | Use the current tool, or pick a color on the wheel, slider, or swatches |
| Right mouse button | Erase |
| `P` | Pencil |
| `L` | Line tool |
| `R` | Rectangle tool |
| `F` | Fill tool |
| `I` | Eyedropper |
| `G` | Show or hide the grid |
| `C` | Clear the canvas (can be undone) |
| `S` | Save as `drawing.png` |
| `Ctrl+Z` | Undo |

The layout of the window is up to you.

## New for this task

### Adding macroquad

In a new Cargo project, run:

```sh
cargo add macroquad
```

At the time of writing this adds version 0.4. On **Windows** and **macOS** nothing else is needed. On **Linux**, install a few system libraries first, for example on Ubuntu or Debian:

```sh
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

### A first window

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

- `#[macroquad::main(...)]` opens the window and starts your `main`. Treat `async` and `.await` as part of macroquad's setup: every frame ends with `next_frame().await`.
- The `loop` runs once per frame, about 60 times a second. Each frame you read the input and draw everything again, from scratch.
- Coordinates are `f32` pixels. `(0.0, 0.0)` is the top-left corner of the window, and `y` grows downward.
- Things drawn later appear on top of things drawn earlier.

### Where to look things up

Everything is in the [macroquad documentation](https://docs.rs/macroquad). These are the functions this task needs:

| Area | Functions and types |
|---|---|
| Mouse | `mouse_position`, `is_mouse_button_down`, `is_mouse_button_pressed`, `is_mouse_button_released` |
| Keyboard | `is_key_pressed`, `is_key_down`, `KeyCode` |
| Drawing | `draw_rectangle`, `draw_rectangle_lines`, `draw_line`, `draw_circle_lines`, `draw_text`, `Color::from_rgba` |
| Images | `Image::gen_image_color`, `Image::set_pixel`, `Image::export_png`, `Texture2D::from_image`, `draw_texture` |
| Window size | `Conf`, used as `#[macroquad::main(window_conf)]` with a function that returns it |

`is_mouse_button_down` is true for every frame while the button is held. `is_mouse_button_pressed` and `is_mouse_button_released` are true only in the single frame where it happens.

## Additional goals

When the requirements work, try some of these:

- Write unit tests for the logic that doesn't need a window: Bresenham lines, flood fill, color conversions, and undo (chapter 6).
- Add redo, for example with `Ctrl+Y`.
- Add a circle tool, using the midpoint circle algorithm, Bresenham's sibling.
- Load a PNG file into the canvas.
- Let the user pick the canvas size, and zoom in and out.
- Add a mirror mode that draws everything twice, symmetrically around the middle of the canvas.
- Support transparency: an alpha value for colors, and a checkerboard behind transparent pixels.
- Select a rectangular area, and move or copy it.
- Save at a larger scale, so that every canvas pixel becomes, say, an 8 × 8 block in the PNG.

## Hints

- **Keep the picture apart from the window.** A `Canvas` struct with a `width`, a `height`, and a `Vec` of colors knows nothing about macroquad. A separate function draws it every frame. You can then test the canvas logic without opening a window.
- **Store the pixels in one `Vec`.** The pixel at `(x, y)` is at index `y * width + x`. Give `Canvas` methods to read and set a pixel that simply ignore positions outside the canvas.
- **Mouse position to canvas pixel:** subtract the canvas's position in the window, divide by the size of one enlarged pixel, and round down. Return an `Option`, with `None` when the mouse isn't over the canvas.
- **Make the tool an enum,** and `match` on it to decide what a click does (chapters 4 and 5).
- **Line and rectangle tools are small state machines.** Remember the starting pixel when the button is pressed. While it's held, draw the preview on top of the canvas without changing the canvas. Commit the shape when the button is released.
- **Bresenham's line algorithm** walks from one end of the line to the other, one pixel at a time. It always steps along the longer axis, and keeps an integer "error" that says when to step along the shorter axis too. No floating-point numbers are involved. The general version handles all directions, including lines going up or to the left. A good description is [on Wikipedia](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm). Read the section about handling all cases.
- **Test Bresenham with properties,** not exact pixel lists. A correct line includes both endpoints, has no gaps (each step moves to a neighboring pixel), and has exactly as many pixels as the longer of its width and height, plus one. Different correct implementations may choose differently between two equally close pixels.
- **Flood fill with a `Vec` as a to-do list** of positions to check, not with recursion. A function that calls itself for every pixel can run out of stack on a big area.
- **The color wheel uses the HSV color model:** hue is an angle from 0 to 360 degrees, and saturation and value (brightness) go from 0 to 1.
  - The angle between the wheel's center and the mouse comes from `f32::atan2`.
  - The distance comes from Pythagoras.
  - Converting HSV to RGB is a well-known formula that splits the hue circle into six 60-degree sectors. Look up "HSV to RGB".
- **Draw the wheel once, not every frame.** Computing thousands of pixels 60 times a second is slow. Fill an `Image` with the wheel once at the start, turn it into a `Texture2D`, and draw the texture every frame.
- **Keep the wheel in sync.** When the eyedropper or a swatch sets the color, the marker on the wheel should move to match. That needs the conversion the other way, RGB to HSV.
- **Undo with snapshots.** Before a change starts, push a copy of the canvas onto a `Vec`, and pop it to undo. `#[derive(Clone)]` gives you the copy (chapter 6). Take the snapshot when the mouse button is *pressed*, not in every frame while it's held.
- Run `cargo clippy` and `cargo fmt` from time to time.
