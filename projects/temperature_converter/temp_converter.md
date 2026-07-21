# Temperature Converter in Rust

This program asks the user to choose a conversion type, then converts a temperature accordingly. Let's break it down piece by piece.

## 1. Setup
```rust
use std::io;
```
This imports Rust's **input/output** library so the program can read what the user types in the terminal.

## 2. Displaying the Menu
```rust
println!("Temperature Converter");
println!("1. Celsius to Fahrenheit");
println!("2. Fahrenheit to Celsius");
```
`println!` just prints text to the screen. Here it shows the user a simple menu with two options.

## 3. Reading the User's Choice
```rust
let mut choice = String::new();
io::stdin().read_line(&mut choice).unwrap();
let choice = choice.trim();
```
- `String::new()` creates an empty, **mutable** (changeable) text box to store what the user types.
- `read_line` reads what the user typed and stuffs it into `choice`.
- `.unwrap()` means "if something goes wrong reading input, just crash" (a quick-and-dirty way to handle errors).
- `.trim()` removes extra spaces or the invisible "enter/newline" character left over from typing.

**Note:** the second `let choice = choice.trim();` creates a *new* variable that shadows (replaces) the old one — a common Rust pattern to clean up a value without needing two different names.

## 4. Checking the Choice — Celsius to Fahrenheit
```rust
if choice == "1" {
    let mut input = String::new();
    println!("Enter Celsius:");
    io::stdin().read_line(&mut input).unwrap();
    let celsius: f64 = input.trim().parse().unwrap();
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{:.2}°F", fahrenheit);
}
```
- If the user typed `"1"`, it asks for a Celsius value.
- `input.trim().parse().unwrap()` converts the typed text (a `String`) into an actual number (`f64` = a decimal number).
- Formula used: **F = C × 9/5 + 32**
- `{:.2}` in the print statement means "show only 2 decimal places."

## 5. Checking the Choice — Fahrenheit to Celsius
```rust
} else if choice == "2" {
    ...
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°C", celsius);
}
```
Same idea, but using the reverse formula: **C = (F − 32) × 5/9**

## 6. Handling Invalid Input
```rust
} else {
    println!("Invalid choice.");
}
```
If the user types anything other than `"1"` or `"2"`, the program just tells them it's invalid — no crash, no conversion.

---

## Quick Summary Table

| Step | What Happens |
|------|--------------|
| Import `io` | Lets the program read keyboard input |
| Show menu | Prints 2 options to choose from |
| Read choice | Captures "1" or "2" as text, trims whitespace |
| Match choice | Uses `if/else if/else` to decide what to do |
| Convert | Parses input string → number → applies formula |
| Output | Prints result rounded to 2 decimal places |

## Key Rust Concepts Used
- **`mut`** → makes a variable changeable (needed since input is being written into it)
- **`unwrap()`** → a shortcut that assumes nothing goes wrong (not ideal for production code, but fine for practice)
- **`parse()`** → converts text into a number
- **Shadowing** → reusing a variable name for a cleaned-up version of the same data
- **`if / else if / else`** → simple decision-making structure

