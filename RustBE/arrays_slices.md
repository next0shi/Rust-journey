# Arrays and Slices in Rust 

## Arrays —

An array is a collection of objects of the same type `T`, stored in contiguous memory (meaning all the elements sit right next to each other in memory, back to back). Two key facts:

- Created using square brackets `[]`.
- Their **length is known at compile time** and is actually baked into the type itself: `[T; length]`. So a `[i32; 5]` isn't just "an array of i32s" — it's specifically "an array of exactly 5 i32s," and that "5" is part of the type, not just a runtime detail.

## Slices — the new concept here

A **slice** is like a "view into" an array, rather than the array itself. The big difference from an array:

Slices are similar to arrays, but their length is not known at compile time.

Instead of storing the data directly, a slice is a two-word object:
1. **A pointer** to where the data actually lives (usually inside some array).
2. **A length** — how many elements this slice covers.

A "word" here just means a chunk of memory the same size as `usize` — which is 64 bits on a 64-bit machine (like a typical modern x86-64 computer), or 32 bits on a 32-bit machine.

So think of a slice as a **little note that says**: "go look over there (pointer), and read this many items (length)" — it doesn't own or copy the data itself; it just *borrows* a look at part of an existing array. This is why its type is written `&[T]` — the `&` means "a reference/borrow," and `[T]` means "a stretch of `T` values."

**Key mental model:**
- **Array** = the actual box of items, fixed size, known at compile time.
- **Slice** = a window pointing at some (or all) of those items, size only known at runtime.

---

## Walking through the code

```rust
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
```

This function borrows a slice — notice the parameter type is `&[i32]`, not `[i32; 5]` or similar. This is powerful: because slices don't need a compile-time-known length, this *one* function can accept slices of **any length**, from any array, without needing a different function for each possible size.

### Creating arrays

```rust
let xs: [i32; 5] = [1, 2, 3, 4, 5];
```
A fixed-size array of 5 `i32`s. The type annotation `[i32; 5]` here is technically optional (Rust could infer it), but written for clarity.

```rust
let ys: [i32; 500] = [0; 500];
```
Using the repeat-shortcut from before: this creates 500 elements, all initialized to `0`.

### Basic array operations

```rust
println!("First element of the array: {}", xs[0]);
println!("Second element of the array: {}", xs[1]);
```
Standard indexing — starts at 0, just like before.

```rust
println!("Number of elements in array: {}", xs.len());
```
`.len()` returns how many elements are in the array.

```rust
println!("Array occupies {} bytes", mem::size_of_val(&xs));
```
`mem::size_of_val(&xs)` tells you the actual memory size (in bytes) that `xs` takes up. This is being shown here to reinforce that arrays are stack allocated — meaning they live in a fast, fixed region of memory (the "stack"), and their whole size is known up front, unlike heap-allocated things like `Vec`.

### Borrowing an array as a slice

```rust
analyze_slice(&xs);
```
Arrays can be automatically borrowed as slices. Even though `xs` is a `[i32; 5]` (an array), passing `&xs` into a function expecting `&[i32]` (a slice) just works — Rust automatically converts a reference-to-array into a slice covering the *whole* array. That's why the function's output shows the first element and "5 elements" — it's looking at the entire array through a slice.

### Borrowing just *part* of an array as a slice

```rust
analyze_slice(&ys[1 .. 4]);
```
This is the real power of slices. `&ys[1 .. 4]` grabs a slice covering only *part* of `ys`. The syntax is `[starting_index..ending_index]`, where:

- `starting_index` is the first position in the slice.
- `ending_index` is one more than the last position in the slice.

So `1..4` means: start at index 1, and stop *before* index 4 — meaning it grabs indices `1, 2, 3` (three elements total), not index 4. This "exclusive end" convention is important to remember — it's why the range looks like it's "one too short" at first glance, but it's actually correct.

### Empty slices

```rust
let empty_array: [u32; 0] = [];
assert_eq!(&empty_array, &[]);
assert_eq!(&empty_array, &[][..]); // Same but more verbose
```
- `[u32; 0]` is a perfectly valid type — an array that holds *zero* elements.
- `&[]` is how you write "an empty slice" literally.
- `&[][..]` is a more explicit (but equivalent) way to write the same empty slice — the `[..]` means "the whole thing, start to finish," which for an empty array is still just... nothing.
- `assert_eq!` just checks that two things are equal, and will crash the program (panic) if they aren't — useful for tests/sanity checks.

### Safe indexing with `.get()`

This is one of the most important patterns in the whole example.

```rust
for i in 0..xs.len() + 1 { // Oops, one element too far!
    match xs.get(i) {
        Some(xval) => println!("{}: {}", i, xval),
        None => println!("Slow down! {} is too far!", i),
    }
}
```

Here's what's happening:
- The loop deliberately goes **one index too far** (`xs.len() + 1` means if `xs` has 5 elements, this loop tries indices 0 through 5 — but valid indices are only 0 through 4).
- Instead of using plain `xs[i]` (which would **panic** and crash the program on an invalid index, as we saw earlier with arrays), this code uses `xs.get(i)`.
- Arrays can be safely accessed using `.get`, which returns an `Option` — meaning instead of crashing, it hands back either:
  - `Some(value)` — if the index was valid, wrapping the actual value found there.
  - `None` — if the index was out of bounds, with **no crash at all**.
- The `match` then handles both cases gracefully: print the value if it's `Some`, or print a friendly warning message if it's `None`.

This is a much safer alternative to indexing directly with `[]` — instead of risking a runtime panic, `.get()` lets you *check* first and decide what to do, whether that's printing a warning (as shown) or using `.expect()` if you'd rather the program exit with a custom message instead of continuing silently.

### The two commented-out panics

```rust
//println!("{}", xs[5]);
```
This is commented out because it would fail: out-of-bound indexing on array with constant value causes compile time error. Since Rust knows at compile time that `xs` has exactly 5 elements (indices 0–4) and that `5` is a fixed, known number, it can catch this mistake *before* the program even runs.

```rust
//println!("{}", xs[..][5]);
```
This is different: out-of-bound indexing on slice causes runtime error. When you access `xs[..]`, you're implicitly turning `xs` into a slice, and slices don't have a compile-time-known length — so Rust *can't* catch the mistake ahead of time. It can only catch it at runtime, causing a panic when the program actually runs (similar to the array indexing panic seen in the earlier "Invalid Array Element Access" section).

---

## Summary table

| Feature | Array | Slice |
|---|---|---|
| Type signature | `[T; length]` | `&[T]` |
| Length known at | Compile time | Runtime only |
| Owns its data? | Yes | No — just borrows/points to data |
| Stored as | The actual data | A pointer + a length (two words) |
| Can view part of an array? | No — it *is* the whole thing | Yes — that's its main purpose |
| Out-of-bounds access | Caught at compile time (if index is a fixed constant) | Only caught at runtime (panics) |
| Safe access option | `.get(i)` → returns `Option` | `.get(i)` → returns `Option` |

## One-line mental model

> **An array is the actual data, fixed size, known up front. A slice is a lightweight "window" (pointer + length) that lets you look at all or part of an array without copying it — and its size can vary at runtime, which is exactly why one function using `&[T]` can handle arrays of any length.**
