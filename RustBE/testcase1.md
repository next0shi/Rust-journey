# Implementing `Display` for a `Vec` — The `?` Operator Explained

## The problem

When you implement `Display`, you often need to call `write!` **multiple times** — once for each piece of output. For example, printing a list means writing an opening `[`, then each element, then a closing `]`. That's several separate `write!` calls.

The catch: **every single `write!` call returns a `fmt::Result`** (success or failure). If you're writing multiple times in a row, you technically need to check *each one* to make sure it worked before moving to the next. Doing that manually would be tedious:

```rust
match write!(f, "[") {
    Ok(_) => {},
    Err(e) => return Err(e),
}
match write!(f, "{}", v) {
    Ok(_) => {},
    Err(e) => return Err(e),
}
// ...and so on, for every write!
```

That's a lot of repetitive boilerplate just to say "if this failed, stop and report the failure; otherwise keep going."

## The solution: `?`

The `?` operator is shorthand for exactly that pattern. Put simply:

> **`some_result?` means: "if this succeeded, unwrap the value and keep going. If it failed, stop the whole function right now and return that error."**

So instead of the long `match` block above, you just write:

```rust
write!(f, "[")?;
```

This one line does the entire check-and-bail-out dance automatically. It keeps your code short and readable while still being safe — no error goes unnoticed.

Think of `?` as a **safety valve**: "try this, and if it explodes, let the explosion propagate up instead of pretending everything's fine."

## Walking through the `List` example

```rust
struct List(Vec<i32>);
```
A tuple struct wrapping a `Vec<i32>` (a growable list of whole numbers).

```rust
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
```
We're manually implementing `Display` (as covered before) — this is required since `Vec<T>` itself has no built-in `Display`.

```rust
        let vec = &self.0;
```
Grabs a reference to the inner `Vec<i32>` so we can loop over it. (`self.0` = the first/only field of the tuple struct.)

```rust
        write!(f, "[")?;
```
Writes the opening bracket. The `?` says: "if this write fails, stop and return the error immediately."

```rust
        for (index, v) in vec.iter().enumerate() {
```
- `vec.iter()` creates an iterator over each element in the vector.
- `.enumerate()` pairs each element with its **position number** (0, 1, 2, ...).
- So each loop iteration gives you `index` (the position) and `v` (the value at that position).

```rust
            if index != 0 { write!(f, ", ")?; }
```
For every element *except the first one* (`index != 0`), print a comma-and-space separator before it. This is the trick for avoiding a trailing/leading comma — you only add the separator *between* items, not before the very first one.

```rust
            write!(f, "{}", v)?;
```
Writes the actual value.

```rust
        }
        write!(f, "]")
    }
}
```
After the loop finishes, write the closing bracket. Notice this last line has **no `?` and no semicolon** — it's the final expression in the function, so its `fmt::Result` becomes the return value of `fmt`. (Adding `?` here would be pointless since there's nothing left to do afterward anyway.)

**Result:** `List(vec![1, 2, 3])` prints as `[1, 2, 3]`.

---

## Solving the Activity

**Goal:** print the index alongside each value, like `[0: 1, 1: 2, 2: 3]`.

Since we already have `index` from `.enumerate()`, we just need to include it in the `write!` call for each element:

```rust
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (index, v) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", index, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

**The only change:** `write!(f, "{}", v)?;` became `write!(f, "{}: {}", index, v)?;` — now each entry prints as `index: value` instead of just `value`.

**Output:**
```
[0: 1, 1: 2, 2: 3]
```

Exactly matches the target. Everything else (the bracket logic, the comma-separator logic, the `?` error handling) stays identical — we only changed *what* gets written for each element.
