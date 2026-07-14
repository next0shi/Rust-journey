# Functions in Rust:

## The basics

You've already used a function without realizing it — `main`, the entry point of every Rust program. `fn` is the keyword that lets you declare a new function.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

**How to write a function:** type `fn`, then a name, then parentheses `()`, then curly braces `{}` containing the code that runs when the function is called.

**How to call a function:** just write its name followed by `()`. Since `another_function` is defined somewhere in the program, `main` is allowed to call it.

**Naming style:** Rust code uses snake case as the conventional style for function and variable names — all lowercase, words separated by underscores (like `another_function`, not `anotherFunction`).

**Order doesn't matter:** Rust doesn't care where you define your functions, only that they're defined somewhere in a scope that can be seen by the caller. So `another_function` can be written *after* `main` in the file, and it still works — unlike some languages where order matters.

Output of the example:
```
Hello, world!
Another function.
```
The lines execute in the order in which they appear in the main function — top to bottom, just like you'd expect.

---

## Parameters — giving functions inputs

A function can accept **parameters** — special variables that are part of a function's signature, letting you pass information into it.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

Here, `another_function` expects one input named `x`, of type `i32`. When we call `another_function(5)`, the `5` gets plugged in as `x`.

**A small vocabulary note:** technically, the `5` you *pass in* is called an **argument**, while `x` in the function definition is the **parameter**. In casual speech, people use these words interchangeably, but technically "parameter" = the placeholder name in the definition, "argument" = the actual value you hand over when calling.

**Important rule:** In function signatures, you must declare the type of each parameter — no exceptions, unlike `let` where Rust can often guess the type on its own. This is a deliberate design choice: requiring type annotations in function definitions means the compiler almost never needs you to specify types elsewhere just to figure out what you mean. It also lets Rust give you clearer error messages, since it always knows exactly what type each function expects.

**Multiple parameters** are separated by commas:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
Output: `The measurement is: 5h`

Two parameters here: `value` (an `i32`) and `unit_label` (a `char`). You just list them comma-separated inside the parentheses, each with its own type.

---

## Statements vs. Expressions — a key Rust concept

This distinction trips a lot of people up coming from other languages, so it's worth slowing down on.

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value.

Think of it this way: a statement *does* something; an expression *produces* something.

### Statements — examples

```rust
let y = 6;
```
This whole line is a statement. It creates a variable and assigns it a value, but the statement itself doesn't hand back a value you can reuse.

Even function definitions are statements — the entire `fn another_function() { ... }` block is itself a statement.

**Key consequence:** because statements don't return values, you can't do this:

```rust
let x = (let y = 6); // does NOT compile
```

Why? The `let y = 6` statement does not return a value, so there isn't anything for x to bind to. Rust actually differs from languages like C or Ruby here — in those languages, `x = y = 6` works fine and gives both variables the value 6, because assignment itself returns a value. **In Rust, assignment does not return a value**, so chaining assignments like that is simply not possible.

### Expressions — examples

Expressions evaluate to a value and make up most of the rest of the code that you'll write in Rust. Some examples straight from the notes:

- A math operation like `5 + 6` is an expression — it evaluates to `11`.
- The `6` inside `let y = 6;` is itself an expression (it evaluates to the value 6) — even though the *whole line* is a statement, part of it is an expression.
- Calling a function is an expression.
- Calling a macro is an expression.
- A new scope block wrapped in `{}` is *also* an expression:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Here, the whole `{ let x = 3; x + 1 }` block is treated as one big expression. It runs `let x = 3;` (a statement), then evaluates `x + 1` (an expression, which becomes `4`), and because that's the *last* line in the block *with no semicolon*, that `4` becomes the value of the entire block — which then gets assigned to `y`.

### The semicolon rule — this is crucial

> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

This is one of the most important (and most easily missed) rules in Rust. Adding or removing a semicolon isn't just stylistic — it can completely change whether a line produces a value or not. Keep this rule in your back pocket; it becomes critical in the next section.

---

## Functions with return values

Functions can return values to the code that calls them. Two important rules:

1. You don't name return values — you just declare their **type** after an arrow `->`.
2. **The return value of a function is simply the value of the final expression in the function body.** (You *can* use the `return` keyword to exit early with a value, but most functions just let the last expression "fall through" as the return value.)

### Simplest possible example

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself — and that's perfectly valid. Since `5` is the last expression in the function (with no semicolon), it becomes the function's return value. This makes `let x = five();` functionally identical to writing `let x = 5;` directly.

### A slightly more useful example

```rust
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

This prints `The value of x is: 6`. `x + 1` is the last line, it's an expression (no semicolon), so its value (`6`) becomes what `plus_one` returns.

### The classic mistake: an accidental semicolon

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;   // <-- notice the semicolon added here
}
```

This **fails to compile.** Why? Adding that semicolon turns `x + 1` from an *expression* into a *statement*. And as we covered above, statements don't return values — they implicitly return `()` (the "unit" type, meaning "nothing"). But the function signature promised `-> i32`. So now there's a contradiction: the function *says* it returns an `i32`, but its body actually returns "nothing."

Rust's error message spells this out directly:
```
error[E0308]: mismatched types
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value
```

The compiler even tells you exactly how to fix it — remove that one semicolon.

---

## Summary cheat sheet

| Concept | Meaning |
|---|---|
| `fn name() { }` | Defines a function |
| Parameters `(x: i32)` | Inputs; types are **always required** |
| Arguments | The actual values passed in when calling |
| Statement | Does something, returns nothing (e.g. `let y = 6;`) |
| Expression | Evaluates to a value (e.g. `5 + 6`, `{ ... }` blocks) |
| Semicolon | Turns an expression into a statement — **removes its return value** |
| `-> Type` | Declares what a function returns |
| Return value | The value of the function's **final expression** (no semicolon), or an early `return value;` |

## One-line mental model

> **A statement does a job and hands back nothing. An expression produces a value you can use.** In Rust, whether the *last line* of a function has a semicolon or not decides whether that function returns something meaningful or returns nothing (`()`) — so that trailing semicolon is far more important than it looks.
