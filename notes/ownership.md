# Rust Ownership —

## The Core Problem It Solves

Every program needs to manage memory. There are three common approaches:
- **Garbage collection** (Java, Python, JS) — a background process cleans up unused memory automatically, but costs performance.
- **Manual management** (C, C++) — you allocate and free memory yourself. Fast, but easy to mess up (forget to free = memory leak, free twice = crash/security bug).
- **Ownership** (Rust) — the compiler enforces a set of rules *at compile time* so memory gets cleaned up automatically, with **zero runtime cost**. If you break the rules, your code simply won't compile.

## The Three Rules

1. Every value has exactly **one owner** (one variable responsible for it).
2. Only **one owner at a time** — ownership can move, but never be shared as "co-owner."
3. When the owner goes **out of scope**, Rust automatically cleans up (frees) the value.

Think of it like a library book: one person has it checked out at a time. If you hand it to a friend, you no longer have it — they do.

## Stack vs Heap (Why This Matters)

- **Stack** = fast, organized, last-in-first-out. Used for values with a known, fixed size (like `i32`, `bool`, `f64`).
- **Heap** = flexible but slower. Used for data that can grow or whose size isn't known ahead of time (like `String`, `Vec`).

A `String` actually stores 3 things on the stack — a **pointer** to the data, a **length**, and a **capacity** — while the actual text characters live on the heap.

This distinction is *why* ownership exists: heap data needs someone to be responsible for freeing it, or you get leaks (never freed) or crashes (freed twice).

## Move: Why Copying a String Isn't What You'd Expect

```rust
let s1 = String::from("hello");
let s2 = s1;
```

You might expect `s2` to be a copy of `s1`. Instead, Rust copies only the *stack part* (pointer, length, capacity) — not the actual heap data. Both `s1` and `s2` would technically point to the same heap memory.

To avoid a **double free** (both variables trying to free the same memory when they go out of scope), Rust simply invalidates `s1`. This is called a **move** — `s1`'s value moved into `s2`, and `s1` can no longer be used.

```rust
println!("{s1}"); // ❌ compile error: value borrowed after move
```

This is Rust catching a bug *before your program ever runs*.

## Clone: When You Actually Want a Copy

If you genuinely want two independent copies of heap data:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy — now both are valid
println!("s1 = {s1}, s2 = {s2}"); // ✅ works fine
```

`.clone()` is a visual flag that says "this might be an expensive operation" — unlike a move, which is always cheap.

## Copy: The Exception for Simple Types

```rust
let x = 5;
let y = x;
println!("x = {x}, y = {y}"); // ✅ both work! no move happened
```

Simple, fixed-size types (integers, floats, booleans, chars, and tuples made only of these) implement the **`Copy` trait**. They live entirely on the stack, copying them is trivial and cheap, so Rust just duplicates the value instead of moving it. No heap data = no ownership problem.

## Ownership and Functions

Passing a value into a function works just like assignment — it can **move** or **copy**:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);       // s moves into the function — s is now invalid here
    
    let x = 5;
    makes_copy(x);            // x is Copy, so it's still usable after this
    println!("{x}");          // ✅ fine
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope, memory is freed here

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // nothing special happens, it's just stack data
```

Functions can also **hand ownership back** by returning a value:

```rust
fn gives_ownership() -> String {
    String::from("yours") // ownership moves out to whoever calls this
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // ownership moves in, then right back out
}
```

## The Annoying Part (and What Comes Next)

If a function needs to *use* a value without taking ownership of it forever, you'd have to pass it in and then return it back out — every time:

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // give the string back along with its length
}
```

That's a lot of ceremony just to check a string's length. This is the exact problem **references** (`&`) solve — they let a function *borrow* a value without taking ownership, so you don't need to keep passing things back and forth. That's the natural next topic after ownership.

## Quick Mental Model

| Concept | Analogy |
|---|---|
| Ownership | One person "owns" the book |
| Move | You hand your book to a friend — you no longer have it |
| Clone | You photocopy the book — now there are two |
| Copy (for simple types) | It's a sticky note, not a book — copying it is free and instant |
| Drop (scope ends) | You return the book to the library automatically when you leave |

