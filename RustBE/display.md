# What is `Display` in Rust?

## The basic idea

`Debug` (from before) gives you printing for free, but it's ugly and mechanical — it just dumps the struct name and its raw contents. `Display` is the *opposite*: it's beautiful and readable, but **you must write it yourself, by hand, every single time**. There's no shortcut, no `#[derive(Display)]`.

Think of it like this:
- `Debug` = an auto-generated receipt printout — functional, not pretty.
- `Display` = a custom greeting card — you design exactly how it looks.

## Why can't `Display` just be automatic too?

Here's the key insight from the notes: **automatic formatting only makes sense when there's one "obviously correct" way to show something.** For `Debug`, "just show me the raw fields" is always a reasonable default. But for `Display`, there often *isn't* one obvious "correct" style.

Take `Vec<T>` (a growable list) as the example:
- A `Vec` of file paths might make sense joined by `:` → `/:/etc:/home/username:/bin`
- A `Vec` of numbers might make sense joined by `,` → `1,2,3`

Since Rust's standard library can't know *your* intended meaning for a generic list, it refuses to guess. So `Display` is **not implemented for `Vec<T>`** or other generic containers — you're stuck using `Debug` (`{:?}`) for those. But for your *own* custom, non-generic structs, you're free (and expected) to implement `Display` however makes sense for your type.

## How to implement `Display`

```rust
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

Breaking this down line by line:

- `use std::fmt;` — imports the formatting module so we can reference `fmt::Display`, `fmt::Formatter`, etc.
- `impl fmt::Display for Structure { ... }` — this says: *"I am now providing the Display behavior for the type `Structure`."* This is called implementing a trait.
- `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result` — this exact function signature is **required** by the `Display` trait; you can't change it. Think of it as filling in a form with a fixed shape:
  - `&self` — a reference to the value being printed (here, our `Structure`).
  - `f: &mut fmt::Formatter` — a "writer" you send your text output to. It's mutable because writing to it changes it.
  - `-> fmt::Result` — tells Rust whether the write succeeded or failed.
- `write!(f, "{}", self.0)` — this is almost identical to `println!`, except instead of printing straight to the terminal, it writes into `f` (the formatter), which Rust then uses to build the final output. `self.0` refers to the first (and only) value stored in the tuple struct `Structure(i32)`.

So essentially: **you're manually telling Rust "when someone prints me with `{}`, here's exactly the text to produce."**

## Full example, explained

```rust
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
```
- `MinMax` holds two numbers (`self.0` and `self.1`, since it's a tuple struct).
- It derives `Debug` (the free, automatic option), *and* manually implements `Display` (the custom option) — a type can have both at once, they don't conflict.
- The `Display` version formats it as `(0, 14)` — clean and specific to what `MinMax` represents.

```rust
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
```
- Same idea, but this struct uses **named fields** (`x` and `y`) instead of tuple positions, so we refer to them as `self.x` and `self.y`.
- The custom `Display` output becomes `x: 3.3, y: 7.2` — again, a style *we* chose, tailored to what a 2D point means.

```rust
fn main() {
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);   // (0, 14)
    println!("Debug: {:?}", minmax);   // MinMax(0, 14)
```
This is the whole point of the lesson: **the same value can be printed two different ways**, depending on which marker you use:
- `{}` → uses your custom `Display` implementation → `(0, 14)`
- `{:?}` → uses the auto-generated `Debug` implementation → `MinMax(0, 14)`

```rust
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);   // x: 3.3, y: 7.2
    println!("Debug: {:?}", point);   // Point2D { x: 3.3, y: 7.2 }
```
Same pattern — Display gives the clean custom version, Debug gives the raw mechanical version.

```rust
    // println!("What does Point2D look like in binary: {:b}?", point);
```
This line is commented out because it **would not compile**. `{:b}` requires yet another trait — `fmt::Binary` — which we never implemented for `Point2D`. This proves the broader point: `std::fmt` has *many* separate traits (`Display`, `Debug`, `Binary`, `Octal`, `Hex`, etc.), and each one only works if you (or `derive`) specifically implemented it for that type. Having `Display` doesn't give you `Binary` for free — they're completely separate contracts.

## Summary: Debug vs Display

| | `Debug` (`{:?}`) | `Display` (`{}`) |
|---|---|---|
| Purpose | Developer debugging | Clean, user-facing output |
| Can auto-generate? | Yes, `#[derive(Debug)]` | No, must write manually |
| Works on generic types like `Vec<T>`? | Yes | No — no single "correct" style exists |
| Control over format | None (whatever `derive` produces) | Full — you decide exactly |

---

## Solving the Activity: adding a `Complex` struct

The goal: create a `Complex` struct with a `real` and `imag` part, where:
- `Display` shows it like `3.3 +7.2i` (with a space after the sign, as the bonus asks)
- `Debug` shows it like `Complex { real: 3.3, imag: -2.3 }`

```rust
use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Choose "+" or "-" depending on whether `imag` is negative,
        // and print the absolute value so the sign isn't doubled up.
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, self.imag.abs())
        }
    }
}

fn main() {
    let a = Complex { real: 3.3, imag: 7.2 };
    let b = Complex { real: 4.7, imag: -2.3 };

    println!("Display: {}", a);
    println!("Debug: {:?}", a);
    println!();
    println!("Display: {}", b);
    println!("Debug: {:?}", b);
}
```

**Why this works:**
- `#[derive(Debug)]` gives us the free `Complex { real: 3.3, imag: 7.2 }` output automatically — no manual work needed there.
- For `Display`, we can't just do `write!(f, "{} +{}i", self.real, self.imag)` blindly, because when `imag` is negative (like `-2.3`), that would print `4.7 +-2.3i` — a double sign, which looks wrong.
- So we manually check the sign with an `if`/`else`: if `imag` is positive or zero, print a literal `+`; if negative, print a literal `-` and use `.abs()` (absolute value) so we don't print the number's own negative sign on top of it.

Expected output:
```
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }

Display: 4.7 - 2.3i
Debug: Complex { real: 4.7, imag: -2.3 }
```

This matches the target format exactly, including the bonus requirement of a space after the `+`/`-` sign.
