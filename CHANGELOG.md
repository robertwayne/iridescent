# Change Log

## 2022.10.19 - v0.2.1

- Removed some unnecessary clones.

## 2022.10.19 - v0.2

- **Breaking** Consolidated the `rgb_foreground` and `rgb_background` methods
  into `foreground` and `background` respectively. This allows for a much
  simpler user-facing API.
- The new `foreground` and `background` methods now accept any type that
  can be converted into a `ForegroundColor` or `BackgroundColor`.
- Added `From<u8>` impl for both `ForegroundColor` and `BackgroundColor`.
- Added `From<Rgb>` impl for both `ForegroundColor` and `BackgroundColor`.
- Added `From<[u8; 3]>` impl for the `Rgb` type.
- Added `From<[u8; 3]>` impl for both `ForegroundColor` and `BackgroundColor`.
- Added support for hex string literals (e.g. `"#FF0000"`):
  - Added `From<&str>` impl for the `Rgb` type.
  - Added `From<&str>` impl for both `ForegroundColor` and `BackgroundColor`.
- **Breaking** `Color` has been split into `ForegroundColor` and
  `BackgroundColor`.
- **Breaking** The random methods are now a part of the `Rgb` type and a new-type wrapper
  over u8, `Simple`. You can call them like so: `Rgb::random()` and
    `Simple::random()`.

Examples of the new API:

```rust
use iridescent::{Styled, constants::{RED, WHITE}};

fn main() {
    let hello = "Hello".foreground("#ff00ff").bold();
    let world = "world".foreground(&[0, 255, 0]);
    println!("{hello}, {world}!");

    let nice = "Nice".foreground(RED).background(WHITE).bold();
    println!("{nice}");
}
```

## 2022.02.06 - v0.1

Initial release.
