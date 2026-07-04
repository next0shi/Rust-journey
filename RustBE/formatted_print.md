## The Big Picture

Rust doesn't have one single "print" function — it has a **family of macros**, and they all share the same rules for formatting text. Think of it like one recipe (how to format text) that you can send to different destinations.

### The macro family

| Macro | Destination | Adds newline? |
|---|---|---|
| `format!` | Returns a `String` (doesn't print) | No |
| `print!` | Screen (stdout) | No |
| `println!` | Screen (stdout) | Yes |
| `eprint!` | Error stream (stderr) | No |
| `eprintln!` | Error stream (stderr) | Yes |

You'll use `println!` 90% of the time. The others exist for special cases — e.g. `eprintln!` for error messages so they don't mix with your program's normal output, or `format!` when you want the text as data instead of printing it immediately.

---

## How the formatting works

Inside the string, `{}` is a **placeholder** — "put a value here."

```rust
println!("{} is {} years old", "Alice", 30);
// Alice is 30 years old
```

You can control *which* value fills each `{}`:

- **By position**: `{0}`, `{1}` — like array indexes, starting at 0.
- **By name**: `{subject}` — clearer when there are many values.
- **By capturing a variable directly** (modern Rust): if a variable named `pi` exists, `{pi}` just grabs it.

You can also control **how** a value looks, using a `:` inside the braces:

- `{:b}` `{:o}` `{:x}` → binary / octal / hex
- `{:.3}` → 3 digits after the decimal point
- `{:>5}` → right-align in a 5-character space
- `{:0>5}` → same, but pad with zeros instead of spaces

Rust checks all of this **at compile time** — if you reference an argument that doesn't exist, or forget one, your code won't even compile. That's a safety net most languages don't give you.

---

## Debugging: `{}` vs `{:?}`

This is the key concept. Rust has two separate "how do I turn this into text" traits:

- **`Display` (`{}`)** — a clean, human-friendly representation. *"Print this so a user reading the output understands it."*
- **`Debug` (`{:?}`)** — a raw, developer-friendly representation. *"Print this so a programmer debugging the code can see its internal structure."*

**Why two traits instead of one?**
Because not every type has an obvious "pretty" way to display itself. What should `Display` show for a `struct Point { x: 5, y: 3 }`? Rust can't guess that — you'd have to define it yourself.

But for debugging, you don't need it to be pretty — you just want to *see the data*. So Rust lets you auto-generate that version with one line:

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

println!("{:?}", Point { x: 5, y: 3 });
// Point { x: 5, y: 3 }
```

`#[derive(Debug)]` tells the compiler: "auto-generate a basic debug printer for this type." No manual work needed.

If you want `{}` (Display) to work too, for a *custom, pretty* format, you have to write it yourself by implementing the `fmt::Display` trait — telling Rust exactly what text to produce.

There's also `{:#?}` — "pretty-print" debug, which spreads the fields across multiple indented lines instead of one line. Handy for larger structs.

---

## Mental model to remember

- **Macros** (`println!`, etc.) = *where the text goes*.
- **Placeholders** (`{}`, `{0}`, `{name}`) = *which value fills each slot, and in what order*.
- **Format specifiers** (`:b`, `:.3`, `:>5`) = *how that value is styled*.
- **`Display` vs `Debug`** = *for whom the text is meant* — end users get `Display`, programmers debugging get `Debug`.
- Built-in types (numbers, strings) already know how to do both. Your own structs only know `Debug`, and only if you ask for it with `#[derive(Debug)]`.
