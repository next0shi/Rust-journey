# Variables and Mutability in Rust

## The core idea

In most programming languages, once you create a variable, you can change its value whenever you want. Rust flips this default: **variables are locked (immutable) unless you explicitly say otherwise.** This is one of Rust's core safety features.

## Immutable by default

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

This code **won't compile**. Why? Because `let x = 5;` creates `x` as immutable — once it's `5`, it's *permanently* `5` (within that scope). Trying to do `x = 6;` afterward is like trying to relabel a sealed box — Rust says no.

The actual error you'd get: "error[E0384]: cannot assign twice to immutable variable `x`", with the compiler even suggesting the fix: "consider making this binding mutable... let mut x = 5;"

**Key reassurance from the notes:** Compiler errors can be frustrating, but really they only mean your program isn't safely doing what you want it to do yet; they do not mean that you're not a good programmer! Even experienced Rust programmers hit these regularly — it's Rust doing its job, not a sign you did something wrong.

## Why would a language want this?

It sounds annoying at first, but there's a real reason:

If one part of our code operates on the assumption that a value will never change and another part of our code changes that value, it's possible that the first part of the code won't do what it was designed to do. Bugs like this can be very hard to trace, especially if the change only happens *sometimes*.

By making immutability the default, the Rust compiler guarantees that when you state that a value won't change, it really won't change, so you don't have to keep track of it yourself. In other words: **you get to trust your own code more**, because Rust enforces the promise for you.

## Opting into mutability with `mut`

When you *do* want a variable's value to change, just add `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

This compiles fine and prints:
```
The value of x is: 5
The value of x is: 6
```

Adding `mut` does two things:
1. **Technically** — it tells Rust "allow this variable to be reassigned."
2. **Socially** — it signals to anyone reading your code, "heads up, this value is expected to change somewhere later." Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable's value.

There's no "right" universal choice — deciding whether to use mutability or not is up to you and depends on what you think is clearest in that particular situation.

## Constants — permanently locked, no exceptions

Constants look similar to immutable variables but are stricter in a few ways:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Key differences from regular `let` variables:

- **You can never make a constant mutable.** You aren't allowed to use mut with constants. Constants aren't just immutable by default—they're always immutable.
- **You use `const` instead of `let`**, and **you must always write out the type** (here, `u32`). With regular variables, Rust can often figure out the type for you; with constants, you must state it explicitly.
- **Constants can live anywhere**, even outside of any function, in the "global scope" — meaning they're accessible throughout your whole program. Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
- **A constant's value must be computable at compile time** — before the program even runs. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime. So you can write `60 * 60 * 3` (just math with fixed numbers) but not something like "the current time" or "user input," since those aren't knowable until the program is actually running.

Why write `60 * 60 * 3` instead of just `10800`? Because it's clearer to a reader — The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that's easier to understand and verify, rather than setting this constant to the value 10,800.

Constants are great for values used throughout your whole program that should never change — such as the maximum number of points any player of a game is allowed to earn, or the speed of light. They also make future maintenance easier: it also helps to have only one place in your code that you would need to change if the hardcoded value needed to be updated in the future.

By convention, constant names are written in **ALL_CAPS_WITH_UNDERSCORES**.

## Shadowing — reusing a variable name for a "new" variable

This is a completely different concept from `mut`, even though it can look similar at a glance.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

Output:
```
The value of x in the inner scope is: 12
The value of x is: 6
```

What's happening step by step:
1. `let x = 5;` → `x` is `5`.
2. `let x = x + 1;` → creates a **brand new** variable, also named `x`, whose value is the old `x` (5) plus 1 → `6`. The old `x` is essentially thrown away/hidden ("shadowed").
3. Inside the `{ }` block (a nested scope), `let x = x * 2;` creates *another* new `x`, based on the current `x` (6) times 2 → `12`. But this only exists **inside that block**.
4. Once the block ends, that innermost `x` disappears, and we're back to the `x` from step 2, which is `6`.

So each `let x = ...` doesn't modify the existing `x` — it creates a **new variable that happens to share the same name**, temporarily hiding the old one.

## Shadowing vs. `mut` — the real difference

These might look similar, but they behave very differently:

**1. Shadowing requires `let` each time.**
If you try to change a shadowed-style variable without `let`, you'll get a compile error, because you're not actually allowed to reassign an immutable variable — you're only allowed to create a new one with `let`. Shadowing is different from marking a variable as mut because we'll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.

**2. Shadowing lets you change the type; `mut` does not.**

This is the big practical difference. With shadowing:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Here, the first spaces variable is a string type, and the second spaces variable is a number type. Since each `let` creates a *completely new* variable, its type is free to be anything — even totally different from before. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.

But if you try the same trick with `mut` instead of shadowing:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

This **fails to compile**, because `mut` only allows you to change the *value* — not the *type* — of an existing variable. The error: "error[E0308]: mismatched types... expected `&str`, found `usize`"

Here, `spaces` was locked in as a string (`&str`) type the moment it was declared, and reassigning a number (`usize`) to it — even a mutable one — breaks that contract.

## Summary table

| Feature | `let` (immutable) | `let mut` | `const` | Shadowing (`let` again) |
|---|---|---|---|---|
| Can reassign value? | ❌ No | ✅ Yes | ❌ Never | N/A — creates new variable |
| Can change type? | N/A | ❌ No | ❌ No | ✅ Yes |
| Needs type annotation? | Optional | Optional | ✅ Required | Optional |
| Can be global? | No | No | ✅ Yes | No |
| Must be knowable at compile time? | No | No | ✅ Yes | No |

## The one-line mental model

> **`let` locks a value in place. `mut` unlocks it so the *same* variable can change. Shadowing doesn't unlock anything — it just quietly swaps in a brand-new variable with the same name, which is why it can even switch types. `const` is the strictest of all: locked forever, known before the program even runs.**
