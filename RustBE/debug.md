# What is `Debug` in Rust?

## The basic idea, in plain English

Imagine you have a box (a struct) that holds some data. If you try to print that box, Rust needs instructions on *how* to turn it into readable text. Without those instructions, Rust just refuses — it throws a compile error.

`Debug` is one set of instructions for printing — specifically, a *quick-and-dirty, developer-focused* way to print. Its whole purpose is to let you peek inside a value while you're coding/testing, not to make something pretty for end users.

## Why does printing need "instructions" at all?

Rust has a formatting system (`std::fmt`) built around **traits**. A trait is basically a contract — "if you implement this trait, you promise to provide certain behavior." Two traits are relevant to printing:

| Trait | Symbol used | Purpose | Do you write it yourself? |
|---|---|---|---|
| `Display` | `{}` | Clean, user-facing output | **Yes**, always manually |
| `Debug` | `{:?}` | Raw, developer-facing output | **No** — Rust can generate it for you |

Built-in types (`i32`, `String`, etc.) already come with both implemented. But any struct or enum *you* create starts out with **neither** — Rust has no idea how you want it shown.

## The `derive` shortcut

```rust
struct UnPrintable(i32);
```
This holds a single number, but Rust can't print it at all — no `{}`, no `{:?}`. Trying will give a compile error.

```rust
#[derive(Debug)]
struct DebugPrintable(i32);
```
The line `#[derive(Debug)]` sits directly above the struct and tells the compiler: *"Please auto-generate a basic Debug implementation for this."* You wrote zero printing logic — the compiler did it for you. Now this struct works with `{:?}`, though still not with `{}`.

Think of `derive` as Rust saying: *"I'll write the boring version for you, but if you want something fancy (Display), you're on your own."*

## Walking through the code example

```rust
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);
```
- `Structure` wraps one number.
- `Deep` wraps a `Structure` inside it — a struct nested inside another struct.
- Both get `#[derive(Debug)]`, so both can be printed with `{:?}`. Notably, nesting isn't a problem — if the inner type is Debug-printable, the outer one can be too.

```rust
println!("{:?} months in a year.", 12);
```
→ Prints: `12 months in a year.`
For simple types like numbers, `{:?}` and `{}` look almost identical.

```rust
println!("{1:?} {0:?} is the {actor:?} name.",
         "Slater",
         "Christian",
         actor="actor's");
```
This isn't really about Debug specifically — it's showing off argument positioning:
- `{1:?}` means "print argument #1" (`"Christian"`, since counting starts at 0)
- `{0:?}` means "print argument #0" (`"Slater"`)
- `{actor:?}` means "print the argument named `actor`" (`"actor's"`)

So the printed result is: `"Christian" "Slater" is the "actor's" name.`
(Strings show up with quotation marks in Debug mode — that's a small difference from Display mode.)

```rust
println!("Now {:?} will print!", Structure(3));
```
→ Prints: `Now Structure(3) will print!`
Debug's auto-generated format is simply: **StructName(value)**.

```rust
println!("Now {:?} will print!", Deep(Structure(7)));
```
→ Prints: `Now Deep(Structure(7)) will print!`
This is the key point the notes are making: the nested value gets printed too, but it's shown exactly as-is — `Deep(Structure(7))` — with no way to simplify it. If you just wanted the output to say `7`, derived Debug can't do that. **You have no control over the format** — you get whatever Rust decides to generate.

## Pretty printing: `{:#?}`

Regular `{:?}` crams everything onto one line. `{:#?}` ("pretty Debug") spreads it out nicely with indentation — same information, easier to read.

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
```
- `name` is a string reference (the `'a` is a *lifetime* — Rust's way of tracking how long a borrowed reference stays valid; not important for understanding printing).
- `age` is an unsigned 8-bit number.

```rust
let name = "Peter";
let age = 27;
let peter = Person { name, age };
println!("{:#?}", peter);
```
Output:
```
Person {
    name: "Peter",
    age: 27,
}
```
Same data as `{:?}` would give you, just formatted across multiple lines instead of squashed onto one — much easier to read, especially for structs with several fields.

## The final takeaway line

> "One can manually implement `fmt::Display` to control the display."

This means: if the auto-generated Debug output isn't good enough — if you want *exact* control over how something prints (e.g., just showing `7` instead of `Structure(7)`) — you have to write your own `impl fmt::Display for YourType { ... }` block by hand. `Debug` is the free, automatic, rough option. `Display` is the manual, precise, polished option.

## Summary cheat sheet

- **`Debug`** = auto-generatable, quick, rough, meant for developers → `{:?}`
- **`Display`** = manual-only, precise, meant for end users → `{}`
- **`{:#?}`** = Debug's "pretty" mode — same info, nicer formatting
- **`#[derive(Debug)]`** = the magic line that gives you Debug printing for free
- The tradeoff: automatic = easy but inflexible; manual = more work but full control
