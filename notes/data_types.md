# Data Types in Rust —

Every value in Rust has a type, which tells Rust what kind of data is being specified so that it knows how to work with that data. Rust is **statically typed** — meaning Rust must figure out the type of every variable *before* the program even runs (at compile time), not while it's running.

Usually you don't have to tell Rust the type yourself — the compiler can usually infer what type we want to use based on the value and how we use it. But sometimes Rust genuinely can't guess, and you have to help it out with a **type annotation**:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Here, `.parse()` could theoretically turn `"42"` into *many* different number types (a small integer, a big integer, a float...), so Rust can't pick one on its own. Without the `: u32` hint, you'd get an error like: "error[E0284]: type annotations needed... consider giving `guess` an explicit type." Adding `: u32` settles the ambiguity.

Rust splits its types into two big buckets: **scalar** (a single value) and **compound** (multiple values grouped together).

---

## Scalar Types (a single value)

A scalar type represents a single value. There are four of them: integers, floating-point numbers, booleans, and characters.

### 1. Integers — whole numbers, no decimals

An integer is a number without a fractional component. Rust gives you many integer types, differing in two ways:

- **Size** (how many bits of memory it uses: 8, 16, 32, 64, 128, or "architecture-dependent")
- **Sign** (whether it can be negative)

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture-based | `isize` | `usize` |

**Signed vs unsigned, simply:** Signed and unsigned refer to whether it's possible for the number to be negative — Rust compares it to writing on paper: when the sign matters, a number is shown with a plus or minus sign; when it's safe to assume it's positive, no sign is needed. So `i32` can be negative, `u32` can only be zero or positive.

**How big can these numbers get?** Each signed type can hold roughly half positive, half negative — e.g. an `i8` ranges from −128 to 127. An unsigned type uses that same space entirely for positive numbers — a `u8` ranges from 0 to 255.

**`isize`/`usize`** are special — their size depends on your computer's architecture (64-bit on 64-bit machines, 32-bit on 32-bit machines). The primary situation in which you'd use isize or usize is when indexing some sort of collection (e.g., getting item #3 from a list).

**Writing integers:** You can write them in different formats:
| Format | Example |
|---|---|
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte | `b'A'` |

The underscores (like `1_000`) are just visual separators — they mean nothing to the computer, they're purely for human readability, same as writing `1,000` on paper.

**Which one should you use?** If you're unsure, Rust's defaults are generally good places to start — integer types default to `i32`, since it's usually the fastest.

### Integer overflow — what happens if a number gets too big?

Say a `u8` variable can only hold 0–255. If you try to force it to hold `256`, that's **overflow**. Two different things can happen depending on how you compile:

- **Debug mode** (normal `cargo run`): Rust includes checks for integer overflow that cause your program to panic at runtime — it crashes on purpose, loudly, to warn you something's wrong.
- **Release mode** (`--release`, for shipped/optimized programs): Rust does *not* check for this. Instead it does "wrapping" — values greater than the maximum "wrap around" to the minimum. So for a `u8`, 256 quietly becomes 0, 257 becomes 1, and so on. The program won't panic, but the variable will have a value that probably isn't what you expected. This silent wraparound is considered a bug waiting to happen — relying on it is considered an error.

If you need to deliberately handle overflow, Rust gives you explicit tools instead of relying on the default behavior:
- `wrapping_*` — always wraps around
- `checked_*` — gives `None` if overflow happens (a safe "did this fail?" signal)
- `overflowing_*` — gives you the result *and* a `true`/`false` saying whether it overflowed
- `saturating_*` — caps out at the type's min/max instead of wrapping

### 2. Floating-point numbers — decimals

Rust has two: `f32` and `f64` (32-bit and 64-bit). All floating-point types are signed (can be negative).

```rust
let x = 2.0; // f64 (the default)
let y: f32 = 3.0;
```

`f64` is the default because on modern CPUs, it's roughly the same speed as f32 but is capable of more precision — so you get more accuracy for basically free.

### Basic math operations

Rust supports all the operations you'd expect:

```rust
let sum = 5 + 10;              // addition
let difference = 95.5 - 4.3;   // subtraction
let product = 4 * 30;          // multiplication
let quotient = 56.7 / 32.2;    // division
let truncated = -5 / 3;        // Results in -1
let remainder = 43 % 5;        // remainder (modulo)
```

One subtlety: integer division truncates toward zero to the nearest integer — so `-5 / 3` doesn't round to `-2`, it chops off the decimal and gives `-1`.

### 3. Booleans — true or false

```rust
let t = true;
let f: bool = false;
```

Simple — just `true` or `false`, one byte in size. Mostly used in conditionals (`if` statements).

### 4. Characters — single "letters" (sort of)

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

Note the **single quotes** — that's what makes it a `char` rather than a string (which uses double quotes). Rust's `char` is actually much bigger and more flexible than in many other languages: it's 4 bytes in size and represents a Unicode scalar value, meaning it can hold way more than plain English letters — accented letters, Chinese/Japanese/Korean characters, emoji, and more are all valid `char`s. As a caveat, your everyday intuition for "what counts as one character" doesn't always match Rust's technical definition — but that's a deeper topic for later.

---

## Compound Types (multiple values grouped together)

Compound types can group multiple values into one type. Rust has two: **tuples** and **arrays**.

### Tuples — group different types together, fixed size

A tuple is a general way of grouping together a number of values with a variety of types into one compound type — meaning the elements *don't* all have to be the same type. Once created, a tuple's size is locked — it cannot grow or shrink.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

**Getting values back out — two ways:**

**1. Destructuring** (unpacking all at once into separate variables):
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {y}"); // 6.4
```
This is called destructuring because it breaks the single tuple into three parts.

**2. Dot-index access** (grabbing one specific value):
```rust
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```
Just like arrays and most things in programming, counting starts at 0 (`.0` is the first item).

**Bonus fact:** An empty tuple `()` has a special name — **unit**. It represents "no value," and expressions implicitly return the unit value if they don't return any other value (this is why functions with no explicit return type still technically "return" something).

### Arrays — group same-type values together, fixed size

Unlike a tuple, every element of an array must have the same type, and arrays in Rust have a fixed length (can't grow or shrink).

```rust
let a = [1, 2, 3, 4, 5];
```

**When to use an array vs. something more flexible?** Arrays store data on the stack (fast, fixed-size memory) rather than the heap (flexible, resizable memory — more on this later). If you need a list that can *grow or shrink*, Rust has a separate type for that called a **vector** (`Vec`), which lives on the heap. In fact: if you're unsure whether to use an array or a vector, chances are you should use a vector.

Arrays are best when you know for certain the size will never change — the classic example: the 12 months of the year, since you know it will always contain 12 elements.

**Declaring an array's type explicitly:**
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
`i32` = the type of each element, `5` = how many elements total.

**Shortcut: same value repeated:**
```rust
let a = [3; 5];
```
This creates 5 elements, all initialized to `3` — equivalent to writing `[3, 3, 3, 3, 3]`, just shorter.

**Accessing elements** works with square-bracket indexing, starting at 0:
```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];  // 1
let second = a[1]; // 2
```

### What happens if you access an invalid index?

If a user (or your code) tries to access an index that doesn't exist — say, index `10` in a 5-element array — the program **panics** at runtime with an error like:
```
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
```

This can't be caught at compile time in cases like user input, because the compiler can't possibly know what value a user will enter when they run the code later. So Rust checks it live, while the program runs, and immediately crashes rather than continuing with bad data.

**Why does Rust bother doing this check at all?** This is one of Rust's core safety guarantees. In many low-level languages, this kind of check isn't performed, and providing a bad index can let you accidentally read random, invalid memory — a classic source of serious bugs and security vulnerabilities. Rust would rather stop your program cleanly than let it silently misbehave.

---

## Quick summary cheat sheet

| Category | Type | What it holds | Fixed size? | Same type required? |
|---|---|---|---|---|
| Scalar | Integer (`i32`, `u8`, etc.) | Whole numbers | — | — |
| Scalar | Float (`f32`, `f64`) | Decimal numbers | — | — |
| Scalar | `bool` | `true`/`false` | — | — |
| Scalar | `char` | One Unicode character | — | — |
| Compound | Tuple | Mixed-type group | ✅ Yes | ❌ No |
| Compound | Array | Same-type group | ✅ Yes | ✅ Yes |

**One-line mental model:**
> Scalars hold *one* value; compounds hold *many*. Tuples let those many values be *different* types but never resize; arrays force them to be the *same* type and also never resize. When you need resizing, you'll reach for a `Vec` later on.
