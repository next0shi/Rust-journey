# Control Flow in Rust:


"Control flow" just means: **deciding which code runs, and how many times.** Two tools do this in Rust: **`if` expressions** (branching — pick a path) and **loops** (repetition — run code again and again).

---

# Part 1: `if` Expressions

## The basics

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

This reads almost like English: "if this condition is true, run this block; if this condition is not met, do not run this block." Since `number` (3) is less than 5, it prints `condition was true`.

The block of code inside each branch is sometimes called an **arm** — the same term used later for `match` expressions.

The `else` part is optional. If you skip it and the condition is false, the program will just skip the if block and move on to the next bit of code — nothing crashes, it just moves on.

## Rule: the condition MUST be a `bool`

This is a place where Rust is stricter than many other languages.

```rust
fn main() {
    let number = 3;
    if number {   // ERROR!
        println!("number was three");
    }
}
```

This fails to compile with: "error[E0308]: mismatched types... expected `bool`, found integer." 

Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. In many languages, a nonzero number is "truthy" and can be used directly in an `if`. **Rust refuses to do this guessing for you** — you must write an actual boolean expression, like:

```rust
if number != 0 {
    println!("number was something other than zero");
}
```

## `else if` — checking multiple conditions

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Output: `number is divisible by 3`

**Important behavior to understand:** Rust only executes the block for the first true condition, and once it finds one, it doesn't even check the rest. So even though 6 is *also* divisible by 2 (`number % 2 == 0` would also be true), that branch never even gets evaluated, because the `% 3 == 0` branch already matched first. Rust stops at the first "yes" and moves on — it doesn't check everything.

**A style tip from the notes:** if you find yourself stacking up many `else if`s, it's often a sign to refactor — Rust has a more powerful tool for this called `match`, covered later.

## Using `if` as an expression (this is very Rust-specific!)

Because `if` is an expression in Rust (recall: expressions produce values), you can use it directly on the right side of a `let`:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```

Output: `The value of number is: 5`

This works because blocks of code evaluate to the last expression in them — so `{ 5 }` evaluates to `5`, and `{ 6 }` evaluates to `6`. Whichever branch actually runs, its resulting value becomes the value of the whole `if` expression, which then gets stored in `number`.

**Critical rule:** both branches must produce **the same type**. This fails:

```rust
let number = if condition { 5 } else { "six" }; // ERROR!
```

Error: "`if` and `else` have incompatible types... expected integer, found `&str`"

**Why is Rust so strict about this?** Because variables must have a single type, and Rust needs to know definitively at compile time what type the number variable is. If `number` could be *either* an integer or a string depending on runtime conditions, the compiler couldn't guarantee type-safety everywhere else `number` gets used — so Rust simply won't allow it.

---

# Part 2: Loops — Repeating Code

Rust has three loop types: `loop`, `while`, and `for`.

## 1. `loop` — repeat forever (until told to stop)

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

This tells Rust to execute a block of code over and over again either forever or until you explicitly tell it to stop. It will print `again!` endlessly unless you interrupt it manually (Ctrl-C) or...

**...you use `break`:**
```rust
break;
```
`break` tells the program when to stop executing the loop — you saw this already in the guessing game to exit once the player won.

**And `continue`:**
`continue` in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration — also used in the guessing game to skip invalid (non-numeric) input.

## Returning a value from a `loop`

Here's something unique to Rust: since `loop` is itself an expression, you can make it **produce a value** by attaching that value to `break`:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

Walking through it: `counter` starts at 0 and increases by 1 each loop pass. Once it hits 10, `break counter * 2;` both **stops the loop** and **hands out the value 20** as the result of the whole loop. That value then gets assigned to `result` — printing "The result is 20".

**A related but different tool:** you can also `return` from inside a loop — but note the difference: `break` only exits the current loop, while `return` always exits the current function entirely (skipping anything after the loop too).

## Loop labels — controlling nested loops

If you have loops within loops, break and continue apply to the innermost loop at that point — by default. But sometimes you want to break the *outer* loop from inside the *inner* one. That's what labels are for.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

- `'counting_up:` is a **label** attached to the outer loop. Loop labels must begin with a single quote.
- The plain `break;` (no label) only breaks the **inner** loop — the one it's physically inside.
- `break 'counting_up;` specifically targets the **outer** labeled loop, breaking out of *both* loops at once.

This lets you say precisely "stop this specific loop" instead of always defaulting to "stop whichever loop I'm currently innermost in."

## 2. `while` — loop with a built-in condition check

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

`while` runs its body **as long as** its condition stays true, and stops automatically the moment it becomes false — no manual `if`/`break` combo needed. This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, making it cleaner for this common pattern.

## 3. `for` — the safest, most common loop for collections

### The problem with using `while` to loop over a collection

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
```

This technically works, but it's risky: this approach is error-prone; we could cause the program to panic if the index value or test condition is incorrect. If you shrink the array to 4 elements but forget to change `index < 5` to `index < 4`, you'd try to access a nonexistent 5th slot and crash. It's also slower, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every single loop pass.

### The `for` loop solution

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Same output, but far safer and simpler. `for` automatically walks through every item in the collection — no manual index tracking, no risk of going out of bounds, and no need to update anything if the array's size later changes. This is why the notes call `for` loops the most commonly used loop construct in Rust — they eliminate an entire category of bugs while also often being faster to run.

### Using `for` with a Range (even for simple counting)

Even when you just want to "count N times" (not loop over a real collection), most Rust programmers still reach for `for`, using a **Range**:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

- `1..4` generates a sequence of numbers starting from one number and ending before another number — so `1, 2, 3` (not including 4).
- `.rev()` reverses that sequence, so it counts down instead: `3, 2, 1`.

This gives the exact same countdown-then-liftoff behavior as the earlier `while` example, just more concisely and safely — no manual counter variable needed.

---

## Summary table

| Construct | Use case | Key trait |
|---|---|---|
| `if` / `else` / `else if` | Branch based on a condition | Condition must be `bool`; can be used as an expression |
| `loop` | Repeat until manually stopped | Can return a value via `break value;` |
| `while` | Repeat while a condition holds | Auto-stops when condition becomes false |
| `for` | Repeat over each item in a collection or range | Safest, most concise, avoids out-of-bounds bugs |
| Loop labels (`'label:`) | Control which nested loop `break`/`continue` affects | Must start with a single quote |

## One-line mental model

> **`if` picks a path; loops repeat a path.** Among loops: `loop` = "forever, until I say stop" (and can hand back a value); `while` = "keep going as long as this stays true"; `for` = "do this once per item" — and `for` is preferred whenever possible because it's both safer and usually faster.
